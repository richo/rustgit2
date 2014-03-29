use std::libc::{c_void,c_char};
use types::{GitCache,GitAttrCache,GIT_CVAR_CACHE_MAX};
use std::mem;

pub mod types;


struct Repository {
    repo: *mut c_void,
}

#[link(name="git2")]
extern {
    fn git_repository_open(repo: **mut c_void, path: &str) -> u8;
}

pub fn new_repository(path: &str) -> Option<Repository> {
    unsafe {
        let mut repo: Repository = mem::init();
        let err = git_repository_open(&repo.repo, path);
        if (err == 0) {
            return Some(repo);
        } else {
            return None;
        }
    }
}
