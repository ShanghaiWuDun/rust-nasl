use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::{ gchar, gint, guint, };
use crate::libc::{ c_int, c_char, };
use crate::glib_sys::{ GHashTable, GSList, };


// @brief NVT 'Categories', influence execution order of NVTs.
//
// @todo Consider creation of an enumeration.


// First plugins actions type.
pub const ACT_FIRST: c_int              = ACT_INIT;
// Last plugins actions type.
pub const ACT_LAST: c_int               = ACT_END;


pub const ACT_UNKNOWN: c_int            = 11;
pub const ACT_END: c_int                = 10;
pub const ACT_FLOOD: c_int              =  9;
pub const ACT_KILL_HOST: c_int          =  8;
pub const ACT_DENIAL: c_int             =  7;
pub const ACT_DESTRUCTIVE_ATTACK: c_int =  6;
pub const ACT_MIXED_ATTACK: c_int       =  5;
pub const ACT_ATTACK: c_int             =  4;
pub const ACT_GATHER_INFO: c_int        =  3;
pub const ACT_SETTINGS: c_int           =  2;
pub const ACT_SCANNER: c_int            =  1;
pub const ACT_INIT: c_int               =  0;


/// @brief The structure for a preference of a NVT.
///
/// The elements of this structure should never be accessed directly.
/// Only the functions corresponding to this module should be used.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvtpref {
    /// < Preference type
    pub type_: *mut gchar,
    /// < Name of the preference
    pub name: *mut gchar,
    /// < Default value of the preference
    pub dflt: *mut gchar,
}

pub type nvtpref_t = nvtpref;

/// @brief The structure of a information record that corresponds to a NVT.
///
/// The elements of this structure should never be accessed directly.
/// Only the functions corresponding to this module should be used.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvti {
    /// < @brief Object ID
    pub oid: *mut gchar,
    /// < @brief The name
    pub name: *mut gchar,
    /// < @brief List of CVEs, this NVT corresponds to
    pub cve: *mut gchar,
    /// < @brief List of Bugtraq IDs, this NVT
    /// corresponds to
    pub bid: *mut gchar,
    /// < @brief List of Cross-references, this NVT
    /// corresponds to
    pub xref: *mut gchar,
    /// < @brief List of tags attached to this NVT
    pub tag: *mut gchar,
    /// < @brief CVSS base score for this NVT.
    pub cvss_base: *mut gchar,
    /// < @brief List of dependencies of this NVT
    pub dependencies: *mut gchar,
    /// < @brief List of required KB keys of this NVT
    pub required_keys: *mut gchar,
    /// < @brief List of mandatory KB keys of this NVT
    pub mandatory_keys: *mut gchar,
    /// < @brief List of excluded KB keys of this NVT
    pub excluded_keys: *mut gchar,
    /// < @brief List of required ports of this NVT
    pub required_ports: *mut gchar,
    /// < @brief List of required UDP ports of this NVT
    pub required_udp_ports: *mut gchar,
    /// < @brief Collection of NVT preferences
    pub prefs: *mut GSList,
    /// < @brief Default timeout time for this NVT
    pub timeout: gint,
    /// < @brief The category, this NVT belongs to
    pub category: gint,
    /// < @brief Family the NVT belongs to
    pub family: *mut gchar,
}

pub type nvti_t = nvti;

/// @brief A collection of information records corresponding to NVTs.
pub type nvtis_t = GHashTable;


#[link(name = "nasl", kind = "dylib")]
extern {
    pub fn nvtpref_new(arg1: *mut gchar, arg2: *mut gchar, arg3: *mut gchar) -> *mut nvtpref_t;
    pub fn nvtpref_free(arg1: *mut nvtpref_t);
    pub fn nvtpref_name(arg1: *const nvtpref_t) -> *mut gchar;
    pub fn nvtpref_type(arg1: *const nvtpref_t) -> *mut gchar;
    pub fn nvtpref_default(arg1: *const nvtpref_t) -> *mut gchar;
    pub fn nvti_new() -> *mut nvti_t;
    pub fn nvti_free(arg1: *mut nvti_t);
    pub fn nvti_oid(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_name(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_cve(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_bid(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_xref(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_tag(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_cvss_base(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_dependencies(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_required_keys(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_mandatory_keys(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_excluded_keys(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_required_ports(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_required_udp_ports(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_timeout(arg1: *const nvti_t) -> gint;
    pub fn nvti_category(arg1: *const nvti_t) -> gint;
    pub fn nvti_family(arg1: *const nvti_t) -> *mut gchar;
    pub fn nvti_pref_len(arg1: *const nvti_t) -> guint;
    pub fn nvti_pref(arg1: *const nvti_t, arg2: guint) -> *const nvtpref_t;
    pub fn nvti_set_oid(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_name(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_cve(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_bid(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_xref(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_tag(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_cvss_base(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_dependencies(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_required_keys(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_mandatory_keys(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_excluded_keys(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_required_ports(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_set_required_udp_ports(
        arg1: *mut nvti_t,
        arg2: *const gchar,
    ) -> c_int;
    pub fn nvti_set_timeout(arg1: *mut nvti_t, arg2: gint) -> c_int;
    pub fn nvti_set_category(arg1: *mut nvti_t, arg2: gint) -> c_int;
    pub fn nvti_set_family(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_cve(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_bid(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_required_keys(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_mandatory_keys(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_excluded_keys(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_required_ports(arg1: *mut nvti_t, arg2: *const gchar) -> c_int;
    pub fn nvti_add_required_udp_ports(
        arg1: *mut nvti_t,
        arg2: *const gchar,
    ) -> c_int;
    pub fn nvti_add_pref(arg1: *mut nvti_t, arg2: *mut nvtpref_t) -> c_int;
    pub fn nvtis_new() -> *mut nvtis_t;
    pub fn nvtis_lookup(arg1: *mut nvtis_t, arg2: *const c_char) -> *mut nvti_t;
    pub fn nvtis_free(arg1: *mut nvtis_t);
    pub fn nvtis_add(arg1: *mut nvtis_t, arg2: *mut nvti_t);
}