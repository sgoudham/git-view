use std::{fmt::format, process::Command};

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

    pub fn open_upstream_repository(&mut self) -> Result<(), String> {
        // Exit out if we're not inside a git repository
        self.is_inside_git_repository()?;
        // Retrieve the current branch
        self.populate_branch()?;
        // Retrieve the remote
        self.populate_remote()?;
        // Retrieve the upstream branch
        let upstream_branch = self.get_upstream_branch()?;
        // Retrieve the full git_url
        // e.g https://github.com/sgoudham/git-view.git
        let git_url = self.get_git_url()?;

        // Extract protocol, domain and urlpath
        let (protocol, domain, urlpath) = self.extract_args_from_git_url(&git_url);

        webbrowser::open(&git_url).expect("Sorry, I couldn't find the default browser!");

        Ok(())
    }

    fn is_inside_git_repository(&self) -> Result<(), String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()
            .expect("`git rev-parse --is-inside-work-tree`");

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from(
                "Looks like you're not in a valid git repository!",
            ))
        }
    }

    fn populate_branch(&mut self) -> Result<(), String> {
        if self.branch.is_none() {
            let branch = Command::new("git")
                .arg("symbolic-ref")
                .arg("-q")
                .arg("--short")
                .arg("HEAD")
                .output()
                .expect("`git symbolic-ref -q --short HEAD`");

            if branch.status.success() {
                match stdout_to_string(&branch.stdout) {
                    Ok(str) => self.branch = Some(str),
                    Err(_) => return Err(String::from("Git branch is not valid UTF-8!")),
                }
            } else {
                return Err(String::from_utf8_lossy(&branch.stderr).to_string());
            }
        }

        Ok(())
    }

    fn get_upstream_branch(&mut self) -> Result<String, String> {
        let absolute_upstream_branch = Command::new("git")
            .arg("config")
            .arg(format!("branch.{}.merge", self.branch.as_ref().unwrap()))
            .output()
            .expect("`git config branch.<branch>.merge`");

        if absolute_upstream_branch.status.success() {
            match stdout_to_string(&absolute_upstream_branch.stdout) {
                Ok(str) => Ok(str.trim_start_matches("refs/heads/").to_string()),
                Err(_) => Err(String::from("Git upstream branch is not valid UTF-8!")),
            }
        } else {
            Err(format!(
                "Looks like the branch '{}' doesn't exist upstream!",
                self.branch.as_ref().unwrap()
            ))
        }
    }

    /// Populates the remote variable within [`GitView`]
    /// User Given Remote -> Default Remote in Config -> Tracked Remote -> 'origin'
    fn populate_remote(&mut self) -> Result<(), String> {
        // Priority goes to user given remote
        if self.remote.is_none() {
            // Priority then goes to the default remote
            let default_remote = Command::new("git")
                .arg("config")
                .arg("open.default.remote")
                .output()
                .expect("`git config open.default.remote`");

            if default_remote.status.success() {
                return match stdout_to_string(&default_remote.stdout) {
                    Ok(str) => {
                        self.remote = Some(str);
                        Ok(())
                    }
                    Err(_) => Err(String::from("Git default remote is not valid UTF-8!")),
                };
            }

            // Priority then goes to the tracked remote
            let tracked_remote = Command::new("git")
                .arg("config")
                .arg(format!("branch.{}.remote", self.branch.as_ref().unwrap()))
                .output()
                .expect("`git config branch.<branch>.remote`");

            if tracked_remote.status.success() {
                return match stdout_to_string(&tracked_remote.stdout) {
                    Ok(str) => {
                        self.remote = Some(str);
                        Ok(())
                    }
                    Err(_) => Err(String::from("Git tracked remote is not valid UTF-8!")),
                };
            }

            // Priority then goes to the default 'origin'
            self.remote = Some(String::from("origin"));
        }

        Ok(())
    }

    fn get_git_url(&self) -> Result<String, String> {
        let is_valid_remote = Command::new("git")
            .arg("ls-remote")
            .arg("--get-url")
            .arg(self.remote.as_ref().unwrap())
            .output()
            .expect("`git ls-remote --get-url <remote>`");

        if is_valid_remote.status.success() {
            match stdout_to_string(&is_valid_remote.stdout) {
                Ok(str) => {
                    if &str != self.remote.as_ref().unwrap() {
                        Ok(str)
                    } else {
                        Err(format!(
                            "Looks like your git remote isn't set for '{}'",
                            self.remote.as_ref().unwrap(),
                        ))
                    }
                }
                Err(_) => Err(String::from("Git URL is not valid UTF-8!")),
            }
        } else {
            Err(String::from_utf8_lossy(&is_valid_remote.stderr).to_string())
        }
    }

    fn extract_args_from_git_url(&self, git_url: &str) -> (String, String, String) {
        let mut protocol = String::from("https");
        let mut domain = String::new();
        let mut url_path = String::new();

        /*
         * To enter this if block, the 'git_url' must be in the following formats:
         *  - ssh://[user@]host.xz[:port]/path/to/repo.git/
         *  - git://host.xz[:port]/path/to/repo.git/
         *  - http[s]://host.xz[:port]/path/to/repo.git/
         *  - ftp[s]://host.xz[:port]/path/to/repo.git/
         */
        if let Some((git_protocol, uri)) = git_url.split_once("://") {

            // If the given URL is 'http', keep it that way
            if git_protocol == "http" {
                protocol = String::from("http");
            }

            // Trim potential username from URI 
            let uri = match uri.split_once('@').map(|(_username, url)| url) {
                Some(path) => path,
                None => uri,
            };

            // Retrieve domain & url_path
            match uri.split_once('/') {
                Some((dom, url)) => {
                    domain.push_str(dom);
                    url_path.push_str(url);
                }
                None => todo!(),
            }
        } else {
            todo!()
        }

        (protocol, domain, url_path)
    }
}

fn stdout_to_string(bytes: &[u8]) -> Result<String, std::str::Utf8Error> {
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
mod lib_tests {
    use crate::GitView;

    #[test]
    fn test_extract_args_from_git_url() {
        let handler = GitView::new(
            Some(String::from("main")),
            Some(String::from("origin")),
            Some(String::from("latest")),
            false,
        );
        let git_url = "https://github.com/sgoudham/git-view.git";

        let (protocol, domain, urlpath) = handler.extract_args_from_git_url(git_url);

        assert_eq!(protocol, "https");
        assert_eq!(domain, "github.com");
        assert_eq!(urlpath, "sgoudham/git-view.git")
    }
}
