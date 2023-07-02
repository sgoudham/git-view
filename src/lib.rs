mod error;
mod git;

use std::borrow::Cow;

use error::{AppError, ErrorType};
use git::{GitOutput, GitTrait, Local, Url};

pub use git::Git;

#[derive(Default)]
pub struct GitView<'a> {
    remote: Option<&'a str>,
    branch: Option<&'a str>,
    commit: Option<&'a str>,
    issue: Option<&'a str>,
    path: Option<&'a str>,
    is_print: bool,
}

impl<'a> GitView<'a> {
    pub fn new(
        branch: Option<&'a str>,
        remote: Option<&'a str>,
        commit: Option<&'a str>,
        issue: Option<&'a str>,
        path: Option<&'a str>,
        is_print: bool,
    ) -> Self {
        Self {
            remote,
            branch,
            commit,
            issue,
            path,
            is_print,
        }
    }

    pub fn view_repository(&self, git: impl GitTrait) -> Result<(), AppError> {
        self.is_valid_repository(&git)?;
        let local_ref = self.get_local_ref(&git)?;
        let remote = self.populate_remote(&local_ref, &git)?;
        let remote_ref = self.get_remote_reference(&local_ref, &remote, &git)?;

        // Retrieve the full git_url
        // e.g https://github.com/sgoudham/git-view.git
        let git_url = self.get_git_url(&remote, &git)?;
        let url = self.parse_git_url(&git_url)?;
        let final_url = self.generate_final_url(&remote_ref, &url, &git)?;

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
        remote: &'a str,
        git: &impl GitTrait,
    ) -> Result<Cow<'a, str>, AppError> {
        match local {
            Local::Branch(branch) => {
                match git.get_upstream_branch(branch)? {
                    GitOutput::Ok(output) => Ok(Cow::Owned(
                        output.trim_start_matches("refs/heads/").to_string(),
                    )),
                    // Upstream branch doesn't exist, try to retrieve default remote branch
                    GitOutput::Err(_) => match git.get_default_branch(remote)? {
                        GitOutput::Ok(default_branch) => match default_branch.split_once('/') {
                            Some((_, split_branch)) => Ok(Cow::Owned(split_branch.into())),
                            None => Ok(Cow::Borrowed(branch)),
                        },
                        // Default branch couldn't be retrieved, just use the local branch
                        // (this WILL result in a 404 error on the user but better than failing?)
                        GitOutput::Err(_) => Ok(Cow::Borrowed(branch)),
                    },
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
                    url.host_str().map_or_else(|| "github.com", |host| host),
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

                    Ok(Url::new(protocol, split_domain, path))
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
        let escaped_remote_ref = escape_ascii_chars(remote_ref);

        if let Some(issue) = self.issue {
            self.handle_issue_flag(issue, &escaped_remote_ref, &mut open_url)?;
            return Ok(open_url);
        }
        if let Some(commit) = self.commit {
            self.handle_commit_flag(commit, &mut open_url, git)?;
            return Ok(open_url);
        }
        if let Some(path) = self.path {
            let prefix = format!("/tree/{}", escaped_remote_ref);
            self.handle_path_flag(Some(prefix.as_str()), path, &mut open_url, git)?;
            return Ok(open_url);
        }

        open_url.push_str(format!("/tree/{}", escaped_remote_ref).as_str());

        Ok(open_url)
    }

    fn handle_issue_flag(
        &self,
        issue: &str,
        remote_ref: &str,
        open_url: &mut String,
    ) -> Result<(), AppError> {
        if issue == "branch" {
            if let Some(issue_num) = capture_digits(remote_ref) {
                open_url.push_str(format!("/issues/{}", issue_num).as_str());
            } else {
                open_url.push_str("/issues");
            }
        } else {
            open_url.push_str(format!("/issues/{}", issue).as_str());
        }

        Ok(())
    }

    fn handle_commit_flag(
        &self,
        commit: &str,
        open_url: &mut String,
        git: &impl GitTrait,
    ) -> Result<(), AppError> {
        if commit == "current" {
            match git.get_current_commit()? {
                GitOutput::Ok(hash) => {
                    open_url.push_str(format!("/tree/{}", hash).as_str());
                }
                GitOutput::Err(err) => return Err(AppError::new(ErrorType::CommandFailed, err)),
            };
        } else {
            open_url.push_str(format!("/tree/{}", commit).as_str());
        }

        // path can still be appended after commit hash
        if let Some(path) = self.path {
            // prefix is empty because trailing slash will be added
            self.handle_path_flag(None, path, open_url, git)?;
        }

        Ok(())
    }

    fn handle_path_flag(
        &self,
        prefix: Option<&str>,
        path: &str,
        open_url: &mut String,
        git: &impl GitTrait,
    ) -> Result<(), AppError> {
        if path == "current-working-directory" {
            match git.get_current_working_directory()? {
                GitOutput::Ok(cwd) => {
                    // If the current working directory is not the root of the repo, append it
                    if !cwd.is_empty() {
                        open_url.push_str(format!("{}/{}", prefix.unwrap(), cwd).as_str());
                    }
                }
                GitOutput::Err(err) => return Err(AppError::new(ErrorType::CommandFailed, err)),
            }
        } else if let Some(prefix) = prefix {
            open_url.push_str(format!("{}/{}", prefix, path).as_str());
        } else {
            open_url.push_str(format!("/{}", path).as_str());
        }

        Ok(())
    }
}

fn capture_digits(remote_ref: &str) -> Option<&str> {
    let mut start = 0;
    let mut end = 0;
    let mut found = false;

    for (indice, char) in remote_ref.char_indices() {
        if found {
            if char.is_numeric() {
                end = indice;
            } else {
                break;
            }
        } else if char.is_numeric() {
            start = indice;
            found = true;
        }
    }

    if found {
        Some(&remote_ref[start..=end])
    } else {
        None
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

    impl<'a> GitView<'a> {
        fn builder() -> GitViewBuilder<'a> {
            GitViewBuilder::default()
        }
    }

    #[derive(Default)]
    pub(crate) struct GitViewBuilder<'a> {
        remote: Option<&'a str>,
        branch: Option<&'a str>,
        commit: Option<&'a str>,
        issue: Option<&'a str>,
        path: Option<&'a str>,
        is_print: bool,
    }

    impl<'a> GitViewBuilder<'a> {
        pub(crate) fn with_remote(mut self, remote: &'a str) -> Self {
            self.remote = Some(remote);
            self
        }

        pub(crate) fn with_branch(mut self, branch: &'a str) -> Self {
            self.branch = Some(branch);
            self
        }

        pub(crate) fn with_commit(mut self, commit: &'a str) -> Self {
            self.commit = Some(commit);
            self
        }

        pub(crate) fn with_issue(mut self, issue: &'a str) -> Self {
            self.issue = Some(issue);
            self
        }

        pub(crate) fn with_path(mut self, path: &'a str) -> Self {
            self.path = Some(path);
            self
        }

        pub(crate) fn build(self) -> GitView<'a> {
            GitView::new(
                self.branch,
                self.remote,
                self.commit,
                self.issue,
                self.path,
                self.is_print,
            )
        }
    }

