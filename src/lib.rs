mod error;
mod git;

use std::process::Command;

use error::{AppError, ErrorType};
use git::{Git, GitCommand, GitOutput};
use url::Url;

#[derive(Debug)]
pub struct GitView {
    remote: Option<String>,
    branch: Option<String>,
    commit: Option<String>,
    is_print: bool,
}

impl GitView {
    pub fn new(
        branch: Option<String>,
        remote: Option<String>,
        commit: Option<String>,
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
        self.populate_branch(&Git::LocalBranch)?;
        // Retrieve the remote
        self.remote = Some(self.populate_remote(
            &Git::DefaultRemote,
            &Git::TrackedRemote(self.branch.as_ref().unwrap()),
        )?);

        // TODO: Figure out how to default to 'master' or 'main' if branch doesn't exist on remote
        //
        // Current theory on how to get it working:
        // 1. Run "git checkout" to check if there's a remote branch for your current local one (there should be no output if remote branch doesn't exist)
        // 2. Then run "git rev-parse --abbrev-ref <remote>/HEAD" and split on the first '/' to get the current default branch
        //      - Although, I think that this command isn't foolproof, it might be the best option though without trying to use some command line parsers

        // Retrieve the remote reference
        let remote_ref =
            self.get_remote_reference(&Git::UpstreamBranch(self.branch.as_ref().unwrap()))?;
        // Retrieve the full git_url
        // e.g https://github.com/sgoudham/git-view.git
        let git_url = self.get_git_url(&Git::IsValidRemote(self.remote.as_ref().unwrap()))?;
        // Extract protocol, domain and urlpath
        let (protocol, domain, urlpath) = self.parse_git_url(&git_url)?;
        // Generate final url to open in the web browser
        // let final_url = self.generate_final_url(protocol, domain, urlpath);

        // Open the URL
        webbrowser::open(
            format!("{}://{}/{}/tree/{}", protocol, domain, urlpath, remote_ref).as_str(),
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

    fn populate_branch(&mut self, command: &impl GitCommand) -> Result<(), AppError> {
        if self.branch.is_none() {
            match command.execute()? {
                GitOutput::Ok(output) => {
                    self.branch = Some(output);
                    Ok(())
                }
                GitOutput::Err(err) => Err(AppError::new(ErrorType::CommandFailed, err)),
            }
        } else {
            Ok(())
        }
    }

    /// Populates the remote variable within [`GitView`]
    /// User Given Remote -> Default Remote in Config -> Tracked Remote -> 'origin'
    fn populate_remote(
        &self,
        default_remote: &impl GitCommand,
        tracked_remote: &impl GitCommand,
    ) -> Result<String, AppError> {
        // Priority goes to user given remote
        if self.remote.is_none() {
            // Priority then goes to the default remote
            match default_remote.execute()? {
                GitOutput::Ok(def) => Ok(def),
                // Priority then goes to the tracked remote
                GitOutput::Err(_) => match tracked_remote.execute()? {
                    GitOutput::Ok(tracked) => Ok(tracked),
                    // Default to the 'origin' remote
                    GitOutput::Err(_) => Ok("origin".to_string()),
                },
            }
        } else {
            Ok(self.remote.as_ref().unwrap().to_string())
        }
    }

    fn get_remote_reference(&self, command: &impl GitCommand) -> Result<String, AppError> {
        match command.execute()? {
            GitOutput::Ok(output) => Ok(output.trim_start_matches("refs/heads/").to_string()),
            GitOutput::Err(_) => Ok(self.branch.as_ref().unwrap().to_string()),
        }
    }

    fn get_git_url(&self, command: &impl GitCommand) -> Result<String, AppError> {
        match command.execute()? {
            GitOutput::Ok(output) => {
                if &output != self.remote.as_ref().unwrap() {
                    Ok(output)
                } else {
                    Err(AppError::new(
                        ErrorType::MissingGitRemote,
                        format!(
                            "Looks like your git remote isn't set for '{}'",
                            self.remote.as_ref().unwrap()
                        ),
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
    fn parse_git_url(&self, git_url: &str) -> Result<(String, String, String), AppError> {
        // rust-url cannot parse 'scp-like' urls -> https://github.com/servo/rust-url/issues/220
        // Manually parse the url ourselves

        if git_url.contains("://") {
            match Url::parse(git_url) {
                Ok(url) => Ok((
                    url.scheme().to_string(),
                    url.host_str()
                        .map_or_else(|| "github.com", |host| host)
                        .to_string(),
                    url.path()
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .trim_end_matches(".git")
                        .to_string(),
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

                    Ok((
                        protocol.to_string(),
                        split_domain.to_string(),
                        path.to_string(),
                    ))
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
mod is_valid_repository {
    use std::process::{ExitStatus, Output};

    use crate::{git::MockGitCommand, GitView};

    fn instantiate_handler() -> GitView {
        GitView::new(
            Some(String::from("main")),
            Some(String::from("origin")),
            Some(String::from("latest")),
            false,
        )
    }

    // #[test]
    fn yes() {
        let handler = instantiate_handler();
        let mut mock = MockGitCommand::new();
        let is_valid_repository = handler.is_valid_repository(&mock);

        assert!(is_valid_repository.is_ok());
    }

    // #[test]
    fn no() {
        let handler = instantiate_handler();
        let mut mock = MockGitCommand::new();
        mock.expect_execute().never();

        let is_valid_repository = handler.is_valid_repository(&mock);

        assert!(is_valid_repository.is_err());
    }
}

#[cfg(test)]
mod parse_git_url {
    use crate::{error::AppError, GitView};
    use test_case::test_case;

    fn instantiate_handler() -> GitView {
        GitView::new(
            Some(String::from("main")),
            Some(String::from("origin")),
            Some(String::from("latest")),
            false,
        )
    }

    // http[s]://host.xz[:port]/path/to/repo.git/
    #[test_case("https://github.com:8080/sgoudham/git-view.git" ; "with port")]
    #[test_case("https://github.com/sgoudham/git-view.git" ; "normal")]
    #[test_case("https://github.com/sgoudham/git-view.git/" ; "with trailing slash")]
    fn https(git_url: &str) -> Result<(), AppError> {
        let handler = instantiate_handler();

        let (protocol, domain, urlpath) = handler.parse_git_url(git_url)?;

        assert_eq!(protocol, "https");
        assert_eq!(domain, "github.com");
        assert_eq!(urlpath, "sgoudham/git-view");

        Ok(())
    }

    // [user@]host.xz:path/to/repo.git/
    #[test_case("git@github.com:sgoudham/git-view.git" ; "with username")]
    #[test_case("github.com:sgoudham/git-view.git" ; "normal")]
    #[test_case("github.com:sgoudham/git-view.git/" ; "with trailing slash")]
    fn ssh(git_url: &str) -> Result<(), AppError> {
        let handler = instantiate_handler();

        let (protocol, domain, urlpath) = handler.parse_git_url(git_url)?;

        assert_eq!(protocol, "https");
        assert_eq!(domain, "github.com");
        assert_eq!(urlpath, "sgoudham/git-view");

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
