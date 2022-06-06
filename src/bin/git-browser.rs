use std::panic::set_hook;

use clap::{command, crate_authors, crate_description, crate_version, Arg, Command, ErrorKind};
use git_browser::GitUpstream;

macro_rules! clap_panic {
    ($e:expr) => {
        command!().error(ErrorKind::DisplayHelp, $e).exit()
    };
}

fn main() {
    set_hook(Box::new(|info| clap_panic!(info)));

    let matches = Command::new("git-upstream")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .long_about(None)
        .arg(Arg::new("branch").help("The branch to open Github repo on"))
        .arg(
            Arg::new("remote")
                .help("The remote to open Github repo on")
                .short('r')
                .long("remote")
                .default_value("origin"),
        )
        .arg(
            Arg::new("commit")
                .help("Open Github on the current commit")
                .short('c')
                .long("commit")
                .conflicts_with_all(&["remote", "branch"]),
        )
        .arg(
            Arg::new("print")
                .help("Only display the URL, does not open Github")
                .short('p')
                .long("print"),
        );

    let matches = matches.get_matches();
    let mut git_upstream = GitUpstream::new(
        matches.value_of("branch").map(str::to_string),
        matches.value_of("remote").unwrap().to_string(),
        matches.is_present("commit"),
        matches.is_present("print"),
    );

    if let Err(err) = git_upstream.open_upstream_repository() {
        clap_panic!(err);
    }
}
