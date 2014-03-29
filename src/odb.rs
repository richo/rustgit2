use std::libc::{c_void};
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
