use types::{GitCache,GitAttrCache,GIT_CVAR_CACHE_MAX};
use std::mem;

pub mod types;


struct Repository {
    _odb:    *u8,   // git_odb *_odb;
    _refdb:  *u8,  // git_refdb *_refdb;
    _config: *u8,  // git_config *_config;
    _index:  *u8,  // git_index *_index;

    objects: GitCache,                    // git_cache objects;
    attrcache: GitAttrCache,              // git_attr_cache attrcache;
    submodules: *u8,               // git_strmap *submodules;
    diff_drivers: *u8, // git_diff_driver_registry *diff_drivers;

    path: ~str,      // char *path_repository;
    workdir: *u8,   // char *workdir;
    namespace: *u8, // char *namespace;

    is_bare: u8,        // unsigned is_bare:1;
    lru_counter: uint,  // unsigned int lru_counter;

    cvar_cache: [int, .. GIT_CVAR_CACHE_MAX] // git_cvar_value cvar_cache[GIT_CVAR_CACHE_MAX];
}

#[link(name="git2")]
extern {
    fn git_repository_open(repo: *mut Repository, path: &str) -> u8;
}

pub fn new_repository(path: &str) -> Option<Repository> {
    unsafe {
        let mut repo: Repository = mem::init();

        if (git_repository_open(&mut repo, path) != 0) {
            return Some(repo);
        } else {
            return None;
        }
    }
}
