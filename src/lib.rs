mod error;
mod git;

use std::borrow::Cow;

use error::{AppError, ErrorType};
use git::{Domain, Git, GitCommand, GitOutput, Url};

#[derive(Debug)]
pub struct GitView<'a> {
    remote: Option<&'a str>,
    branch: Option<&'a str>,
    commit: Option<&'a str>,
    is_print: bool,
}

impl<'a> GitView<'a> {
    pub fn new(
        branch: Option<&'a str>,
        remote: Option<&'a str>,
        commit: Option<&'a str>,
        is_print: bool,
    ) -> Self {
        Self {
            remote,
            branch,
            commit,
            is_print,
        }
    }

    pub fn open_upstream_repository(&mut self) -> Result<(), AppError> {
        // Exit out if we're not inside a git repository
        self.is_valid_repository(&Git::IsValidRepository)?;
        // Retrieve the current branch
        let branch = self.populate_branch(&Git::LocalBranch)?;
        // Retrieve the remote
        let remote = self.populate_remote(&Git::DefaultRemote, &Git::TrackedRemote(&branch))?;

        // TODO: Figure out how to default to 'master' or 'main' if branch doesn't exist on remote
        //
        // Current theory on how to get it working:
        // 1. Run "git checkout" to check if there's a remote branch for your current local one (there should be no output if remote branch doesn't exist)
        // 2. Then run "git rev-parse --abbrev-ref <remote>/HEAD" and split on the first '/' to get the current default branch
        //      - Although, I think that this command isn't foolproof, it might be the best option though without trying to use some command line parsers

        // Retrieve the remote reference
        let remote_ref = self.get_remote_reference(&branch, &Git::UpstreamBranch(&branch))?;
        // Retrieve the full git_url
        // e.g https://github.com/sgoudham/git-view.git
        let git_url = self.get_git_url(&remote, &Git::IsValidRemote(&remote))?;
        // Extract protocol, domain and urlpath
        let url = self.parse_git_url(&git_url)?;
        // Generate final url to open in the web browser
        // let final_url = self.generate_final_url(protocol, domain, urlpath);

        // Open the URL
        webbrowser::open(
            format!(
                "{}://{}/{}/tree/{}",
                url.protocol, url.domain, url.path, remote_ref
            )
            .as_str(),
        )?;

        Ok(())
    }

    fn is_valid_repository(&self, command: &impl GitCommand) -> Result<(), AppError> {
        match command.execute()? {
            GitOutput::Ok(_) => Ok(()),
            GitOutput::Err(_) => Err(AppError::new(
                ErrorType::MissingGitRepository,
                "Looks like you're not in a valid git repository!".to_string(),
            )),
        }
    }

    fn populate_branch(&self, command: &impl GitCommand) -> Result<Cow<'_, str>, AppError> {
        if self.branch.is_none() {
            match command.execute()? {
                GitOutput::Ok(output) => Ok(Cow::Owned(output)),
                GitOutput::Err(err) => Err(AppError::new(ErrorType::CommandFailed, err)),
            }
        } else {
            Ok(Cow::Borrowed(self.branch.as_ref().unwrap()))
        }
    }

    /// Populates the remote variable within [`GitView`]
    /// User Given Remote -> Default Remote in Config -> Tracked Remote -> 'origin'
    fn populate_remote(
        &self,
        default_remote: &impl GitCommand,
        tracked_remote: &impl GitCommand,
    ) -> Result<Cow<'_, str>, AppError> {
        // Priority goes to user given remote
        if self.remote.is_none() {
            // Priority then goes to the default remote
            match default_remote.execute()? {
                GitOutput::Ok(def) => Ok(Cow::Owned(def)),
                // Priority then goes to the tracked remote
                GitOutput::Err(_) => match tracked_remote.execute()? {
                    GitOutput::Ok(tracked) => Ok(Cow::Owned(tracked)),
                    // Default to the 'origin' remote
                    GitOutput::Err(_) => Ok(Cow::Owned("origin".into())),
                },
            }
        } else {
            Ok(Cow::Borrowed(self.remote.as_ref().unwrap()))
        }
    }

    fn get_remote_reference(
        &self,
        branch: &'a str,
        command: &impl GitCommand,
    ) -> Result<Cow<'a, str>, AppError> {
        match command.execute()? {
            GitOutput::Ok(output) => Ok(Cow::Owned(
                output.trim_start_matches("refs/heads/").to_string(),
            )),
            GitOutput::Err(_) => Ok(Cow::Borrowed(branch)),
        }
    }

    fn get_git_url(&self, remote: &'a str, command: &impl GitCommand) -> Result<String, AppError> {
        match command.execute()? {
            GitOutput::Ok(output) => {
                if output != remote {
                    Ok(output)
                } else {
                    Err(AppError::new(
                        ErrorType::MissingGitRemote,
                        format!("Looks like your git remote isn't set for '{}'", remote),
                    ))
                }
            }
            GitOutput::Err(err) => Err(AppError::new(ErrorType::CommandFailed, err)),
        }
    }

    /*
     * Potential formats:
     *  - ssh://[user@]host.xz[:port]/path/to/repo.git/
     *  - git://host.xz[:port]/path/to/repo.git/
     *  - http[s]://host.xz[:port]/path/to/repo.git/
     *  - ftp[s]://host.xz[:port]/path/to/repo.git/
     *  - [user@]host.xz:path/to/repo.git/
     */
    fn parse_git_url(&self, git_url: &str) -> Result<Url, AppError> {
        // rust-url cannot parse 'scp-like' urls -> https://github.com/servo/rust-url/issues/220
        // Manually parse the url ourselves

        if git_url.contains("://") {
            match url::Url::parse(git_url) {
                Ok(url) => Ok(Url::new(
                    url.scheme(),
                    Domain::from_str(url.host_str().map_or_else(|| "github.com", |host| host)),
                    url.path()
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .trim_end_matches(".git"),
                )),
                Err(_) => Err(AppError::new(
                    ErrorType::InvalidGitUrl,
                    format!("Sorry, couldn't parse git url '{}'", git_url),
                )),
            }
        } else {
            match git_url.split_once(':') {
                Some((domain, path)) => {
                    let protocol = "https";
                    let path = path.trim_end_matches('/').trim_end_matches(".git");
                    let split_domain = match domain.split_once('@') {
                        Some((_username, dom)) => dom,
                        None => domain,
                    };

                    Ok(Url::new(protocol, Domain::from_str(split_domain), path))
                }
                None => Err(AppError::new(
                    ErrorType::InvalidGitUrl,
                    format!("Sorry, couldn't parse git url '{}'", git_url),
                )),
            }
        }
    }

    fn generate_final_url(&self, protocol: String, domain: String, urlpath: String) -> String {
        todo!();
    }
}

