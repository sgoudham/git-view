use clap::{crate_authors, crate_version, Arg, Command};

pub fn cmd() -> clap::Command<'static> {
    Command::new("git-view")
        .version(crate_version!())
        .author(crate_authors!())
        .about(
            "A git sub-command to view your git repository in the browser.

This currently supports the following URLs:
    - github.com
    - bitbucket.org
",
        )
        .arg(
            Arg::new("remote")
                .long_help("The remote to view git repository on\n[default: default remote]")
                .short('r')
                .long("remote")
                .value_name("name")
                .takes_value(true)
                .display_order(1),
        )
        .arg(
            Arg::new("branch")
                .long_help("The branch to view git repository on\n[default: current branch]")
                .short('b')
                .long("branch")
                .value_name("name")
                .takes_value(true)
                .display_order(2),
        )
        .arg(
            Arg::new("commit")
                .long_help("The commit to view git repository on\n[default: current commit]")
                .short('c')
                .long("commit")
                .value_name("hash")
                .default_missing_value("current")
                .conflicts_with_all(&["remote", "branch"])
                .display_order(3),
        )
        .arg(
            Arg::new("suffix")
                .long_help("A suffix to append onto the git repository URL")
                .short('s')
                .long("suffix")
                .value_name("suffix")
                .takes_value(true)
                .display_order(4),
        )
        .arg(
            Arg::new("issue")
                .long_help("Attempt to parse issue number and open issue link")
                .short('i')
                .long("issue")
                .display_order(5),
        )
        .arg(
            Arg::new("print")
                .long_help("Don't open browser and print the URL")
                .short('p')
                .long("print")
                .display_order(6),
        )
}
