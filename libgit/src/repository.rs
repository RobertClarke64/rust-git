use std::path::PathBuf;
use std::fmt;

pub struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
    conf: Option<String>,
}

impl GitRepository {

    pub fn new(path: PathBuf, force: bool) -> Result<GitRepository, String> {
        let gitdir = path.join(".git");

        if !(force || gitdir.exists()) {
            return Err(format!("Not a git repository {}", path.to_str().unwrap()));
        }

        Ok(GitRepository {
            worktree: PathBuf::new(),
            gitdir,
            conf: None,
        })
    }

}

impl fmt::Debug for GitRepository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GitRepository: {}", self.gitdir.to_str().unwrap())
    }
}
