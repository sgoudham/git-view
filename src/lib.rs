mod error;

use std::process::Command;

use error::AppError;
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
        self.is_inside_git_repository()?;
        // Retrieve the current branch
        self.populate_branch()?;
        // Retrieve the remote
        self.populate_remote()?;

        // TODO: Figure out how to default to 'master' or 'main' if branch doesn't exist on remote

        // Retrieve the remote reference
        let remote_ref = self.get_remote_reference()?;
        // Retrieve the full git_url
        // e.g https://github.com/sgoudham/git-view.git
        let git_url = self.get_git_url()?;

        // Extract protocol, domain and urlpath
        let (protocol, domain, urlpath) = self.parse_git_url(&git_url)?;

        // Open the URL
        webbrowser::open(
            format!("{}://{}{}/tree/{}", protocol, domain, urlpath, remote_ref).as_str(),
        )?;

        Ok(())
    }

    fn is_inside_git_repository(&self) -> Result<(), AppError> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(AppError::MissingGitRepository(String::from(
                "Looks like you're not in a valid git repository!",
            )))
        }
    }

    fn populate_branch(&mut self) -> Result<(), AppError> {
        if self.branch.is_none() {
            let branch = Command::new("git")
                .arg("symbolic-ref")
                .arg("-q")
                .arg("--short")
                .arg("HEAD")
                .output()?;

            if branch.status.success() {
                match stdout_to_string(&branch.stdout) {
                    Ok(str) => self.branch = Some(str),
                    Err(_) => {
                        return Err(AppError::InvalidUtf8(String::from(
                            "Git branch is not valid UTF-8!",
                        )))
                    }
                }
            } else {
                return Err(AppError::CommandError(
                    String::from_utf8_lossy(&branch.stderr).to_string(),
                ));
            }
        }

        Ok(())
    }

    fn get_remote_reference(&mut self) -> Result<String, AppError> {
        let absolute_upstream_branch = Command::new("git")
            .arg("config")
            .arg(format!("branch.{}.merge", self.branch.as_ref().unwrap()))
            .output()?;

        if absolute_upstream_branch.status.success() {
            match stdout_to_string(&absolute_upstream_branch.stdout) {
                Ok(str) => Ok(str.trim_start_matches("refs/heads/").to_string()),
                Err(_) => Err(AppError::InvalidUtf8(String::from(
                    "Git upstream branch is not valid UTF-8!",
                ))),
            }
        } else {
            Ok(self.branch.as_ref().unwrap().to_string())
        }
    }

    /// Populates the remote variable within [`GitView`]
    /// User Given Remote -> Default Remote in Config -> Tracked Remote -> 'origin'
    fn populate_remote(&mut self) -> Result<(), AppError> {
        // Priority goes to user given remote
        if self.remote.is_none() {
            // Priority then goes to the default remote
            let default_remote = Command::new("git")
                .arg("config")
                .arg("open.default.remote")
                .output()?;

            if default_remote.status.success() {
                return match stdout_to_string(&default_remote.stdout) {
                    Ok(str) => {
                        self.remote = Some(str);
                        Ok(())
                    }
                    Err(_) => Err(AppError::InvalidUtf8(String::from(
                        "Git default remote is not valid UTF-8!",
                    ))),
                };
            }

            // Priority then goes to the tracked remote
            let tracked_remote = Command::new("git")
                .arg("config")
                .arg(format!("branch.{}.remote", self.branch.as_ref().unwrap()))
                .output()?;

            if tracked_remote.status.success() {
                return match stdout_to_string(&tracked_remote.stdout) {
                    Ok(str) => {
                        self.remote = Some(str);
                        Ok(())
                    }
                    Err(_) => Err(AppError::InvalidUtf8(String::from(
                        "Git tracked remote is not valid UTF-8!",
                    ))),
                };
            }

            // Priority then goes to the default 'origin'
            self.remote = Some(String::from("origin"));
        }

        Ok(())
    }

    fn get_git_url(&self) -> Result<String, AppError> {
        let is_valid_remote = Command::new("git")
            .arg("ls-remote")
            .arg("--get-url")
            .arg(self.remote.as_ref().unwrap())
            .output()?;

        if is_valid_remote.status.success() {
            match stdout_to_string(&is_valid_remote.stdout) {
                Ok(str) => {
                    if &str != self.remote.as_ref().unwrap() {
                        Ok(str)
                    } else {
                        Err(AppError::MissingGitRemote(format!(
                            "Looks like your git remote isn't set for '{}'",
                            self.remote.as_ref().unwrap(),
                        )))
                    }
                }
                Err(_) => Err(AppError::InvalidUtf8(String::from(
                    "Git URL is not valid UTF-8!",
                ))),
            }
        } else {
            Err(AppError::CommandError(
                String::from_utf8_lossy(&is_valid_remote.stderr).to_string(),
            ))
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
        match Url::parse(git_url) {
            Ok(url) => Ok((
                url.scheme().to_string(),
                url.host_str()
                    .map_or_else(|| "github.com", |host| host)
                    .to_string(),
                url.path()
                    .trim_end_matches('/')
                    .trim_end_matches(".git")
                    .to_string(),
            )),
            Err(_) => Err(AppError::InvalidGitUrl(format!(
                "Sorry, couldn't parse git url '{}'",
                git_url
            ))),
        }
    }
}

fn stdout_to_string(bytes: &[u8]) -> Result<String, AppError> {
    let mut utf_8_string = String::from(std::str::from_utf8(bytes)?.trim());

    if utf_8_string.ends_with('\n') {
        utf_8_string.pop();
        if utf_8_string.ends_with('\r') {
            utf_8_string.pop();
        }
    }

    Ok(utf_8_string)
}

#[cfg(test)]
mod parse_git_url {
    use crate::{error::AppError, GitView};

    fn instantiate_handler() -> GitView {
        GitView::new(
            Some(String::from("main")),
            Some(String::from("origin")),
            Some(String::from("latest")),
            false,
        )
    }

    #[test]
    fn with_dot_git() -> Result<(), AppError> {
        let handler = instantiate_handler();

        let git_url_normal = "https://github.com/sgoudham/git-view.git";

        let (protocol, domain, urlpath) = handler.parse_git_url(git_url_normal)?;

        assert_eq!(protocol, "https");
        assert_eq!(domain, "github.com");
        assert_eq!(urlpath, "/sgoudham/git-view");

        Ok(())
    }

    #[test]
    fn with_dot_git_and_trailing_slash() -> Result<(), AppError> {
        let handler = instantiate_handler();

        let git_url_normal = "https://github.com/sgoudham/git-view.git/";

        let (protocol, domain, urlpath) = handler.parse_git_url(git_url_normal)?;

        assert_eq!(protocol, "https");
        assert_eq!(domain, "github.com");
        assert_eq!(urlpath, "/sgoudham/git-view");

        Ok(())
    }

    #[test]
    fn invalid_git_url() {
        let handler = instantiate_handler();

        let git_url_normal = "This isn't a git url";

        let error = handler.parse_git_url(git_url_normal);
        assert!(error.is_err());
        assert_eq!(
            error.unwrap_err().print(),
            "Sorry, couldn't parse git url 'This isn't a git url'"
        );
    }
}
