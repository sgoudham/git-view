mod error;
mod git;

use std::borrow::Cow;

use error::{AppError, ErrorType};
use git::{Domain, GitOutput, GitTrait, Url};

pub use git::Git;

enum Local<'a> {
    Branch(Cow<'a, str>),
    NotBranch,
}

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

    pub fn view_repository(&mut self, git: impl GitTrait) -> Result<(), AppError> {
        // Exit out if we're not inside a git repository
        self.is_valid_repository(&git)?;
        // Retrieve the current local ref (branch or not branch)
        let local = self.get_local_ref(&git)?;
        // Retrieve the remote
        let remote = self.populate_remote(&local, &git)?;
        // Retrieve the remote reference
        let remote_ref = self.get_remote_reference(&local, &git)?;

        // Retrieve the full git_url
        // e.g https://github.com/sgoudham/git-view.git
        let git_url = self.get_git_url(&remote, &git)?;
        // Extract protocol, domain and urlpath
        let url = self.parse_git_url(&git_url)?;
        // Generate final url to open in the web browser
        let final_url = self.generate_final_url(&remote_ref, &url, &git)?;

        // Display OR Open the URL
        if self.is_print {
            println!("{}", final_url);
        } else {
            webbrowser::open(final_url.as_str())?;
        }

        Ok(())
    }

    fn is_valid_repository(&self, git: &impl GitTrait) -> Result<(), AppError> {
        match git.is_valid_repository()? {
            GitOutput::Ok(_) => Ok(()),
            GitOutput::Err(_) => Err(AppError::new(
                ErrorType::MissingGitRepository,
                "Looks like you're not in a valid git repository!".to_string(),
            )),
        }
    }

    fn get_local_ref(&self, git: &impl GitTrait) -> Result<Local, AppError> {
        if self.branch.is_none() {
            match git.get_local_branch()? {
                GitOutput::Ok(output) => Ok(Local::Branch(Cow::Owned(output))),
                GitOutput::Err(_) => Ok(Local::NotBranch),
            }
        } else {
            Ok(Local::Branch(Cow::Borrowed(self.branch.as_ref().unwrap())))
        }
    }

    /// Populates the remote variable within [`GitView`]
    /// User Given Remote -> Default Remote in Config -> Tracked Remote -> 'origin'
    fn populate_remote(
        &self,
        local: &Local,
        git: &impl GitTrait,
    ) -> Result<Cow<'_, str>, AppError> {
        // Priority goes to user given remote
        if self.remote.is_none() {
            match local {
                Local::Branch(branch) => {
                    // Priority then goes to the default remote
                    match git.get_default_remote()? {
                        GitOutput::Ok(def) => Ok(Cow::Owned(def)),
                        // Priority then goes to the tracked remote
                        GitOutput::Err(_) => match git.get_tracked_remote(branch)? {
                            GitOutput::Ok(tracked) => Ok(Cow::Owned(tracked)),
                            // Default to the 'origin' remote
                            GitOutput::Err(_) => Ok(Cow::Owned("origin".into())),
                        },
                    }
                }
                Local::NotBranch => Ok(Cow::Owned("origin".into())),
            }
        } else {
            Ok(Cow::Borrowed(self.remote.as_ref().unwrap()))
        }
    }

    fn get_remote_reference(
        &self,
        local: &'a Local,
        git: &impl GitTrait,
    ) -> Result<Cow<'a, str>, AppError> {
        match local {
            Local::Branch(branch) => {
                match git.get_upstream_branch(branch)? {
                    GitOutput::Ok(output) => Ok(Cow::Owned(
                        output.trim_start_matches("refs/heads/").to_string(),
                    )),
                    // If retrieving the upstream_branch fails, that means that there is no valid
                    // upstream branch (surprise surprise)
                    //
                    // When there's no valid remote branch, we should default to the repository's
                    // default branch, for the vast majority of cases, this will be either "main"
                    // or "master" but it could be different for whatever INSANE person has changed
                    // their default to differ from those two terms.
                    //
                    // Thankfully, we have a command 'git rev-parse --abbrev-ref <remote>/HEAD'
                    // to let us retrieve the default branch (we also need to split on the first /
                    // encountered and take the second split part)
                    //
                    // However, it's a bit dodgy so we can't guarantee it will work everytime. If
                    // the command 'git rev-parse --abbrev-ref <remote>/HEAD' fails, we should just
                    // default to the local branch and the user will just have to suck it up and
                    // deal with the 404 error that they will probably get.
                    GitOutput::Err(_) => Ok(Cow::Borrowed(branch)),
                }
            }
            // Priority is given to the current tag
            Local::NotBranch => match git.get_current_tag()? {
                GitOutput::Ok(tag) => Ok(Cow::Owned(tag)),
                // Priority is then given the current commit
                GitOutput::Err(_) => match git.get_current_commit()? {
                    GitOutput::Ok(commit_hash) => Ok(Cow::Owned(commit_hash)),
                    // Error out if even the current commit could not be found
                    GitOutput::Err(err) => Err(AppError::new(ErrorType::CommandFailed, err)),
                },
            },
        }
    }

    fn get_git_url(&self, remote: &str, git: &impl GitTrait) -> Result<String, AppError> {
        match git.is_valid_remote(remote)? {
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

    fn generate_final_url(
        &self,
        remote_ref: &str,
        url: &Url,
        git: &impl GitTrait,
    ) -> Result<String, AppError> {
        let mut open_url = format!("{}://{}/{}", url.protocol, url.domain, url.path);

        // Handle commit flag
        if let Some(commit) = self.commit {
            if commit == "latest" {
                let commit_hash = match git.get_current_commit()? {
                    GitOutput::Ok(hash) => Ok(hash),
                    GitOutput::Err(err) => Err(AppError::new(ErrorType::CommandFailed, err)),
                }?;
                open_url.push_str(format!("/tree/{}", commit_hash).as_str());
            } else {
                open_url.push_str(format!("/tree/{}", commit).as_str());
            }

            return Ok(open_url);
        }

        // Handle issue flag and no flags
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

        if remote_ref == "master" || remote_ref == "main" {
            Ok(open_url)
        } else {
            open_url.push_str(&branch_ref);
            Ok(open_url)
        }
    }
}

fn capture_digits(remote_ref: &str) -> &str {
    let mut start = 0;
    let mut end = 0;
    let mut found = false;

    for (indice, grapheme) in remote_ref.char_indices() {
        if found {
            if grapheme.is_numeric() {
                end = indice;
            } else {
                break;
            }
        } else if grapheme.is_numeric() {
            start = indice;
            found = true;
        }
    }

    if found {
        &remote_ref[start..=end]
    } else {
        remote_ref
    }
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
            git::{GitOutput, MockGitTrait},
            lib_tests::instantiate_handler,
        };

        #[test]
        fn yes() {
            let handler = instantiate_handler();

            let mut mock = MockGitTrait::default();
            mock.expect_is_valid_repository()
                .returning(|| Ok(GitOutput::Ok("Valid".to_owned())));

            let is_valid_repository = handler.is_valid_repository(&mock);

            assert!(is_valid_repository.is_ok());
        }

        #[test]
        fn no() {
            let handler = instantiate_handler();
            let mut mock = MockGitTrait::default();
            mock.expect_is_valid_repository()
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

    mod get_local_ref {

        #[test]
        fn user_given_branch() {}

        #[test]
        fn is_branch() {}

        #[test]
        fn is_not_branch() {}
    }

    mod parse_git_url {
        use crate::{error::AppError, lib_tests::instantiate_handler};
        use test_case::test_case;

        #[test_case("https://github.com:8080/sgoudham/git-view.git" ; "with port")]
        #[test_case("https://github.com/sgoudham/git-view.git"      ; "normal")]
        #[test_case("https://github.com/sgoudham/git-view.git/"     ; "with trailing slash")]
        fn https(git_url: &str) -> Result<(), AppError> {
            let handler = instantiate_handler();

            let url = handler.parse_git_url(git_url)?;

            assert_eq!(url.protocol, "https");
            assert_eq!(url.domain.to_string(), "github.com");
            assert_eq!(url.path, "sgoudham/git-view");

            Ok(())
        }

        #[test_case("git@github.com:sgoudham/git-view.git"  ; "with username")]
        #[test_case("github.com:sgoudham/git-view.git"      ; "normal")]
        #[test_case("github.com:sgoudham/git-view.git/"     ; "with trailing slash")]
        fn scp_like(git_url: &str) -> Result<(), AppError> {
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

    mod capture_digits {
        use test_case::test_case;

        use crate::capture_digits;

        #[test_case("TICKET-WITH-NO-NUMBERS",   "TICKET-WITH-NO-NUMBERS"    ; "with no numbers")]
        #[test_case("🥵🥵Hazel🥵-1234🥵🥵",     "1234"                      ; "with emojis")]
        #[test_case("TICKET-1234-To-V10",       "1234"                      ; "with multiple issue numbers")]
        #[test_case("TICKET-1234",              "1234"                      ; "with issue number at end")]
        #[test_case("1234-TICKET",              "1234"                      ; "with issue number at start")]
        #[test_case("1234",                     "1234"                      ; "with no letters")]
        fn branch(input: &str, expected_remote_ref: &str) {
            let actual_remote_ref = capture_digits(input);
            assert_eq!(actual_remote_ref, expected_remote_ref);
        }
    }

    mod escape_ascii_chars {
        use test_case::test_case;

        use crate::escape_ascii_chars;

        #[test_case("🥵🥵Hazel🥵-%1234#🥵🥵",   "🥵🥵Hazel🥵-%251234%23🥵🥵"    ; "with emojis")]
        #[test_case("TICKET-%1234#",            "TICKET-%251234%23"             ; "with hashtag and percentage")]
        #[test_case("TICKET-%1234",             "TICKET-%251234"                ; "with percentage")]
        #[test_case("TICKET-#1234",             "TICKET-%231234"                ; "with hashtag")]
        #[test_case("TICKET",                   "TICKET"                        ; "with only alphabet")]
        fn branch(input: &str, expected_remote_ref: &str) {
            let actual_remote_ref = escape_ascii_chars(input);
            assert_eq!(actual_remote_ref, expected_remote_ref);
        }
    }
}
