#include <libssh/libssh.h>


// SNMP
tree_cell *nasl_snmpv1_get (lex_ctxt *);
tree_cell *nasl_snmpv2c_get (lex_ctxt *);
tree_cell *nasl_snmpv3_get (lex_ctxt *);

// SSH
tree_cell *nasl_ssh_connect (lex_ctxt *lexic);
tree_cell *nasl_ssh_disconnect (lex_ctxt *lexic);
tree_cell *nasl_ssh_session_id_from_sock (lex_ctxt *lexic);
tree_cell *nasl_ssh_get_sock (lex_ctxt *lexic);
tree_cell *nasl_ssh_set_login (lex_ctxt *lexic);
tree_cell *nasl_ssh_userauth (lex_ctxt *lexic);
tree_cell *nasl_ssh_request_exec (lex_ctxt *lexic);
tree_cell *nasl_ssh_shell_open (lex_ctxt *lexic);
tree_cell *nasl_ssh_shell_read (lex_ctxt *lexic);
tree_cell *nasl_ssh_shell_write (lex_ctxt *lexic);
tree_cell *nasl_ssh_shell_close (lex_ctxt *lexic);
tree_cell *nasl_ssh_login_interactive (lex_ctxt *lexic);
tree_cell *nasl_ssh_login_interactive_pass (lex_ctxt *lexic);

tree_cell *nasl_ssh_exec (lex_ctxt *);

tree_cell *nasl_ssh_get_issue_banner (lex_ctxt *lexic);
tree_cell *nasl_ssh_get_server_banner (lex_ctxt *lexic);
tree_cell *nasl_ssh_get_auth_methods (lex_ctxt *lexic);
tree_cell *nasl_ssh_get_host_key (lex_ctxt *lexic);


// WMI
tree_cell *nasl_wmi_versioninfo (lex_ctxt * lexic);
tree_cell *nasl_wmi_connect (lex_ctxt * lexic);
tree_cell *nasl_wmi_close (lex_ctxt * lexic);
tree_cell *nasl_wmi_query (lex_ctxt * lexic);

tree_cell *nasl_wmi_connect_rsop (lex_ctxt * lexic);
tree_cell *nasl_wmi_query_rsop (lex_ctxt * lexic);

