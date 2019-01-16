

#define MD4_DIGEST_LENGTH 16

int generate_script_signature (char *);

tree_cell *nasl_cert_open (lex_ctxt *lexic);
tree_cell *nasl_cert_close (lex_ctxt *lexic);
tree_cell *nasl_cert_query (lex_ctxt *lexic);

tree_cell *nasl_md2 (lex_ctxt *);
tree_cell *nasl_md4 (lex_ctxt *);
tree_cell *nasl_md5 (lex_ctxt *);
tree_cell *nasl_sha (lex_ctxt *);
tree_cell *nasl_sha1 (lex_ctxt *);
tree_cell *nasl_sha256 (lex_ctxt *);
tree_cell *nasl_ripemd160 (lex_ctxt *);
tree_cell *nasl_hmac_md2 (lex_ctxt *);
tree_cell *nasl_hmac_md5 (lex_ctxt *);
tree_cell *nasl_hmac_sha1 (lex_ctxt *);
tree_cell *nasl_hmac_sha256 (lex_ctxt *);
tree_cell *nasl_hmac_sha384 (lex_ctxt *);
tree_cell *nasl_hmac_sha512 (lex_ctxt *);
tree_cell *nasl_hmac_dss (lex_ctxt *);
tree_cell *nasl_hmac_ripemd160 (lex_ctxt *);
tree_cell *nasl_prf_sha256 (lex_ctxt *);
tree_cell *nasl_prf_sha384 (lex_ctxt *);
tree_cell *nasl_tls1_prf (lex_ctxt *);
tree_cell *nasl_ntlmv1_hash (lex_ctxt *);
tree_cell *nasl_nt_owf_gen (lex_ctxt *);
tree_cell *nasl_lm_owf_gen (lex_ctxt *);
tree_cell *nasl_ntv2_owf_gen (lex_ctxt *);
tree_cell *nasl_ntlmv2_hash (lex_ctxt *);
tree_cell *nasl_ntlmv2_response (lex_ctxt * lexic);
tree_cell *nasl_ntlm2_response (lex_ctxt * lexic);
tree_cell *nasl_ntlm_response (lex_ctxt * lexic);
tree_cell *nasl_keyexchg (lex_ctxt * lexic);
tree_cell *nasl_insert_hexzeros (lex_ctxt * lexic);
tree_cell *nasl_get_password (lex_ctxt * lexic);
tree_cell *nasl_get_sign (lex_ctxt * lexic);
tree_cell *nasl_get_smb2_sign (lex_ctxt * lexic);
tree_cell *nasl_hmac_sha256 (lex_ctxt * lexic);
tree_cell *nasl_cipher_des (lex_ctxt *);


tree_cell *nasl_bn_random (lex_ctxt *);
tree_cell *nasl_dh_generate_key (lex_ctxt *);
tree_cell *nasl_bn_cmp (lex_ctxt *);
tree_cell *nasl_dh_compute_key (lex_ctxt *);
tree_cell *nasl_rsa_public_encrypt (lex_ctxt *);
tree_cell *nasl_rsa_private_decrypt (lex_ctxt *);
tree_cell *nasl_rsa_public_decrypt (lex_ctxt *);
tree_cell *nasl_bf_cbc_encrypt (lex_ctxt *);
tree_cell *nasl_bf_cbc_decrypt (lex_ctxt *);
tree_cell *nasl_dsa_do_verify (lex_ctxt * lexic);
tree_cell *nasl_pem_to_rsa (lex_ctxt * lexic);
tree_cell *nasl_pem_to_dsa (lex_ctxt * lexic);
tree_cell *nasl_rsa_sign (lex_ctxt * lexic);
tree_cell *nasl_dsa_do_sign (lex_ctxt * lexic);
tree_cell *nasl_rc4_encrypt (lex_ctxt * lexic);
tree_cell *nasl_aes128_cbc_encrypt (lex_ctxt * lexic);
tree_cell *nasl_aes256_cbc_encrypt (lex_ctxt * lexic);
tree_cell *nasl_aes128_ctr_encrypt (lex_ctxt * lexic);
tree_cell *nasl_aes256_ctr_encrypt (lex_ctxt * lexic);
tree_cell *nasl_des_ede_cbc_encrypt (lex_ctxt * lexic);
tree_cell *nasl_aes128_gcm_encrypt (lex_ctxt * lexic);
tree_cell *nasl_aes256_gcm_encrypt (lex_ctxt * lexic);

