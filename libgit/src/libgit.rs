mod repository;

use repository::GitRepository;

use std::path::PathBuf;

pub fn cmd_init() {
    println!("libgit::cmd_init() success");
    let path = PathBuf::from(".");
    let repo = GitRepository::new(path, false);

    dbg!(repo);

}