    mod is_valid_repository {
        use crate::{
            error::ErrorType,
            git::{GitOutput, MockGitTrait},
            GitView,
        };

        #[test]
        fn yes() {
            let handler = GitView::default();
            let mut mock = MockGitTrait::default();

            mock.expect_is_valid_repository()
                .returning(|| Ok(GitOutput::Ok("Valid".to_owned())));

            let is_valid_repository = handler.is_valid_repository(&mock);

            assert!(is_valid_repository.is_ok());
        }

        #[test]
        fn no() {
            let handler = GitView::default();
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
        use std::borrow::Cow;

        use crate::{
            git::{GitOutput, MockGitTrait},
            GitView, Local,
        };

        #[test]
        fn user_given_branch() {
            let handler = GitView::builder().with_branch("main").build();
            let mock = MockGitTrait::default();
            let expected_local_ref = Ok(Local::Branch(Cow::Borrowed("main")));

            let actual_local_ref = handler.get_local_ref(&mock);

            assert!(actual_local_ref.is_ok());
            assert_eq!(actual_local_ref, expected_local_ref);
        }

        #[test]
        fn is_branch() {
            let handler = GitView::default();
            let mut mock = MockGitTrait::default();
            let expected_local_ref = Ok(Local::Branch(Cow::Borrowed("dev")));

            mock.expect_get_local_branch()
                .returning(|| Ok(GitOutput::Ok("dev".into())));

            let actual_local_ref = handler.get_local_ref(&mock);

            assert!(actual_local_ref.is_ok());
            assert_eq!(actual_local_ref, expected_local_ref);
        }

        #[test]
        fn is_not_branch() {
            let handler = GitView::default();
            let mut mock = MockGitTrait::default();
            let expected_local_ref = Ok(Local::NotBranch);

            mock.expect_get_local_branch()
                .returning(|| Ok(GitOutput::Err("Error".into())));

            let actual_local_ref = handler.get_local_ref(&mock);

            assert!(actual_local_ref.is_ok());
            assert_eq!(actual_local_ref, expected_local_ref);
        }
    }

