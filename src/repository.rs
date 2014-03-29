use std::libc::{c_void,c_char};
use types::{GitCache,GitAttrCache,GIT_CVAR_CACHE_MAX};
use std::mem;
use std::c_str::CString;

pub mod types;

struct Repository {
    repo: *mut c_void,
}

impl Repository {
    fn path(&self) -> Option<~str> {
        unsafe {
            let path = git_repository_path(self.repo);
            match CString::new(path, true).as_str() {
                Some(s) => return Some(s.to_owned()),
                None => return None
            }
        }
    }

    fn open(path: &str) -> Option<Repository> {
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
}

#[link(name="git2")]
extern {
    fn git_repository_open(repo: **mut c_void, path: &str) -> u8;
    fn git_repository_path(repo: *mut c_void) -> *c_char;
}

}
