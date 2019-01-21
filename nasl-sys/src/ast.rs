use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::{ gchar, gint, guint, };
use crate::kb::kb_t;
use crate::nvti::nvti_t;
use crate::libc::{ c_void, c_char, c_uchar, c_int, c_uint, c_short, c_long, in6_addr, in_addr, };
use crate::glib_sys::{ GHashTable, GList, GSList, };


pub type node_type = c_uint;

pub const NODE_EMPTY: node_type = 0;
pub const NODE_IF_ELSE: node_type = 1;
pub const NODE_INSTR_L: node_type = 2;
pub const NODE_FOR: node_type = 3;
pub const NODE_WHILE: node_type = 4;
pub const NODE_FOREACH: node_type = 5;
pub const NODE_REPEAT_UNTIL: node_type = 6;
pub const NODE_REPEATED: node_type = 7;
pub const NODE_FUN_DEF: node_type = 8;
pub const NODE_FUN_CALL: node_type = 9;
pub const NODE_DECL: node_type = 10;
pub const NODE_ARG: node_type = 11;
pub const NODE_RETURN: node_type = 12;
pub const NODE_BREAK: node_type = 13;
pub const NODE_CONTINUE: node_type = 14;
pub const NODE_ARRAY_EL: node_type = 15;
pub const NODE_AFF: node_type = 16;
pub const NODE_VAR: node_type = 17;
pub const NODE_LOCAL: node_type = 18;
pub const NODE_GLOBAL: node_type = 19;
pub const NODE_PLUS_EQ: node_type = 20;
pub const NODE_MINUS_EQ: node_type = 21;
pub const NODE_MULT_EQ: node_type = 22;
pub const NODE_DIV_EQ: node_type = 23;
pub const NODE_MODULO_EQ: node_type = 24;
pub const NODE_L_SHIFT_EQ: node_type = 25;
pub const NODE_R_SHIFT_EQ: node_type = 26;
pub const NODE_R_USHIFT_EQ: node_type = 27;
pub const EXPR_AND: node_type = 28;
pub const EXPR_OR: node_type = 29;
pub const EXPR_NOT: node_type = 30;
pub const EXPR_PLUS: node_type = 31;
pub const EXPR_MINUS: node_type = 32;
pub const EXPR_U_MINUS: node_type = 33;
pub const EXPR_MULT: node_type = 34;
pub const EXPR_DIV: node_type = 35;
pub const EXPR_MODULO: node_type = 36;
pub const EXPR_EXPO: node_type = 37;
pub const EXPR_BIT_AND: node_type = 38;
pub const EXPR_BIT_OR: node_type = 39;
pub const EXPR_BIT_XOR: node_type = 40;
pub const EXPR_BIT_NOT: node_type = 41;
pub const EXPR_INCR: node_type = 42;
pub const EXPR_DECR: node_type = 43;
pub const EXPR_L_SHIFT: node_type = 44;
pub const EXPR_R_SHIFT: node_type = 45;
pub const EXPR_R_USHIFT: node_type = 46;
pub const COMP_MATCH: node_type = 47;
pub const COMP_NOMATCH: node_type = 48;
pub const COMP_RE_MATCH: node_type = 49;
pub const COMP_RE_NOMATCH: node_type = 50;
pub const COMP_LT: node_type = 51;
pub const COMP_LE: node_type = 52;
pub const COMP_EQ: node_type = 53;
pub const COMP_NE: node_type = 54;
pub const COMP_GT: node_type = 55;
pub const COMP_GE: node_type = 56;
pub const CONST_INT: node_type = 57;
pub const CONST_STR: node_type = 58;
pub const CONST_DATA: node_type = 59;
pub const CONST_REGEX: node_type = 60;
pub const ARRAY_ELEM: node_type = 61;
pub const REF_VAR: node_type = 62;
pub const REF_ARRAY: node_type = 63;
pub const DYN_ARRAY: node_type = 64;


// exec_nasl_script modes
pub const NASL_EXEC_DESCR: c_int      = 1 << 0;
pub const NASL_EXEC_PARSE_ONLY: c_int = 1 << 1;
pub const NASL_ALWAYS_SIGNED: c_int   = 1 << 2;
pub const NASL_COMMAND_LINE: c_int    = 1 << 3;
pub const NASL_LINT: c_int            = 1 << 4;

