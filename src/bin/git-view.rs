use std::panic::set_hook;

use clap::{command, crate_authors, crate_description, crate_version, Arg, Command, ErrorKind};
use git_view::GitView;

macro_rules! clap_panic {
    ($e:expr) => {
        command!().error(ErrorKind::DisplayHelp, $e).exit()
    };
}

fn main() {
    set_hook(Box::new(|info| clap_panic!(info)));

    let matches = Command::new("git-view")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .long_about(None)
        .arg(
            Arg::new("branch")
                .help("The branch to view git repository on")
                .short('b')
                .long("branch")
                .value_name("name")
                .takes_value(true)
                .display_order(2),
        )
        .arg(
            Arg::new("remote")
                .help("The remote to view git repository on")
                .short('r')
                .long("remote")
                .value_name("name")
                .takes_value(true)
                .display_order(1),
        )
        .arg(
            Arg::new("commit")
                .help("The commit to view git repository on")
                .short('c')
                .long("commit")
                .value_name("hash")
                .default_missing_value("latest")
                .conflicts_with_all(&["remote", "branch"])
                .display_order(3),
        )
        .arg(
            Arg::new("print")
                .help("Print the URL (doesn't open browser)")
                .short('p')
                .long("print")
                .display_order(4),
        );

    let matches = matches.get_matches();
    let mut git_view = GitView::new(
        matches.value_of("branch").map(str::to_string),
        matches.value_of("remote").map(str::to_string),
        matches.value_of("commit").map(str::to_string),
        matches.is_present("print"),
    );

    if let Err(err) = git_view.open_upstream_repository() {
        clap_panic!(err);
    }
}