    mod populate_remote {
        use std::borrow::Cow;

        use mockall::predicate::eq;

        use crate::{
            git::{GitOutput, MockGitTrait},
            GitView, Local,
        };

        #[test]
        fn is_not_branch() {
            let handler = GitView::builder().with_remote("origin").build();
            let mock = MockGitTrait::default();

            let actual_remote = handler.populate_remote(&Local::NotBranch, &mock);

            assert!(actual_remote.is_ok());
            assert_eq!(actual_remote.unwrap(), "origin");
        }

        #[test]
        fn user_given_remote() {
            let handler = GitView::builder().with_remote("origin").build();
            let mock = MockGitTrait::default();

            let actual_remote = handler.populate_remote(&Local::Branch(Cow::Borrowed("")), &mock);

            assert!(actual_remote.is_ok());
            assert_eq!(actual_remote.unwrap(), handler.remote.unwrap());
        }

        #[test]
        fn is_default_remote() {
            let handler = GitView::default();
            let mut mock = MockGitTrait::default();

            mock.expect_get_default_remote()
                .returning(|| Ok(GitOutput::Ok("default_remote".into())));

            let actual_remote = handler.populate_remote(&Local::Branch(Cow::Borrowed("")), &mock);

            assert!(actual_remote.is_ok());
            assert_eq!(actual_remote.unwrap(), "default_remote");
        }

        #[test]
        fn is_tracked_remote() {
            let handler = GitView::default();
            let mut mock = MockGitTrait::default();

            mock.expect_get_default_remote()
                .returning(|| Ok(GitOutput::Err("error".into())));
            mock.expect_get_tracked_remote()
                .with(eq("branch"))
                .returning(|_| Ok(GitOutput::Ok("tracked_remote".into())));

            let actual_remote =
                handler.populate_remote(&Local::Branch(Cow::Borrowed("branch")), &mock);

            assert!(actual_remote.is_ok());
            assert_eq!(actual_remote.unwrap(), "tracked_remote");
        }

        #[test]
        fn is_not_default_or_tracked() {
            let handler = GitView::default();
            let mut mock = MockGitTrait::default();

            mock.expect_get_default_remote()
                .returning(|| Ok(GitOutput::Err("error".into())));
            mock.expect_get_tracked_remote()
                .with(eq("branch"))
                .returning(|_| Ok(GitOutput::Err("error".into())));

            let actual_remote =
                handler.populate_remote(&Local::Branch(Cow::Borrowed("branch")), &mock);

            assert!(actual_remote.is_ok());
            assert_eq!(actual_remote.unwrap(), "origin");
        }
    }

    mod get_remote_reference {
        use std::borrow::Cow;

        use crate::{
            error::ErrorType,
            git::{GitOutput, MockGitTrait},
            GitView, Local,
        };

        #[test]
        fn is_branch_and_exists_on_remote() {
            let handler = GitView::default();
            let local = Local::Branch(Cow::Borrowed("main"));
            let mut mock = MockGitTrait::default();

            mock.expect_get_upstream_branch()
                .returning(|_| Ok(GitOutput::Ok("refs/heads/main".into())));

            let actual_upstream_branch = handler.get_remote_reference(&local, "origin", &mock);

            assert!(actual_upstream_branch.is_ok());
            assert_eq!(actual_upstream_branch.unwrap(), "main");
        }

        #[test]
        fn is_branch_and_successfully_get_default() {
            let handler = GitView::default();
            let local = Local::Branch(Cow::Borrowed("main"));
            let mut mock = MockGitTrait::default();

            mock.expect_get_upstream_branch()
                .returning(|_| Ok(GitOutput::Err("error".into())));
            mock.expect_get_default_branch()
                .returning(|_| Ok(GitOutput::Ok("origin/main".into())));

            let actual_upstream_branch = handler.get_remote_reference(&local, "origin", &mock);

            assert!(actual_upstream_branch.is_ok());
            assert_eq!(actual_upstream_branch.unwrap(), "main")
        }

        #[test]
        fn is_branch_and_fail_to_get_default() {
            let handler = GitView::default();
            let local = Local::Branch(Cow::Borrowed("main"));
            let mut mock = MockGitTrait::default();

            mock.expect_get_upstream_branch()
                .returning(|_| Ok(GitOutput::Err("error".into())));
            mock.expect_get_default_branch()
                .returning(|_| Ok(GitOutput::Err("error".into())));

            let actual_upstream_branch = handler.get_remote_reference(&local, "origin", &mock);

            assert!(actual_upstream_branch.is_ok());
            assert_eq!(actual_upstream_branch.unwrap(), "main")
        }