#[cfg(test)]
mod lib_tests {
    use crate::GitView;

    fn instantiate_handler() -> GitView<'static> {
        GitView::new(Some("main"), Some("origin"), Some("latest"), false)
    }

    mod is_valid_repository {

        use crate::{
            error::ErrorType,
            git::{GitOutput, MockGitCommand},
            lib_tests::instantiate_handler,
        };

        #[test]
        fn yes() {
            let handler = instantiate_handler();

            let mut mock = MockGitCommand::new();
            mock.expect_execute()
                .returning(|| Ok(GitOutput::Ok("Valid".to_owned())));

            let is_valid_repository = handler.is_valid_repository(&mock);

            assert!(is_valid_repository.is_ok());
        }

        #[test]
        fn no() {
            let handler = instantiate_handler();
            let mut mock = MockGitCommand::new();
            mock.expect_execute()
                .returning(|| Ok(GitOutput::Err("Error".to_owned())));

            let is_valid_repository = handler.is_valid_repository(&mock);

            assert!(is_valid_repository.is_err());

            let error = is_valid_repository.as_ref().unwrap_err();
            assert_eq!(error.error_type, ErrorType::MissingGitRepository);
            assert_eq!(
                error.error_str,
                "Looks like you're not in a valid git repository!"
            );
        }
    }

    mod parse_git_url {
        use crate::{error::AppError, lib_tests::instantiate_handler};
        use test_case::test_case;

        // http[s]://host.xz[:port]/path/to/repo.git/
        #[test_case("https://github.com:8080/sgoudham/git-view.git" ; "with port")]
        #[test_case("https://github.com/sgoudham/git-view.git" ; "normal")]
        #[test_case("https://github.com/sgoudham/git-view.git/" ; "with trailing slash")]
        fn https(git_url: &str) -> Result<(), AppError> {
            let handler = instantiate_handler();

            let url = handler.parse_git_url(git_url)?;

            assert_eq!(url.protocol, "https");
            assert_eq!(url.domain.to_string(), "github.com");
            assert_eq!(url.path, "sgoudham/git-view");

            Ok(())
        }

        // [user@]host.xz:path/to/repo.git/
        #[test_case("git@github.com:sgoudham/git-view.git" ; "with username")]
        #[test_case("github.com:sgoudham/git-view.git" ; "normal")]
        #[test_case("github.com:sgoudham/git-view.git/" ; "with trailing slash")]
        fn ssh(git_url: &str) -> Result<(), AppError> {
            let handler = instantiate_handler();

            let url = handler.parse_git_url(git_url)?;

            assert_eq!(url.protocol, "https");
            assert_eq!(url.domain.to_string(), "github.com");
            assert_eq!(url.path, "sgoudham/git-view");

            Ok(())
        }

        #[test]
        fn invalid_git_url() {
            let handler = instantiate_handler();
            let git_url_normal = "This isn't a git url";

            let error = handler.parse_git_url(git_url_normal);

            assert!(error.is_err());
            assert_eq!(
                error.unwrap_err().error_str,
                "Sorry, couldn't parse git url 'This isn't a git url'"
            );
        }
    }
}
