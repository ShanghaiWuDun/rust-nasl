#include <glib.h>
#include "kb.h"

#include "builtin/crypto.h"
#include "builtin/datetime.h"
#include "builtin/misc.h"
#include "builtin/network.h"
#include "builtin/text_utils.h"


enum node_type {
    NODE_EMPTY = 0,
    NODE_IF_ELSE,                 /* [0] = cond, [1] = if_block, [2] = else_block */
    NODE_INSTR_L,                 /* Block. [0] = first instr, [1] = tail */
    NODE_FOR,                     /* [0] = start expr, [1] = cond, [2] = end_expr, [3] = block */
    NODE_WHILE,                   /* [0] = cond, [1] = block */
    NODE_FOREACH,
    NODE_REPEAT_UNTIL,
    NODE_REPEATED,                /* [0] = func call, [1] = repeat nb */
    NODE_FUN_DEF,                 /* [0] = argdecl, [1] = block */
    NODE_FUN_CALL,                /* [0] = script_infos */
    NODE_DECL,                    /* [0] = next arg in list */
    NODE_ARG,                     /* val = name can be NULL, [0] = val, [1] = next arg */
    NODE_RETURN,                  /* ret val */
    NODE_BREAK,
    NODE_CONTINUE,

    NODE_ARRAY_EL,                /* val = array name, [0] = index */
    NODE_AFF,                     /* [0] = lvalue, [1] = rvalue */
    NODE_VAR,                     /* val = variable name */
    NODE_LOCAL,                   /* [0] = argdecl */
    NODE_GLOBAL,

    NODE_PLUS_EQ,
    NODE_MINUS_EQ,
    NODE_MULT_EQ,
    NODE_DIV_EQ,
    NODE_MODULO_EQ,

    NODE_L_SHIFT_EQ,
    NODE_R_SHIFT_EQ,
    NODE_R_USHIFT_EQ,

    EXPR_AND,
    EXPR_OR,
    EXPR_NOT,

    EXPR_PLUS,
    EXPR_MINUS,
    EXPR_U_MINUS,
    EXPR_MULT,
    EXPR_DIV,
    EXPR_MODULO,
    EXPR_EXPO,

    EXPR_BIT_AND,
    EXPR_BIT_OR,
    EXPR_BIT_XOR,
    EXPR_BIT_NOT,
    EXPR_INCR,
    EXPR_DECR,
    EXPR_L_SHIFT,
    EXPR_R_SHIFT,
    EXPR_R_USHIFT,

    COMP_MATCH,
    COMP_NOMATCH,
    COMP_RE_MATCH,
    COMP_RE_NOMATCH,

    COMP_LT,
    COMP_LE,
    COMP_EQ,
    COMP_NE,
    COMP_GT,
    COMP_GE,

    CONST_INT,
    CONST_STR,                    /* "impure" string */

    CONST_DATA,                   /* binary data / "pure" string */
    CONST_REGEX,                  /* Compiled regex */

    ARRAY_ELEM,                   /* val = char index or NULL if integer,
                                 * [0] = value, [1] = next element */
    /* For exec only */
    REF_VAR,
    REF_ARRAY,
    DYN_ARRAY
};

typedef struct TC {
    short type;
    short line_nb;
    short ref_count;              /* Cell is freed when count reaches zero */
    int size;
    union {
        char *str_val;
        long int i_val;
        void *ref_val;            /* internal reference */
    } x;
    struct TC *link[4];
} tree_cell;


#define FAKE_CELL ((void*)1)
#define EXIT_CELL ((void*)2)

void init_nasl_library (lex_ctxt *);
void add_nasl_library (GSList **);
int nasl_verify_signature (const char *, const char *, size_t);

int nasl_is_leaf (const tree_cell *);
char *get_line_nb (const tree_cell *);
void nasl_dump_tree (const tree_cell *);
void ref_cell (tree_cell *);
void deref_cell (tree_cell *);
const char *nasl_type_name (int);
int cell_type (const tree_cell *);
char *dump_cell_val (const tree_cell *);


tree_cell *alloc_tree_cell (void);
tree_cell *alloc_expr_cell (int, int, tree_cell *, tree_cell *);
tree_cell *alloc_RE_cell (int, int, tree_cell *, char *);
tree_cell *alloc_typed_cell (int);
tree_cell *dup_cell (const tree_cell *);