tree_cell *nasl_wmi_connect_reg (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_get_sz (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_enum_value (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_enum_key (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_get_bin_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_get_dword_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_get_ex_string_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_get_mul_string_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_get_qword_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_set_dword_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_set_qword_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_set_ex_string_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_set_string_val (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_create_key (lex_ctxt * lexic);
tree_cell *nasl_wmi_reg_delete_key (lex_ctxt * lexic);


// SMB
tree_cell *nasl_smb_versioninfo (lex_ctxt * lexic);
tree_cell *nasl_smb_connect (lex_ctxt * lexic);
tree_cell *nasl_smb_close (lex_ctxt * lexic);
tree_cell *nasl_smb_file_SDDL (lex_ctxt * lexic);
tree_cell *nasl_smb_file_owner_sid (lex_ctxt * lexic);
tree_cell *nasl_smb_file_group_sid (lex_ctxt * lexic);
tree_cell *nasl_smb_file_trustee_rights (lex_ctxt * lexic);
tree_cell *nasl_win_cmd_exec (lex_ctxt * lexic);


// HTTP
tree_cell *http_open_socket (lex_ctxt *);
tree_cell *http_close_socket (lex_ctxt *);
tree_cell *http_get (lex_ctxt *);
tree_cell *http_head (lex_ctxt *);
tree_cell *http_post (lex_ctxt *);
tree_cell *http_delete (lex_ctxt *);
tree_cell *http_put (lex_ctxt *);
tree_cell *nasl_http_recv_headers (lex_ctxt *);
tree_cell *cgibin (lex_ctxt *);
tree_cell *nasl_is_cgi_installed (lex_ctxt *);

tree_cell *nasl_http_keepalive_send_recv (lex_ctxt *);
tree_cell *nasl_http_share_exists (lex_ctxt *);


// IP
tree_cell *forge_ip_packet (lex_ctxt *);
tree_cell *set_ip_elements (lex_ctxt *);
tree_cell *get_ip_element (lex_ctxt *);
tree_cell *dump_ip_packet (lex_ctxt *);
tree_cell *insert_ip_options (lex_ctxt *);


// TCP
tree_cell *forge_tcp_packet (lex_ctxt *);
tree_cell *get_tcp_element (lex_ctxt *);
tree_cell *set_tcp_elements (lex_ctxt *);
tree_cell *dump_tcp_packet (lex_ctxt *);
tree_cell *nasl_tcp_ping (lex_ctxt *);

// UDP
tree_cell *forge_udp_packet (lex_ctxt *);
tree_cell *set_udp_elements (lex_ctxt *);
tree_cell *dump_udp_packet (lex_ctxt *);
tree_cell *get_udp_element (lex_ctxt *);

// ICMP
tree_cell *forge_icmp_packet (lex_ctxt *);
tree_cell *get_icmp_element (lex_ctxt *);

tree_cell *forge_igmp_packet (lex_ctxt *);


tree_cell *nasl_send_packet (lex_ctxt *);
tree_cell *nasl_pcap_next (lex_ctxt *);
tree_cell *nasl_send_capture (lex_ctxt *);


tree_cell *forge_ipv6_packet (lex_ctxt *);
tree_cell *set_ipv6_elements (lex_ctxt *);
tree_cell *get_ipv6_element (lex_ctxt *);
tree_cell *dump_ipv6_packet (lex_ctxt *);
tree_cell *insert_ipv6_options (lex_ctxt *);

tree_cell *forge_tcp_v6_packet (lex_ctxt *);
tree_cell *get_tcp_v6_element (lex_ctxt *);
tree_cell *set_tcp_v6_elements (lex_ctxt *);
tree_cell *dump_tcp_v6_packet (lex_ctxt *);

tree_cell *forge_udp_v6_packet (lex_ctxt *);
tree_cell *set_udp_v6_elements (lex_ctxt *);
tree_cell *dump_udp_v6_packet (lex_ctxt *);
tree_cell *get_udp_v6_element (lex_ctxt *);

tree_cell *forge_icmp_v6_packet (lex_ctxt *);
tree_cell *get_icmp_v6_element (lex_ctxt *);

tree_cell *forge_igmp_v6_packet (lex_ctxt *);

tree_cell *nasl_tcp_v6_ping (lex_ctxt *);
tree_cell *nasl_send_v6packet (lex_ctxt *);





tree_cell *add_hostname (lex_ctxt *);
tree_cell *get_hostname (lex_ctxt *);
tree_cell *get_hostnames (lex_ctxt *);
tree_cell *get_hostname_source (lex_ctxt *);
tree_cell *resolve_hostname (lex_ctxt *);
tree_cell *get_host_ip (lex_ctxt *);
tree_cell *get_host_open_port (lex_ctxt *);
tree_cell *get_port_state (lex_ctxt *);
tree_cell *get_udp_port_state (lex_ctxt *);
tree_cell *nasl_islocalhost (lex_ctxt *);
tree_cell *nasl_islocalnet (lex_ctxt *);
tree_cell *nasl_this_host (lex_ctxt *);
tree_cell *nasl_this_host_name (lex_ctxt *);
tree_cell *get_port_transport (lex_ctxt *);
tree_cell *nasl_same_host (lex_ctxt *);
tree_cell *nasl_target_is_ipv6 (lex_ctxt * lexic);





tree_cell *nasl_open_sock_tcp (lex_ctxt *);
tree_cell *nasl_open_sock_udp (lex_ctxt *);
/* private func */
tree_cell *nasl_open_sock_tcp_bufsz (lex_ctxt *, int);
tree_cell *nasl_socket_get_error (lex_ctxt *);

tree_cell *nasl_open_priv_sock_tcp (lex_ctxt *);
tree_cell *nasl_open_priv_sock_udp (lex_ctxt *);

tree_cell *nasl_send (lex_ctxt *);
tree_cell *nasl_socket_negotiate_ssl (lex_ctxt *);
tree_cell *nasl_recv (lex_ctxt *);
tree_cell *nasl_recv_line (lex_ctxt *);
tree_cell *nasl_socket_get_cert (lex_ctxt *);
tree_cell *nasl_socket_get_ssl_session_id (lex_ctxt *);
tree_cell *nasl_socket_get_ssl_version (lex_ctxt *);
tree_cell *nasl_socket_get_ssl_ciphersuite (lex_ctxt *);
tree_cell *nasl_socket_cert_verify (lex_ctxt *);

tree_cell *nasl_close_socket (lex_ctxt *);

tree_cell *nasl_join_multicast_group (lex_ctxt *);
tree_cell *nasl_leave_multicast_group (lex_ctxt *);

tree_cell *nasl_get_source_port (lex_ctxt *);

tree_cell *nasl_get_sock_info (lex_ctxt *lexic);