        #[test]
        fn not_branch_and_get_current_tag() {
            let handler = GitView::default();
            let local = Local::NotBranch;
            let mut mock = MockGitTrait::default();

            mock.expect_get_current_tag()
                .returning(|| Ok(GitOutput::Ok("v1.0.0".into())));

            let actual_upstream_branch = handler.get_remote_reference(&local, "origin", &mock);

            assert!(actual_upstream_branch.is_ok());
            assert_eq!(actual_upstream_branch.unwrap(), "v1.0.0")
        }

        #[test]
        fn not_branch_and_get_current_commit() {
            let handler = GitView::default();
            let local = Local::NotBranch;
            let mut mock = MockGitTrait::default();

            mock.expect_get_current_tag()
                .returning(|| Ok(GitOutput::Err("error".into())));
            mock.expect_get_current_commit()
                .returning(|| Ok(GitOutput::Ok("hash".into())));

            let actual_upstream_branch = handler.get_remote_reference(&local, "origin", &mock);

            assert!(actual_upstream_branch.is_ok());
            assert_eq!(actual_upstream_branch.unwrap(), "hash")
        }

        #[test]
        fn not_branch_and_no_tag_or_commit() {
            let handler = GitView::default();
            let local = Local::NotBranch;
            let mut mock = MockGitTrait::default();

            mock.expect_get_current_tag()
                .returning(|| Ok(GitOutput::Err("error".into())));
            mock.expect_get_current_commit()
                .returning(|| Ok(GitOutput::Err("error".into())));

            let actual_upstream_branch = handler.get_remote_reference(&local, "origin", &mock);

            assert!(actual_upstream_branch.is_err());

            let error = actual_upstream_branch.as_ref().unwrap_err();
            assert_eq!(error.error_type, ErrorType::CommandFailed);
            assert_eq!(error.error_str, "error");
        }
    }

    mod get_git_url {
        use crate::{
            error::{AppError, ErrorType},
            git::{GitOutput, MockGitTrait},
            GitView,
        };

        #[test]
        fn is_valid_remote() {
            let handler = GitView::default();
            let expected_remote = "origin";
            let mut mock = MockGitTrait::default();

            mock.expect_is_valid_remote()
                .returning(|_| Ok(GitOutput::Ok("https://github.com/sgoudham/git-view".into())));

            let actual_remote = handler.get_git_url(expected_remote, &mock);

            assert!(actual_remote.is_ok());
            assert_eq!(
                actual_remote.unwrap(),
                "https://github.com/sgoudham/git-view"
            )
        }

        #[test]
        fn is_not_valid_remote() {
            let handler = GitView::default();
            let expected_remote = "origin";
            let mut mock = MockGitTrait::default();

            mock.expect_is_valid_remote()
                .returning(|_| Ok(GitOutput::Ok("origin".into())));

            let actual_remote = handler.get_git_url(expected_remote, &mock);

            assert!(actual_remote.is_err());
            assert_eq!(
                actual_remote.unwrap_err().error_str,
                "Looks like your git remote isn't set for 'origin'"
            );
        }

        #[test]
        fn command_failed() {
            let handler = GitView::default();
            let expected_remote = "origin";
            let mut mock = MockGitTrait::default();

            mock.expect_is_valid_remote()
                .returning(|_| Err(AppError::new(ErrorType::CommandFailed, "error".into())));

            let actual_remote = handler.get_git_url(expected_remote, &mock);

            assert!(actual_remote.is_err());
            assert_eq!(actual_remote.unwrap_err().error_str, "error");
        }
    }

    mod parse_git_url {
        use crate::{error::AppError, GitView};
        use test_case::test_case;

        #[test_case("https://github.com:8080/sgoudham/git-view.git" ; "with port")]
        #[test_case("https://github.com/sgoudham/git-view.git"      ; "normal")]
        #[test_case("https://github.com/sgoudham/git-view.git/"     ; "with trailing slash")]
        fn https(git_url: &str) -> Result<(), AppError> {
            let handler = GitView::default();

            let url = handler.parse_git_url(git_url)?;

            assert_eq!(url.protocol, "https");
            assert_eq!(url.domain, "github.com");
            assert_eq!(url.path, "sgoudham/git-view");

            Ok(())
        }