enum {
    VAR2_UNDEF = 0,
    VAR2_INT,
    VAR2_STRING,
    VAR2_DATA,
    VAR2_ARRAY
};

#define VAR_NAME_HASH 17

typedef struct st_nasl_string {
    unsigned char *s_val;
    int s_siz;
} nasl_string_t;

struct st_a_nasl_var;

typedef struct st_nasl_array {
    int max_idx;                  /* max index - 1! */
    struct st_a_nasl_var **num_elt;       /* max_idx elements */
    struct st_n_nasl_var **hash_elt;      /* VAR_NAME_HASH elements */
} nasl_array;


#if NASL_DEBUG > 0
#define ALL_VARIABLES_NAMED
#endif

typedef struct st_a_nasl_var {
    int var_type;
    #ifdef ALL_VARIABLES_NAMED
    char *av_name;
    #endif
    union {
        nasl_string_t v_str;        /* character string / data */
        long int v_int;             /* integer */
        nasl_array v_arr;           /* array */
    } v;
} anon_nasl_var;

typedef struct st_n_nasl_var {
    struct st_a_nasl_var u;
    #ifndef ALL_VARIABLES_NAMED
    char *var_name;
    #else
    #define var_name  u.av_name
    #endif
    struct st_n_nasl_var *next_var;       /* next variable with same name hash */
} named_nasl_var;

typedef struct {
    nasl_array *a;                /* array */
    int i1;                       /* index of numbered elements */
    int iH;                       /* index of hash */
    named_nasl_var *v;            /* current variable in hash */
} nasl_iterator;

tree_cell *nasl_affect (tree_cell *, tree_cell *);

void clear_unnamed_var (anon_nasl_var *);
const char *var2str (const anon_nasl_var *);

anon_nasl_var *nasl_get_var_by_num (void *, nasl_array *, int, int);

nasl_iterator nasl_array_iterator (void *, tree_cell *);
tree_cell *nasl_iterate_array (nasl_iterator *);
int add_var_to_list (nasl_array *, int, const anon_nasl_var *);
int add_var_to_array (nasl_array *, char *, const anon_nasl_var *);
int array_max_index (nasl_array *);
void free_array (nasl_array *);

tree_cell *copy_ref_array (const tree_cell *);
int hash_str2 (const char *, int);
tree_cell *var2cell (anon_nasl_var *);

tree_cell *make_array_from_elems (tree_cell *);
char *array2str (const nasl_array *);



// Type for a built-in nasl function.
typedef struct st_nasl_func {
    char *func_name;
    void *block;                  /* Can be pointer to a C function! */
} nasl_func;

nasl_func *func_is_internal (const char *);

void free_func (nasl_func *);



// Lex
typedef struct struct_lex_ctxt {
    struct struct_lex_ctxt *up_ctxt;
    tree_cell *ret_val;           /* return value or exit flag */
    unsigned fct_ctxt:1;          /* This is a function context */
    unsigned break_flag:1;        /* Break from loop */
    unsigned cont_flag:1;         /* Next iteration in loop */
    unsigned always_signed:1;
    struct script_infos *script_infos;
    const char *oid;
    int recv_timeout;
    int line_nb;
    /* Named variables hash set + anonymous variables array */
    nasl_array ctx_vars;
    /* Functions hash table */
    GHashTable *functions;
} lex_ctxt;

#define NASL_COMPAT_LEX_CTXT    "NASL compat lex context"

lex_ctxt *init_empty_lex_ctxt (void);
void free_lex_ctxt (lex_ctxt *);
void dump_ctxt (lex_ctxt *);

nasl_func *get_func_ref_by_name (lex_ctxt *, const char *);
tree_cell *decl_nasl_func (lex_ctxt *, tree_cell *, int);
nasl_func *insert_nasl_func (lex_ctxt *, const char *, tree_cell *, int);
tree_cell *nasl_func_call (lex_ctxt *, const nasl_func *, tree_cell *);

tree_cell *get_variable_by_name (lex_ctxt *, const char *);
tree_cell *get_array_elem (lex_ctxt *, const char * /*array name */ ,
                           tree_cell *);
