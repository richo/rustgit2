use std::libc::{c_void,c_char,c_int};
use std::mem;
use std::c_str::CString;
use oid::GitOid;
use odb::GitOdb;

pub mod oid;
pub mod odb;

struct Repository {
    repo: *mut c_void,
}

impl Repository {

    fn odb(&self) -> Option<GitOdb> {
        let odb: GitOdb = unsafe { mem::init() };
        let err = unsafe { git_repository_odb(&odb.odb, self.repo) };

        if err == 0 {
            return Some(odb);
        } else {
            return None;
        }
    }

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

    fn each_object(&self, cb: |*GitOid| -> u8) {
        unsafe {
            let odb = self.odb().unwrap();
            let err = git_odb_foreach(odb.odb, each_object_wrapper, &cb);
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe { std::libc::free(self.repo) };
    }
}

extern "C" fn each_object_wrapper(oid: *GitOid, cb: |*GitOid| -> u8) -> u8 {
    return cb(oid);
}

#[link(name="git2")]
extern {
    fn git_repository_open(repo: **mut c_void, path: &str) -> c_int;
    // TODO This pointer is passed in effectively as const
    fn git_repository_path(repo: *mut c_void) -> *c_char;
    fn git_repository_odb(odb: **mut c_void, repo: *mut c_void) -> c_int;
    fn git_odb_foreach(repo: *mut c_void, cb: extern "C" fn(*GitOid, |*GitOid| -> u8) -> u8, data: *|*GitOid| -> u8) -> c_int;
}