        #[test_case("git@github.com:sgoudham/git-view.git"  ; "with username")]
        #[test_case("github.com:sgoudham/git-view.git"      ; "normal")]
        #[test_case("github.com:sgoudham/git-view.git/"     ; "with trailing slash")]
        fn scp_like(git_url: &str) -> Result<(), AppError> {
            let handler = GitView::default();

            let url = handler.parse_git_url(git_url)?;

            assert_eq!(url.protocol, "https");
            assert_eq!(url.domain, "github.com");
            assert_eq!(url.path, "sgoudham/git-view");

            Ok(())
        }

        #[test]
        fn invalid_git_url() {
            let handler = GitView::default();
            let git_url_normal = "This isn't a git url";

            let error = handler.parse_git_url(git_url_normal);

            assert!(error.is_err());
            assert_eq!(
                error.unwrap_err().error_str,
                "Sorry, couldn't parse git url 'This isn't a git url'"
            );
        }
    }

    mod generate_final_url {
        use crate::{
            git::{GitOutput, MockGitTrait, Url},
            GitView,
        };
        use test_case::test_case;

        #[test]
        fn is_latest_commit() {
            let handler = GitView::builder().with_commit("current").build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/eafdb9a";
            let mut mock = MockGitTrait::default();

            mock.expect_get_current_commit()
                .returning(|| Ok(GitOutput::Ok("eafdb9a".into())));

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_user_commit() {
            let handler = GitView::builder()
                .with_commit("8s2jl250as7f234jasfjj")
                .build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url =
                "https://github.com/sgoudham/git-view/tree/8s2jl250as7f234jasfjj";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_latest_commit_with_path_current_working_directory() {
            let handler = GitView::builder()
                .with_commit("current")
                .with_path("src/main.rs")
                .build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url =
                "https://github.com/sgoudham/git-view/tree/eafdb9a/src/main.rs";

            let mut mock = MockGitTrait::default();
            mock.expect_get_current_commit()
                .returning(|| Ok(GitOutput::Ok("eafdb9a".into())));

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test_case("main" ; "main")]
        #[test_case("master" ; "master")]
        fn is_master_or_main(branch: &str) {
            let handler = GitView::default();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = format!("https://github.com/sgoudham/git-view/tree/{branch}");
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url(branch, &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test_case("main" ; "main")]
        #[test_case("master" ; "master")]
        fn is_master_or_main_with_issue_flag(branch: &str) {
            let handler = GitView::builder().with_issue("branch").build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/issues";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url(branch, &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_user_issue() {
            let handler = GitView::builder().with_issue("branch").build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/issues/1234";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("TICKET-1234", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_user_issue_with_args() {
            let handler = GitView::builder().with_issue("42").build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/issues/42";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_normal_branch() {
            let handler = GitView::builder().build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/%23test%23";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("#test#", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_user_path() {
            let handler = GitView::builder().with_path("src/main.rs").build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/main/src/main.rs";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_path_at_repo_root() {
            let handler = GitView::builder()
                .with_path("current-working-directory")
                .build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view";

            let mut mock = MockGitTrait::default();
            mock.expect_get_current_working_directory()
                .returning(|| Ok(GitOutput::Ok("".into())));

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_path_at_sub_directory() {
            let handler = GitView::builder()
                .with_path("current-working-directory")
                .build();
            let url = Url::new("https", "github.com", "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/main/src/";

            // `git rev-parse --show-prefix` returns relative path with a trailing slash
            let mut mock = MockGitTrait::default();
            mock.expect_get_current_working_directory()
                .returning(|| Ok(GitOutput::Ok("src/".into())));

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }
    }

    mod capture_digits {
        use test_case::test_case;

        use crate::capture_digits;

        #[test_case("🥵🥵Hazel🥵-1234🥵🥵",     "1234"                      ; "with emojis")]
        #[test_case("TICKET-1234-To-V10",       "1234"                      ; "with multiple issue numbers")]
        #[test_case("TICKET-1234",              "1234"                      ; "with issue number at end")]
        #[test_case("1234-TICKET",              "1234"                      ; "with issue number at start")]
        #[test_case("1234",                     "1234"                      ; "with no letters")]
        fn branch(input: &str, expected_remote_ref: &str) {
            let actual_remote_ref = capture_digits(input);
            assert_eq!(actual_remote_ref, Some(expected_remote_ref));
        }

        #[test]
        fn branch_no_numbers() {
            let input = "TICKET-WITH-NO-NUMBERS";
            let actual_remote_ref = capture_digits(input);
            assert_eq!(actual_remote_ref, None);
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