pub const NASL_ERR_NOERR: c_int      = 0;
pub const NASL_ERR_ETIMEDOUT: c_int  = 1;
pub const NASL_ERR_ECONNRESET: c_int = 2;
pub const NASL_ERR_EUNREACH: c_int   = 3;
pub const NASL_ERR_EUNKNOWN: c_int   = 99;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct TC {
    pub type_: c_short,
    pub line_nb: c_short,
    pub ref_count: c_short,
    pub size: c_int,
    pub x: TC__bindgen_ty_1,
    pub link: [*mut TC; 4usize],
}
pub type tree_cell = TC;


#[repr(C)]
#[derive(Copy, Clone)]
pub union TC__bindgen_ty_1 {
    pub str_val: *mut c_char,
    pub i_val: c_long,
    pub ref_val: *mut c_void,
    _bindgen_union_align: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_nasl_string {
    pub s_val: *mut c_uchar,
    pub s_siz: c_int,
}

pub type nasl_string_t = st_nasl_string;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_nasl_array {
    pub max_idx: c_int,
    pub num_elt: *mut *mut st_a_nasl_var,
    pub hash_elt: *mut *mut st_n_nasl_var,
}

pub type nasl_array = st_nasl_array;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct struct_lex_ctxt {
    pub up_ctxt: *mut struct_lex_ctxt,
    pub ret_val: *mut tree_cell,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub script_infos: *mut script_infos,
    pub oid: *const c_char,
    pub recv_timeout: c_int,
    pub line_nb: c_int,
    pub ctx_vars: nasl_array,
    pub functions: *mut GHashTable,
}

impl struct_lex_ctxt {
    #[inline]
    pub fn fct_ctxt(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_fct_ctxt(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn break_flag(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_break_flag(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn cont_flag(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_cont_flag(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn always_signed(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_always_signed(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        fct_ctxt: c_uint,
        break_flag: c_uint,
        cont_flag: c_uint,
        always_signed: c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let fct_ctxt: u32 = unsafe { ::std::mem::transmute(fct_ctxt) };
            fct_ctxt as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let break_flag: u32 = unsafe { ::std::mem::transmute(break_flag) };
            break_flag as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let cont_flag: u32 = unsafe { ::std::mem::transmute(cont_flag) };
            cont_flag as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let always_signed: u32 = unsafe { ::std::mem::transmute(always_signed) };
            always_signed as u64
        });
        __bindgen_bitfield_unit
    }
}

pub type lex_ctxt = struct_lex_ctxt;


pub type _bindgen_ty_2 = c_uint;

pub const VAR2_UNDEF: _bindgen_ty_2 = 0;
pub const VAR2_INT: _bindgen_ty_2 = 1;
pub const VAR2_STRING: _bindgen_ty_2 = 2;
pub const VAR2_DATA: _bindgen_ty_2 = 3;
pub const VAR2_ARRAY: _bindgen_ty_2 = 4;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_a_nasl_var {
    pub var_type: c_int,
    pub v: st_a_nasl_var__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union st_a_nasl_var__bindgen_ty_1 {
    pub v_str: nasl_string_t,
    pub v_int: c_long,
    pub v_arr: nasl_array,
    _bindgen_union_align: [u64; 3usize],
}

pub type anon_nasl_var = st_a_nasl_var;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_n_nasl_var {
    pub u: st_a_nasl_var,
    pub var_name: *mut c_char,
    pub next_var: *mut st_n_nasl_var,
}

pub type named_nasl_var = st_n_nasl_var;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nasl_iterator {
    pub a: *mut nasl_array,
    pub i1: c_int,
    pub iH: c_int,
    pub v: *mut named_nasl_var,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_nasl_func {
    pub func_name: *mut c_char,
    pub block: *mut c_void,
}

pub type nasl_func = st_nasl_func;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct naslctxt {
    pub line_nb: c_int,
    pub always_signed: c_int,
    pub index: c_int,
    pub tree: *mut tree_cell,
    pub buffer: *mut c_char,
    pub kb: kb_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct scan_globals {
    pub network_targets: *mut c_char,
    pub network_scan_status: *mut c_char,
    pub files_translation: *mut GHashTable,
    pub files_size_translation: *mut GHashTable,
    pub global_socket: c_int,
    pub scan_id: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct host_info {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct script_infos {
    pub globals: *mut scan_globals,
    pub key: kb_t,
    pub nvti: *mut nvti_t,
    pub oid: *mut c_char,
    pub name: *mut c_char,
    pub udp_data: *mut GHashTable,
    pub ip: *mut in6_addr,
    pub vhosts: *mut GSList,
    pub standalone: c_int,
    pub denial_port: c_int,
    pub alive: c_int,
}



#[link(name = "nasl", kind = "dylib")]
extern {
    pub fn init_nasl_library(arg1: *mut lex_ctxt);
    pub fn add_nasl_library(arg1: *mut *mut GSList);
    pub fn nasl_verify_signature(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: usize,
    ) -> c_int;
    pub fn nasl_is_leaf(arg1: *const tree_cell) -> c_int;
    pub fn get_line_nb(arg1: *const tree_cell) -> *mut c_char;
    pub fn nasl_dump_tree(arg1: *const tree_cell);
    pub fn ref_cell(arg1: *mut tree_cell);
    pub fn deref_cell(arg1: *mut tree_cell);
    pub fn nasl_type_name(arg1: c_int) -> *const c_char;
    pub fn cell_type(arg1: *const tree_cell) -> c_int;
    pub fn dump_cell_val(arg1: *const tree_cell) -> *mut c_char;
    pub fn alloc_tree_cell() -> *mut tree_cell;
    pub fn alloc_expr_cell(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut tree_cell,
        arg4: *mut tree_cell,
    ) -> *mut tree_cell;
    pub fn alloc_RE_cell(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut tree_cell,
        arg4: *mut c_char,
    ) -> *mut tree_cell;
    pub fn alloc_typed_cell(arg1: c_int) -> *mut tree_cell;
    pub fn dup_cell(arg1: *const tree_cell) -> *mut tree_cell;
    pub fn nasl_affect(arg1: *mut tree_cell, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn clear_unnamed_var(arg1: *mut anon_nasl_var);
    pub fn var2str(arg1: *const anon_nasl_var) -> *const c_char;
    pub fn nasl_get_var_by_num(
        arg1: *mut c_void,
        arg2: *mut nasl_array,
        arg3: c_int,
        arg4: c_int,
    ) -> *mut anon_nasl_var;
    pub fn nasl_array_iterator(
        arg1: *mut c_void,
        arg2: *mut tree_cell,
    ) -> nasl_iterator;
    pub fn nasl_iterate_array(arg1: *mut nasl_iterator) -> *mut tree_cell;
    pub fn add_var_to_list(
        arg1: *mut nasl_array,
        arg2: c_int,
        arg3: *const anon_nasl_var,
    ) -> c_int;
    pub fn add_var_to_array(
        arg1: *mut nasl_array,
        arg2: *mut c_char,
        arg3: *const anon_nasl_var,
    ) -> c_int;
    pub fn array_max_index(arg1: *mut nasl_array) -> c_int;
    pub fn free_array(arg1: *mut nasl_array);
    pub fn copy_ref_array(arg1: *const tree_cell) -> *mut tree_cell;
    pub fn hash_str2(
        arg1: *const c_char,
        arg2: c_int,
    ) -> c_int;
    pub fn var2cell(arg1: *mut anon_nasl_var) -> *mut tree_cell;
    pub fn make_array_from_elems(arg1: *mut tree_cell) -> *mut tree_cell;
    pub fn array2str(arg1: *const nasl_array) -> *mut c_char;
    pub fn func_is_internal(arg1: *const c_char) -> *mut nasl_func;
    pub fn free_func(arg1: *mut nasl_func);
    pub fn init_empty_lex_ctxt() -> *mut lex_ctxt;
    pub fn free_lex_ctxt(arg1: *mut lex_ctxt);
    pub fn dump_ctxt(arg1: *mut lex_ctxt);
    pub fn get_func_ref_by_name(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
    ) -> *mut nasl_func;
    pub fn decl_nasl_func(
        arg1: *mut lex_ctxt,
        arg2: *mut tree_cell,
        arg3: c_int,
    ) -> *mut tree_cell;
    pub fn insert_nasl_func(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
        arg3: *mut tree_cell,
        arg4: c_int,
    ) -> *mut nasl_func;
    pub fn nasl_func_call(
        arg1: *mut lex_ctxt,
        arg2: *const nasl_func,
        arg3: *mut tree_cell,
    ) -> *mut tree_cell;
    pub fn get_variable_by_name(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
    ) -> *mut tree_cell;
    pub fn get_array_elem(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
        arg3: *mut tree_cell,
    ) -> *mut tree_cell;
    pub fn add_numbered_var_to_ctxt(
        arg1: *mut lex_ctxt,
        arg2: c_int,
        arg3: *mut tree_cell,
    ) -> *mut anon_nasl_var;
    pub fn add_named_var_to_ctxt(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
        arg3: *mut tree_cell,
    ) -> *mut named_nasl_var;
    pub fn nasl_read_var_ref(arg1: *mut lex_ctxt, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn nasl_incr_variable(
        arg1: *mut lex_ctxt,
        arg2: *mut tree_cell,
        arg3: c_int,
        arg4: c_int,
    ) -> *mut tree_cell;
    pub fn nasl_return(arg1: *mut lex_ctxt, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn decl_local_variables(arg1: *mut lex_ctxt, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn decl_global_variables(arg1: *mut lex_ctxt, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn cell2atom(arg1: *mut lex_ctxt, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn get_int_var_by_num(
        arg1: *mut lex_ctxt,
        arg2: c_int,
        arg3: c_int,
    ) -> c_long;
    pub fn get_str_var_by_num(
        arg1: *mut lex_ctxt,
        arg2: c_int,
    ) -> *mut c_char;
    pub fn get_int_var_by_name(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_long;
    pub fn get_str_var_by_name(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
    ) -> *mut c_char;
    pub fn get_var_size_by_name(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
    ) -> c_int;
    pub fn get_var_type_by_name(
        arg1: *mut lex_ctxt,
        arg2: *const c_char,
    ) -> c_int;
    pub fn get_var_size_by_num(
        arg1: *mut lex_ctxt,
        arg2: c_int,
    ) -> c_int;
    pub fn get_var_type_by_num(
        arg1: *mut lex_ctxt,
        arg2: c_int,
    ) -> c_int;
    pub fn nasl_perror(arg1: *mut lex_ctxt, arg2: *mut c_char, ...);
    pub fn nasl_trace(arg1: *mut lex_ctxt, arg2: *mut c_char, ...);
    pub fn nasl_trace_enabled() -> c_int;
    pub fn nasl_set_filename(arg1: *const c_char);
    pub fn nasl_set_function_filename(arg1: *const c_char);
    pub fn nasl_get_filename(arg1: *const c_char) -> *const c_char;
    pub fn nasl_set_function_name(arg1: *const c_char);
    pub fn nasl_get_function_name() -> *const c_char;
    pub fn plugin_run_find_service(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn plugin_run_openvas_tcp_scanner(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn plugin_run_synscan(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn plugin_run_nmap(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_exec(arg1: *mut lex_ctxt, arg2: *mut tree_cell) -> *mut tree_cell;
    pub fn cell_cmp(
        arg1: *mut lex_ctxt,
        arg2: *mut tree_cell,
        arg3: *mut tree_cell,
    ) -> c_long;
    pub fn init_nasl_ctx(
        arg1: *mut naslctxt,
        arg2: *const c_char,
    ) -> c_int;
    pub fn nasl_clean_ctx(arg1: *mut naslctxt);
    pub fn script_timeout(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_oid(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_cve_id(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_bugtraq_id(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_xref(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_tag(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_name(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_version(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_copyright(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_category(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_family(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_dependencies(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_require_keys(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_mandatory_keys(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_exclude_keys(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_require_ports(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_require_udp_ports(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_preference(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_add_preference(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_get_preference(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_get_preference_file_content(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn script_get_preference_file_location(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn safe_checks(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn scan_phase(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn network_targets(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_script_oid(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_kb_item(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_kb_list(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_kb_item(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn replace_kb_item(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn security_message(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn log_message(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn error_message(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_scanner_get_port(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_scanner_add_port(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_scanner_status(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_vendor_version(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn generate_script_signature(arg1: *mut c_char) -> c_int;
    pub fn nasl_cert_open(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_cert_close(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_cert_query(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_md2(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_md4(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_md5(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_sha(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_sha1(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_sha256(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ripemd160(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_md2(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_md5(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_sha1(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_sha256(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_sha384(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_sha512(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_dss(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hmac_ripemd160(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_prf_sha256(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_prf_sha384(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_tls1_prf(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ntlmv1_hash(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_nt_owf_gen(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_lm_owf_gen(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ntv2_owf_gen(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ntlmv2_hash(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ntlmv2_response(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ntlm2_response(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ntlm_response(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_keyexchg(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_insert_hexzeros(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_password(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_sign(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_smb2_sign(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_cipher_des(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_bn_random(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_dh_generate_key(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_bn_cmp(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_dh_compute_key(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rsa_public_encrypt(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rsa_private_decrypt(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rsa_public_decrypt(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_bf_cbc_encrypt(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_bf_cbc_decrypt(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_dsa_do_verify(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_pem_to_rsa(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_pem_to_dsa(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rsa_sign(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_dsa_do_sign(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rc4_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_aes128_cbc_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_aes256_cbc_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_aes128_ctr_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_aes256_ctr_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_des_ede_cbc_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_aes128_gcm_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_aes256_gcm_encrypt(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_isotime_now(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_isotime_is_valid(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_isotime_scan(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_isotime_print(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_isotime_add(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rand(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_usleep(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_sleep(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ftp_log_in(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ftp_get_pasv_address(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_telnet_init(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_start_denial(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_end_denial(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_dump_ctxt(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_do_exit(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_isnull(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_make_list(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_make_array(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_keys(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_max_index(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_typeof(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_defined_func(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_sort_array(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_unixtime(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_gettimeofday(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_localtime(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_mktime(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_open_sock_kdc(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_dec2str(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_byte_order(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_gunzip(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_gzip(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_pread(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_find_in_path(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_fread(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_fwrite(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_unlink(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_tmp_dir(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_file_stat(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_file_open(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_file_close(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_file_read(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_file_write(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_file_seek(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_snmpv1_get(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_snmpv2c_get(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_snmpv3_get(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_connect(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_disconnect(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_session_id_from_sock(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_get_sock(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_set_login(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_userauth(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_request_exec(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_shell_open(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_shell_read(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_shell_write(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_shell_close(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_login_interactive(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_login_interactive_pass(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_exec(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_get_issue_banner(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_get_server_banner(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_get_auth_methods(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ssh_get_host_key(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_versioninfo(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_connect(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_close(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_query(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_connect_rsop(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_query_rsop(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_connect_reg(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_get_sz(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_enum_value(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_enum_key(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_get_bin_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_get_dword_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_get_ex_string_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_get_mul_string_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_get_qword_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_set_dword_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_set_qword_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_set_ex_string_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_set_string_val(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_create_key(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_wmi_reg_delete_key(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_versioninfo(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_connect(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_close(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_file_SDDL(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_file_owner_sid(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_file_group_sid(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_smb_file_trustee_rights(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_win_cmd_exec(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_open_socket(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_close_socket(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_get(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_head(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_post(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_delete(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn http_put(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_http_recv_headers(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn cgibin(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_is_cgi_installed(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_http_keepalive_send_recv(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_http_share_exists(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_ip_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_ip_elements(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_ip_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn dump_ip_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn insert_ip_options(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_tcp_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_tcp_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_tcp_elements(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn dump_tcp_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_tcp_ping(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_udp_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_udp_elements(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn dump_udp_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_udp_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_icmp_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_icmp_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_igmp_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_send_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_pcap_next(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_send_capture(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_ipv6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_ipv6_elements(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_ipv6_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn dump_ipv6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn insert_ipv6_options(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_tcp_v6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_tcp_v6_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_tcp_v6_elements(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn dump_tcp_v6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_udp_v6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn set_udp_v6_elements(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn dump_udp_v6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_udp_v6_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_icmp_v6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_icmp_v6_element(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn forge_igmp_v6_packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_tcp_v6_ping(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_send_v6packet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn add_hostname(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_hostname(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_hostnames(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_hostname_source(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn resolve_hostname(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_host_ip(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_host_open_port(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_port_state(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_udp_port_state(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_islocalhost(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_islocalnet(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_this_host(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_this_host_name(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn get_port_transport(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_same_host(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_target_is_ipv6(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_open_sock_tcp(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_open_sock_udp(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_open_sock_tcp_bufsz(
        arg1: *mut lex_ctxt,
        arg2: c_int,
    ) -> *mut tree_cell;
    pub fn nasl_socket_get_error(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_open_priv_sock_tcp(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_open_priv_sock_udp(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_send(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_socket_negotiate_ssl(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_recv(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_recv_line(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_socket_get_cert(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_socket_get_ssl_session_id(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_socket_get_ssl_version(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_socket_get_ssl_ciphersuite(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_socket_cert_verify(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_close_socket(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_join_multicast_group(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_leave_multicast_group(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_source_port(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_get_sock_info(lexic: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_string(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_rawstring(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_strlen(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_strcat(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_display(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hex(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_hexstr(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ord(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_tolower(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_toupper(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ereg(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_eregmatch(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_ereg_replace(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_egrep(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_match(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_split(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_chomp(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_substr(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_insstr(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_strstr(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_crap(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_int(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_stridx(arg1: *mut lex_ctxt) -> *mut tree_cell;
    pub fn nasl_str_replace(arg1: *mut lex_ctxt) -> *mut tree_cell;

    // NOTE: 补漏
    pub fn nasl_version() -> *const std::os::raw::c_char;
    pub fn add_nasl_inc_dir (include_dir: *const c_char) -> c_int;
    pub fn nasl_clean_inc();
    pub fn parse_script_infos(infos: *mut script_infos) -> *mut nvti_t;
    pub fn exec_nasl_script(infos: *mut script_infos, mode: c_int) -> c_int;

    pub fn gcrypt_init();
    pub fn openvas_SSL_init() -> c_int;

    pub fn init(ip: *mut in6_addr, vhosts: *mut GSList, kb: kb_t) -> *mut script_infos;
    pub fn nvti_category_is_safe(category: c_int);

    pub fn gvm_hosts_new(hosts_str: *const gchar) -> *mut gvm_hosts_t;
    pub fn gvm_hosts_resolve(hosts: *mut gvm_hosts_t);
    pub fn gvm_hosts_next(hosts: *mut gvm_hosts_t) -> *const gvm_host_t;
    pub fn gvm_host_get_addr6(host: *const gvm_host_t, ip6: *mut in6_addr);
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum host_type {
    HOST_TYPE_NAME = 0,       /* Hostname eg. foo */
    HOST_TYPE_IPV4,           /* eg. 192.168.1.1 */
    HOST_TYPE_CIDR_BLOCK,     /* eg. 192.168.15.0/24 */
    HOST_TYPE_RANGE_SHORT,    /* eg. 192.168.15.10-20 */
    HOST_TYPE_RANGE_LONG,     /* eg. 192.168.15.10-192.168.18.3 */
    HOST_TYPE_IPV6,           /* eg. ::1 */
    HOST_TYPE_CIDR6_BLOCK,    /* eg. ::ffee/120 */
    HOST_TYPE_RANGE6_LONG,    /* eg. ::1:200:7-::1:205:500 */
    HOST_TYPE_RANGE6_SHORT,   /* eg. ::1-fe10 */
    HOST_TYPE_MAX             /* Boundary checking. */
}

/// @brief The structure for a single host object.
/// 
/// The elements of this structure should never be accessed directly.
/// Only the functions corresponding to this module should be used.
#[repr(C)]
pub struct gvm_host {
    pub unamed_u: gvm_host_u,
    pub type_: host_type,
    pub vhosts: *mut GSList,
}

#[repr(C)]
pub union gvm_host_u {
    pub name: *mut gchar,
    pub addr: in_addr,
    pub addr6: in6_addr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gvm_hosts {
    /// Original hosts definition string.
    pub orig_str: gchar,
    /// Hosts objects list.
    pub hosts: *mut GList,
    /// Current host object in iteration.
    pub current: *mut GList,
    /// Number of single host objects in hosts list.
    pub count: c_uint,
    /// Number of duplicate/excluded values.
    pub removed: c_uint,
}

pub type gvm_hosts_t = gvm_hosts;
pub type gvm_host_t  = gvm_host;
