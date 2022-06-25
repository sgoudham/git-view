mod error;
mod git;

use std::borrow::Cow;

use error::{AppError, ErrorType};
use git::{Domain, GitOutput, GitTrait, Local, Url};

pub use git::Git;

#[derive(Default)]
pub struct GitView<'a> {
    remote: Option<&'a str>,
    branch: Option<&'a str>,
    commit: Option<&'a str>,
    suffix: Option<&'a str>,
    is_issue: bool,
    is_print: bool,
}

impl<'a> GitView<'a> {
    pub fn new(
        branch: Option<&'a str>,
        remote: Option<&'a str>,
        commit: Option<&'a str>,
        suffix: Option<&'a str>,
        is_issue: bool,
        is_print: bool,
    ) -> Self {
        Self {
            remote,
            branch,
            commit,
            suffix,
            is_issue,
            is_print,
        }
    }

    pub fn view_repository(&mut self, git: impl GitTrait) -> Result<(), AppError> {
        // Exit out if we're not inside a git repository
        self.is_valid_repository(&git)?;
        // Retrieve the current local ref (branch or not branch)
        let local_ref = self.get_local_ref(&git)?;
        // Retrieve the remote
        let remote = self.populate_remote(&local_ref, &git)?;
        // Retrieve the remote reference
        let remote_ref = self.get_remote_reference(&local_ref, &remote, &git)?;

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
            Domain::BitBucket => {
                format!("/src/{}", remote_ref)
            }
        };

        if remote_ref != "master" && remote_ref != "main" {
            open_url.push_str(&branch_ref);
        }

        if let Some(suffix) = self.suffix {
            open_url.push_str(suffix);
        }

        Ok(open_url)
    }
}

fn capture_digits(remote_ref: &str) -> &str {
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
        suffix: Option<&'a str>,
        is_issue: bool,
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

        pub(crate) fn with_suffix(mut self, suffix: &'a str) -> Self {
            self.suffix = Some(suffix);
            self
        }

        pub(crate) fn with_issue(mut self, is_issue: bool) -> Self {
            self.is_issue = is_issue;
            self
        }

        pub(crate) fn build(self) -> GitView<'a> {
            GitView::new(
                self.branch,
                self.remote,
                self.commit,
                self.suffix,
                self.is_issue,
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
            assert_eq!(url.domain.to_string(), "github.com");
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
            assert_eq!(url.domain.to_string(), "github.com");
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
            git::{Domain, GitOutput, MockGitTrait, Url},
            GitView,
        };
        use test_case::test_case;

        #[test]
        fn is_latest_commit() {
            let handler = GitView::builder().with_commit("latest").build();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/commit_hash";
            let mut mock = MockGitTrait::default();

            mock.expect_get_current_commit()
                .returning(|| Ok(GitOutput::Ok("commit_hash".into())));

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_user_commit() {
            let handler = GitView::builder()
                .with_commit("8s2jl250as7f234jasfjj")
                .build();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let expected_final_url =
                "https://github.com/sgoudham/git-view/tree/8s2jl250as7f234jasfjj";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("main", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_bitbucket() {
            let handler = GitView::default();
            let url = Url::new("https", Domain::BitBucket, "sgoudham/git-view");
            let expected_final_url = "https://bitbucket.org/sgoudham/git-view/src/dev";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("dev", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_github() {
            let handler = GitView::default();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/dev";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("dev", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test_case("main" ; "main")]
        #[test_case("master" ; "master")]
        fn is_master_or_main(branch: &str) {
            let handler = GitView::default();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url(branch, &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_user_issue() {
            let handler = GitView::builder().with_issue(true).build();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/issues/1234";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("TICKET-1234", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test]
        fn is_normal_branch() {
            let handler = GitView::builder().with_issue(false).build();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let expected_final_url = "https://github.com/sgoudham/git-view/tree/%23test%23";
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url("#test#", &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
        }

        #[test_case("main", "https://github.com/sgoudham/git-view/releases" ; "with_branch_main")]
        #[test_case("dev", "https://github.com/sgoudham/git-view/tree/dev/releases" ; "with_branch_dev")]
        fn with_suffix(remote_ref: &str, expected_final_url: &str) {
            let handler = GitView::builder().with_suffix("/releases").build();
            let url = Url::new("https", Domain::GitHub, "sgoudham/git-view");
            let mock = MockGitTrait::default();

            let actual_final_url = handler.generate_final_url(remote_ref, &url, &mock);

            assert!(actual_final_url.is_ok());
            assert_eq!(actual_final_url.unwrap(), expected_final_url);
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
