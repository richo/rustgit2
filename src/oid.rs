
static GIT_OID_RAWSZ: u8 =  20;

pub struct GitOid {
    oid: [u8, ..GIT_OID_RAWSZ],
}
