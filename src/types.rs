use std::libc::{size_t, ssize_t};

// FIXME Work out why enum types aren't valid as int assignments
// enum GitCvarT {
pub static
    GIT_CVAR_FALSE: int = 0;
pub static
    GIT_CVAR_TRUE: int = 1;
pub static
    GIT_CVAR_INT32: int = 2;
pub static
    GIT_CVAR_STRING: int = 3;
// }

enum GitCvarCached {
	GIT_CVAR_AUTO_CRLF = 0, /* core.autocrlf */
	GIT_CVAR_EOL,           /* core.eol */
	GIT_CVAR_SYMLINKS,      /* core.symlinks */
	GIT_CVAR_IGNORECASE,    /* core.ignorecase */
	GIT_CVAR_FILEMODE,      /* core.filemode */
	GIT_CVAR_IGNORESTAT,    /* core.ignorestat */
	GIT_CVAR_TRUSTCTIME,    /* core.trustctime */
	GIT_CVAR_ABBREV,        /* core.abbrev */
	GIT_CVAR_PRECOMPOSE,    /* core.precomposeunicode */
	// GIT_CVAR_CACHE_MAX
}
pub static GIT_CVAR_CACHE_MAX: int = 9;

// FIXME Work out how to do enum types with repeated values
// enum GitCvarValue {
pub static
	/* The value hasn't been loaded from the cache yet */
	GIT_CVAR_NOT_CACHED: int = -1;

	/* core.safecrlf: false, 'fail', 'warn' */
pub static
	GIT_SAFE_CRLF_FALSE: int = 0;
pub static
	GIT_SAFE_CRLF_FAIL: int = 1;
pub static
	GIT_SAFE_CRLF_WARN: int = 2;

	/* core.autocrlf: false, true, 'input; */
pub static
	GIT_AUTO_CRLF_FALSE: int = 0;
pub static
	GIT_AUTO_CRLF_TRUE: int = 1;
pub static
	GIT_AUTO_CRLF_INPUT: int = 2;
pub static
	GIT_AUTO_CRLF_DEFAULT: int = GIT_AUTO_CRLF_FALSE;

	/* core.eol: unset, 'crlf', 'lf', 'native' */
pub static
	GIT_EOL_UNSET: int = 0;
pub static
	GIT_EOL_CRLF: int = 1;
pub static
	GIT_EOL_LF: int = 2;
#[cfg(target_os = "win32")]
pub static
	GIT_EOL_NATIVE: int = GIT_EOL_CRLF;
#[cfg(not(target_os = "win32"))]
pub static
	GIT_EOL_NATIVE: int = GIT_EOL_LF;
// #endif
pub static
	GIT_EOL_DEFAULT: int = GIT_EOL_NATIVE;

	/* core.symlinks: bool */
pub static
	GIT_SYMLINKS_DEFAULT: int = GIT_CVAR_TRUE;
	/* core.ignorecase */
pub static
	GIT_IGNORECASE_DEFAULT: int = GIT_CVAR_FALSE;
	/* core.filemode */
pub static
	GIT_FILEMODE_DEFAULT: int = GIT_CVAR_TRUE;
	/* core.ignorestat */
pub static
	GIT_IGNORESTAT_DEFAULT: int = GIT_CVAR_FALSE;
	/* core.trustctime */
pub static
	GIT_TRUSTCTIME_DEFAULT: int = GIT_CVAR_TRUE;
	/* core.abbrev */
pub static
	GIT_ABBREV_DEFAULT: int = 7;
	/* core.precomposeunicode */
pub static
	GIT_PRECOMPOSE_DEFAULT: int = GIT_CVAR_FALSE;
// }


pub struct GitCache {
    git_oidmap: *u8,      // git_oidmap *map;
    lock: GitMutex,       // git_mutex   lock;
    used_memory: ssize_t, // ssize_t     used_memory;
}

pub struct GitAttrCache {
    initialized: int,   // int initialized,
    pool: GitPool,      // git_pool pool,
    files: *u8,         // git_strmap *files,   /* hash path to git_attr_file of rules */
    macros: *u8,        // git_strmap *macros,  /* hash name to vector<git_attr_assignment> */
    cfg_attr_file: *u8, // char *cfg_attr_file, /* cached value of core.attributesfile */
    cfg_excl_file: *u8, // char *cfg_excl_file, /* cached value of core.excludesfile */
}

pub struct GitMutex;

pub struct GitPool;
