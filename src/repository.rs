use std::libc::{c_void,c_char,c_int};
use std::mem;
use std::c_str::CString;

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
        let repo: Repository = unsafe { mem::init() };
        let err = unsafe { git_repository_open(&repo.repo, path) };

        if err == 0 {
            return Some(repo);
        } else {
            return None;
        }
    }
}

#[link(name="git2")]
extern {
    fn git_repository_open(repo: **mut c_void, path: &str) -> c_int;
    // TODO This pointer is passed in effectively as const
    fn git_repository_path(repo: *mut c_void) -> *c_char;
}

}
