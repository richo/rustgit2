use std::libc::{c_void,c_char,c_int};
use std::mem;
use oid::GitOid;

pub mod oid;

// Actual struct,
// pub struct GitOdb {
//     rc: GitRefcount, // git_refcount rc;
//     backends: GitVector,// git_vector backends;
//     own_cache: GitCache, // git_cache own_cache;
// }

// Totally ok for an internal representation, treat backend as opaque
pub struct GitOdb {
    odb: *mut c_void,
}

impl GitOdb {
    pub fn each_object(&self, cb: |&GitOid| -> u8) {
        unsafe {
            let err = git_odb_foreach(self.odb, each_object_wrapper, &cb);
            println!("Error: {}", err);
        }
    }

    fn open(path: &str) -> Option<GitOdb> {
        let odb: GitOdb = unsafe { mem::init() };
        let err = unsafe { git_odb_open(&odb.odb, path) };

        if err == 0 {
            return Some(odb);
        } else {
            return None;
        }
    }
}

extern "C" fn each_object_wrapper(oid: *GitOid, cb: |&GitOid| -> u8) -> u8 {
    // TODO Successfully call callback
    return 0;
}

#[link(name="git2")]
extern {
    fn git_odb_foreach(repo: *mut c_void, cb: extern "C" fn(*GitOid, |&GitOid| -> u8) -> u8, data: *|&GitOid| -> u8) -> c_int;
    fn git_odb_open(odb: **mut c_void, dir: &str) -> u8;
}
