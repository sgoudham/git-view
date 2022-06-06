use std::process::Command;

pub struct GitUpstream {
    remote: String,
    branch: Option<String>,
    is_commit: bool,
    is_print: bool,
}

impl GitUpstream {
    pub fn new(branch: Option<String>, remote: String, is_commit: bool, is_print: bool) -> Self {
        Self {
            remote,
            branch,
            is_commit,
            is_print,
        }
    }

    pub fn open_upstream_repository(&mut self) -> Result<(), String> {
        // Exit out if we're not inside a git repository
        self.is_inside_git_repository()?;
        // Retrieve the current branch
        self.populate_branch()?;

        let git_url = Command::new("git")
            .args(["ls-remote", "--get-url", &self.remote])
            .output()
            .expect("`git ls-remote --get-url <remote>`");

        if git_url.status.success() {
            webbrowser::open(unsafe {
                trim_string(std::str::from_utf8_unchecked(&git_url.stdout))
            })
            .expect("Couldn't find the browser!");
        } else {
            return Err(unsafe { String::from_utf8_unchecked(git_url.stderr) });
        }

        Ok(())
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
                let branch_str = unsafe { std::str::from_utf8_unchecked(&branch.stdout) };
                self.branch = Some(String::from(trim_string(branch_str)));
            } else {
                return Err(unsafe { String::from_utf8_unchecked(branch.stderr) });
            }
        }

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
            Err(String::from("Looks like you're not in a valid git repository!"))
        }
    }
}

fn trim_string(str: &str) -> &str {
    str.trim()
        .strip_suffix("\r\n")
        .or_else(|| str.strip_suffix('\n'))
        .unwrap_or(str)
}
