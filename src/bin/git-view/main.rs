use std::panic::set_hook;

use clap::{command, ErrorKind};
use git_view::Git;
use git_view::GitView;

mod cmd;

macro_rules! clap_panic {
    ($e:expr) => {
        command!().error(ErrorKind::DisplayHelp, $e).exit()
    };
}

fn main() {
    set_hook(Box::new(|info| clap_panic!(info)));

    let matches = cmd::cmd().get_matches();
    let mut git_view = GitView::new(
        matches.value_of("branch"),
        matches.value_of("remote"),
        matches.value_of("commit"),
        matches.value_of("suffix"),
        matches.is_present("issue"),
        matches.is_present("print"),
    );

    if let Err(app_error) = git_view.view_repository(Git) {
        clap_panic!(app_error.error_str);
    }
}
