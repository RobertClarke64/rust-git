mod repository;

use std::env;
use repository::GitRepository;

use std::path::PathBuf;

pub fn cmd_init() {
    let path = env::current_dir().unwrap();
    let repo = GitRepository::new(path, false);

    dbg!(repo);

}
