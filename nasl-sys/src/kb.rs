use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::{ gchar, gint, guint, };
use crate::nvti::nvti_t;
use crate::libc::{ c_int, c_char, c_uint, };
use crate::glib_sys::{ GHashTable, GSList, };


pub const KB_PATH_DEFAULT: &str = "/tmp/redis.sock";

pub type kb_item_type = c_uint;

pub const KB_TYPE_UNSPEC: kb_item_type = 0;
pub const KB_TYPE_INT: kb_item_type = 1;
pub const KB_TYPE_STR: kb_item_type = 2;
pub const KB_TYPE_CNT: kb_item_type = 3;


pub type kb_nvt_pos = c_uint;

pub const NVT_FILENAME_POS: kb_nvt_pos = 0;
pub const NVT_REQUIRED_KEYS_POS: kb_nvt_pos = 1;
pub const NVT_MANDATORY_KEYS_POS: kb_nvt_pos = 2;
pub const NVT_EXCLUDED_KEYS_POS: kb_nvt_pos = 3;
pub const NVT_REQUIRED_UDP_PORTS_POS: kb_nvt_pos = 4;
pub const NVT_REQUIRED_PORTS_POS: kb_nvt_pos = 5;
pub const NVT_DEPENDENCIES_POS: kb_nvt_pos = 6;
pub const NVT_TAGS_POS: kb_nvt_pos = 7;
pub const NVT_CVES_POS: kb_nvt_pos = 8;
pub const NVT_BIDS_POS: kb_nvt_pos = 9;
pub const NVT_XREFS_POS: kb_nvt_pos = 10;
pub const NVT_CATEGORY_POS: kb_nvt_pos = 11;
pub const NVT_TIMEOUT_POS: kb_nvt_pos = 12;
pub const NVT_FAMILY_POS: kb_nvt_pos = 13;
pub const NVT_NAME_POS: kb_nvt_pos = 14;
pub const NVT_TIMESTAMP_POS: kb_nvt_pos = 15;
pub const NVT_OID_POS: kb_nvt_pos = 16;


/// @brief Knowledge base item (defined by name, type (int/char*) and value).
/// Implemented as a singly linked list
#[repr(C)]
pub struct kb_item {
    /// < One of KB_TYPE_INT or KB_TYPE_STR.
    pub type_: kb_item_type,
    pub __bindgen_anon_1: kb_item__bindgen_ty_1,
    /// < Length of string.
    pub len: usize,
    /// < Next item in list.
    pub next: *mut kb_item,
    /// < Name length (including final NULL byte).
    pub namelen: usize,
    /// < Name of this knowledge base item.
    pub name: __IncompleteArrayField<c_char>,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub union kb_item__bindgen_ty_1 {
    pub v_str: *mut c_char,
    pub v_int: c_int,
    _bindgen_union_align: u64,
}


/// @brief Top-level KB. This is to be inherited by KB implementations.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kb {
    /// < KB vtable.
    pub kb_ops: *const kb_operations,
}


/// @brief type abstraction to hide KB internals.
pub type kb_t = *mut kb;
/// @brief KB interface. Functions provided by an implementation. All functions
/// have to be provided, there is no default/fallback. These functions
/// should be called via the corresponding static inline wrappers below.
/// See the wrappers for the documentation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kb_operations {
    pub kb_new: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut kb_t,
            arg2: *const c_char,
        ) -> c_int,
    >,
    pub kb_delete: ::std::option::Option<unsafe extern "C" fn(arg1: kb_t) -> c_int>,
    pub kb_find: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
        ) -> kb_t,
    >,
    pub kb_direct_conn: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: c_int,
        ) -> kb_t,
    >,
    pub kb_get_single: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: kb_item_type,
        ) -> *mut kb_item,
    >,
    pub kb_get_str: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
        ) -> *mut c_char,
    >,
    pub kb_get_int: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
        ) -> c_int,
    >,
    pub kb_get_nvt: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: kb_nvt_pos,
        ) -> *mut c_char,
    >,
    pub kb_get_nvt_all: ::std::option::Option<
        unsafe extern "C" fn(arg1: kb_t, arg2: *const c_char) -> *mut nvti_t,
    >,
    /// < Get list of OIDs.
    pub kb_get_nvt_oids: ::std::option::Option<unsafe extern "C" fn(arg1: kb_t) -> *mut GSList>,
    pub kb_push_str: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: *const c_char,
        ) -> c_int,
    >,
    pub kb_pop_str: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
        ) -> *mut c_char,
    >,
    pub kb_get_all: ::std::option::Option<
        unsafe extern "C" fn(arg1: kb_t, arg2: *const c_char) -> *mut kb_item,
    >,
    pub kb_get_pattern: ::std::option::Option<
        unsafe extern "C" fn(arg1: kb_t, arg2: *const c_char) -> *mut kb_item,
    >,
    pub kb_count: ::std::option::Option<
        unsafe extern "C" fn(arg1: kb_t, arg2: *const c_char) -> usize,
    >,
    pub kb_add_str: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: usize,
        ) -> c_int,
    >,
    pub kb_add_str_unique: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: usize,
        ) -> c_int,
    >,
    pub kb_set_str: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: usize,
        ) -> c_int,
    >,
    pub kb_add_int: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: c_int,
        ) -> c_int,
    >,
    pub kb_add_int_unique: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: c_int,
        ) -> c_int,
    >,
    pub kb_set_int: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
            arg3: c_int,
        ) -> c_int,
    >,
    pub kb_add_nvt: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const nvti_t,
            arg3: *const c_char,
        ) -> c_int,
    >,
    pub kb_del_items: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
        ) -> c_int,
    >,
    pub kb_save: ::std::option::Option<unsafe extern "C" fn(arg1: kb_t) -> c_int>,
    pub kb_lnk_reset:
        ::std::option::Option<unsafe extern "C" fn(arg1: kb_t) -> c_int>,
    pub kb_flush: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: kb_t,
            arg2: *const c_char,
        ) -> c_int,
    >,
    pub kb_get_kb_index:
        ::std::option::Option<unsafe extern "C" fn(arg1: kb_t) -> c_int>,
}


#[link(name = "nasl", kind = "dylib")]
extern {

    pub static mut KBDefaultOperations: *const kb_operations;

    /// @brief Release a KB item (or a list).
    pub fn kb_item_free(arg1: *mut kb_item);
}