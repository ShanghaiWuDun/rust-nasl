

tree_cell *nasl_rand (lex_ctxt *);
tree_cell *nasl_usleep (lex_ctxt *);
tree_cell *nasl_sleep (lex_ctxt *);
tree_cell *nasl_ftp_log_in (lex_ctxt *);
tree_cell *nasl_ftp_get_pasv_address (lex_ctxt *);
tree_cell *nasl_telnet_init (lex_ctxt *);
tree_cell *nasl_start_denial (lex_ctxt *);
tree_cell *nasl_end_denial (lex_ctxt *);
tree_cell *nasl_dump_ctxt (lex_ctxt *);
tree_cell *nasl_do_exit (lex_ctxt *);
tree_cell *nasl_isnull (lex_ctxt *);
tree_cell *nasl_make_list (lex_ctxt *);
tree_cell *nasl_make_array (lex_ctxt *);
tree_cell *nasl_keys (lex_ctxt *);
tree_cell *nasl_max_index (lex_ctxt *);
tree_cell *nasl_typeof (lex_ctxt *);
tree_cell *nasl_defined_func (lex_ctxt *);
tree_cell *nasl_sort_array (lex_ctxt *);
tree_cell *nasl_unixtime (lex_ctxt *);
tree_cell *nasl_gettimeofday (lex_ctxt *);
tree_cell *nasl_localtime (lex_ctxt *);
tree_cell *nasl_mktime (lex_ctxt *);
tree_cell *nasl_open_sock_kdc (lex_ctxt *);
tree_cell *nasl_dec2str (lex_ctxt *);
tree_cell *nasl_get_byte_order (lex_ctxt *);
tree_cell *nasl_gunzip (lex_ctxt *);
tree_cell *nasl_gzip (lex_ctxt *);


// CMD Exec
tree_cell *nasl_pread (lex_ctxt *);
tree_cell *nasl_find_in_path (lex_ctxt *);

tree_cell *nasl_fread (lex_ctxt *);
tree_cell *nasl_fwrite (lex_ctxt *);
tree_cell *nasl_unlink (lex_ctxt *);
tree_cell *nasl_get_tmp_dir (lex_ctxt *);
tree_cell *nasl_file_stat (lex_ctxt *);
tree_cell *nasl_file_open (lex_ctxt *);
tree_cell *nasl_file_close (lex_ctxt *);
tree_cell *nasl_file_read (lex_ctxt *);
tree_cell *nasl_file_write (lex_ctxt *);
tree_cell *nasl_file_seek (lex_ctxt *);