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
    is_issue: bool,
    is_print: bool,
}

impl<'a> GitView<'a> {
    pub fn new(
        branch: Option<&'a str>,
        remote: Option<&'a str>,
        commit: Option<&'a str>,
        is_issue: bool,
        is_print: bool,
    ) -> Self {
        Self {
            remote,
            branch,
            commit,
            is_issue,
            is_print,
        }
    }

    pub fn view_repository(&mut self) -> Result<(), AppError> {
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
        let final_url = self.generate_final_url(&remote_ref, &url);

        // Display OR Open the URL
        if self.is_print {
            println!("{}", final_url);
        } else {
            webbrowser::open(final_url.as_str())?;
        }

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
                // Don't error on this, instead make a new enum Branch/NotBranch and match on it
                // later
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
            // So as explained above, branch might not exist either
            // Instead of returning branch this early, match on the aforementioned enum above
            //
            // Branch:
            // Just do what you're doing already and `Cow::Borrowed(branch)`
            //
            // NotBranch:
            // Check to see if the user is on a tag or a specific commit hash. If all
            // else fails THEN error out
            GitOutput::Err(_) => Ok(Cow::Borrowed(branch)),
        }
    }

    fn get_git_url(&self, remote: &str, command: &impl GitCommand) -> Result<String, AppError> {
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

    fn generate_final_url(&self, remote_ref: &str, url: &Url) -> String {
        let branch_ref = match &url.domain {
            Domain::GitHub => {
                if self.is_issue {
                    format!("/issues/{}", capture_digits(remote_ref))
                } else {
                    format!("/tree/{}", escape_ascii_chars(remote_ref))
                }
            }
            Domain::BitBucket => todo!(),
        };

        let mut open_url = format!("{}://{}/{}", url.protocol, url.domain, url.path);

        // if self.commit.unwrap() == "latest" {
        //     ()
        // } else {
        //     ()
        // }

        if remote_ref == "master" || remote_ref == "main" {
            open_url
        } else {
            open_url.push_str(&branch_ref);
            open_url
        }
    }
}

fn capture_digits(remote_ref: &str) -> String {
    todo!()
}

fn escape_ascii_chars(remote_ref: &str) -> Cow<'_, str> {
    // I could use this below but I wanted to be more comfortable with Cow
    // branch.replace('%', "%25").replace('#', "%23");

    if remote_ref.contains(['%', '#']) {
        let mut escaped_str = String::with_capacity(remote_ref.len());

        for char in remote_ref.chars() {
            match char {
                '%' => escaped_str.push_str("%25"),
                '#' => escaped_str.push_str("%23"),
                _ => escaped_str.push(char),
            };
        }

        Cow::Owned(escaped_str)
    } else {
        Cow::Borrowed(remote_ref)
    }
}

#[cfg(test)]
mod lib_tests {
    use crate::GitView;

    fn instantiate_handler() -> GitView<'static> {
        GitView::new(Some("main"), Some("origin"), None, false, false)
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