anon_nasl_var *add_numbered_var_to_ctxt (lex_ctxt *, int, tree_cell *);
named_nasl_var *add_named_var_to_ctxt (lex_ctxt *, const char *, tree_cell *);
tree_cell *nasl_read_var_ref (lex_ctxt *, tree_cell *);
tree_cell *nasl_incr_variable (lex_ctxt *, tree_cell *, int, int);
tree_cell *nasl_return (lex_ctxt *, tree_cell *);

tree_cell *decl_local_variables (lex_ctxt *, tree_cell *);
tree_cell *decl_global_variables (lex_ctxt *, tree_cell *);

tree_cell *cell2atom (lex_ctxt *, tree_cell *);

long int get_int_var_by_num (lex_ctxt *, int, int);
char *get_str_var_by_num (lex_ctxt *, int);
long int get_int_var_by_name (lex_ctxt *, const char *, int);
char *get_str_var_by_name (lex_ctxt *, const char *);

int get_var_size_by_name (lex_ctxt *, const char *);
int get_var_type_by_name (lex_ctxt *, const char *);

int get_var_size_by_num (lex_ctxt *, int);
int get_var_type_by_num (lex_ctxt *, int);




// Exec
void nasl_perror (lex_ctxt *, char *, ...);
void nasl_trace (lex_ctxt *, char *, ...);
int nasl_trace_enabled (void);
void nasl_set_filename (const char *);
void nasl_set_function_filename (const char *);
const char *nasl_get_filename (const char *);
void nasl_set_function_name (const char *);
const char *nasl_get_function_name (void);


// Plugin (nmap)
tree_cell *plugin_run_find_service (lex_ctxt *);
tree_cell *plugin_run_openvas_tcp_scanner (lex_ctxt *);
tree_cell *plugin_run_synscan (lex_ctxt *);
tree_cell *plugin_run_nmap (lex_ctxt *);


tree_cell *nasl_exec (lex_ctxt *, tree_cell *);
long int cell_cmp (lex_ctxt *, tree_cell *, tree_cell *);
// tree_cell *cell2atom (lex_ctxt *, tree_cell *);



typedef struct {
    int line_nb;
    int always_signed;
    int index;
    tree_cell *tree;
    char *buffer;
    kb_t kb;
} naslctxt;
int init_nasl_ctx (naslctxt *, const char *);
void nasl_clean_ctx (naslctxt *);



tree_cell *script_timeout (lex_ctxt *);
tree_cell *script_oid (lex_ctxt *);
tree_cell *script_cve_id (lex_ctxt *);
tree_cell *script_bugtraq_id (lex_ctxt *);
tree_cell *script_xref (lex_ctxt *);
tree_cell *script_tag (lex_ctxt *);
tree_cell *script_name (lex_ctxt *);
tree_cell *script_version (lex_ctxt *);
tree_cell *script_copyright (lex_ctxt *);
tree_cell *script_category (lex_ctxt *);
tree_cell *script_family (lex_ctxt *);
tree_cell *script_dependencies (lex_ctxt *);
tree_cell *script_require_keys (lex_ctxt *);
tree_cell *script_mandatory_keys (lex_ctxt *);
tree_cell *script_exclude_keys (lex_ctxt *);
tree_cell *script_require_ports (lex_ctxt *);
tree_cell *script_require_udp_ports (lex_ctxt *);
tree_cell *nasl_get_preference (lex_ctxt *);
tree_cell *script_add_preference (lex_ctxt *);
tree_cell *script_get_preference (lex_ctxt *);
tree_cell *script_get_preference_file_content (lex_ctxt *);
tree_cell *script_get_preference_file_location (lex_ctxt *);
tree_cell *safe_checks (lex_ctxt *);
tree_cell *scan_phase (lex_ctxt *);
tree_cell *network_targets (lex_ctxt *);
tree_cell *get_script_oid (lex_ctxt *);
tree_cell *get_kb_item (lex_ctxt *);
tree_cell *get_kb_list (lex_ctxt *);
tree_cell *set_kb_item (lex_ctxt *);
tree_cell *replace_kb_item (lex_ctxt *);
tree_cell *security_message (lex_ctxt *);
tree_cell *log_message (lex_ctxt *);
tree_cell *error_message (lex_ctxt *);
tree_cell *nasl_scanner_get_port (lex_ctxt *);
tree_cell *nasl_scanner_add_port (lex_ctxt *);
tree_cell *nasl_scanner_status (lex_ctxt *);
tree_cell *nasl_vendor_version (lex_ctxt *);
