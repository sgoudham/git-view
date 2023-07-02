use std::panic::set_hook;

use clap::{command, crate_authors, crate_description, crate_version, Arg, Command, ErrorKind};
use git_view::Git;
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
        .arg(
            Arg::new("remote")
                .long_help("The remote to view on GitHub\n[default: default remote]")
                .short('r')
                .long("remote")
                .value_name("name")
                .takes_value(true)
                .display_order(1),
        )
        .arg(
            Arg::new("branch")
                .long_help("The branch to view on GitHub\n[default: current branch]")
                .short('b')
                .long("branch")
                .value_name("name")
                .takes_value(true)
                .display_order(2),
        )
        .arg(
            Arg::new("issue")
                .long_help("The GitHub issue number\n[default: number from current branch]")
                .short('i')
                .long("issue")
                .value_name("number")
                .default_missing_value("branch")
                .conflicts_with("commit")
                .takes_value(true)
                .display_order(3),
        )
        .arg(
            Arg::new("commit")
                .long_help("The commit to view on GitHub\n[default: current commit]")
                .short('c')
                .long("commit")
                .value_name("hash")
                .default_missing_value("current")
                .conflicts_with_all(&["remote", "branch"])
                .display_order(4),
        )
        .arg(
            Arg::new("path")
                .long_help(
                    "The directory/file to view on GitHub\n[default: current working directory]",
                )
                .short('p')
                .long("path")
                .default_missing_value("current-working-directory")
                .conflicts_with("issue")
                .takes_value(true)
                .value_hint(clap::ValueHint::AnyPath)
                .display_order(5),
        )
        .arg(
            Arg::new("print")
                .long_help("Print URL instead of opening on GitHub")
                .long("print")
                .display_order(6),
        );

    let matches = matches.get_matches();
    let git_view = GitView::new(
        matches.value_of("branch"),
        matches.value_of("remote"),
        matches.value_of("commit"),
        matches.value_of("issue"),
        matches.value_of("path"),
        matches.is_present("print"),
    );

    if let Err(app_error) = git_view.view_repository(Git) {
        clap_panic!(app_error.error_str);
    }
}
