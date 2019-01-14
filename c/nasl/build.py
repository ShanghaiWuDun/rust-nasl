#!/usr/bin/env python3
#coding: utf8

import os
import sys
import subprocess


FILES = """arc4.c capture_packet.c charcnv.c exec.c genrand.c hmacmd5.c
    iconv.c lint.c md4.c md5.c nasl.c nasl_builtin_find_service.c
    nasl_builtin_openvas_tcp_scanner.c nasl_builtin_synscan.c nasl_builtin_nmap.c
    nasl_cmd_exec.c nasl_crypto2.c nasl_snmp.c nasl_ssh.c nasl_cert.c
    nasl_crypto.c nasl_debug.c nasl_func.c nasl_grammar.tab.c nasl_host.c
    nasl_http.c nasl_init.c nasl_lex_ctxt.c nasl_misc_funcs.c nasl_scanner_glue.c
    nasl_packet_forgery.c nasl_packet_forgery_v6.c nasl_signature.c nasl_smb.c
    nasl_socket.c nasl_text_utils.c nasl_tree.c nasl_var.c nasl_wmi.c
    nasl_isotime.c ntlmssp.c strutils.c smb_crypt.c smb_crypt2.c
    smb_signing.c time.c     
    smb_interface_stub.c wmi_interface_stub.c
    """

def get_cflags():
    cflags = []
    cmds = [
        "pkg-config --cflags glib-2.0",
        "pkg-config --cflags gio-2.0",
        "ksba-config --cflags",
        "pcap-config --cflags",
        "gpgme-config --cflags",
        "libgcrypt-config --cflags",
        "pkg-config --cflags hiredis",
    ]
    for cmd in cmds:
        cflag = subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
        cflags.append(cflag.replace("\n", ""))

    return " ".join(cflags)

def get_libs():
    clibs = []
    cmds = [
        "pkg-config --libs glib-2.0",
        "pkg-config --libs gio-2.0",
        "pkg-config --libs gnutls",
        "ksba-config --libs",
        "pcap-config --libs",
        "gpgme-config --libs",
        "libgcrypt-config --libs",
        "pkg-config --libs hiredis",
    ]

    for cmd in cmds:
        clib = subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
        clibs.append(clib.replace("\n", ""))

    # -l m 为 libm , math
    return " ".join(clibs) + " -lssh -lm"


def build_grammar():
    cmd = "bison -d -v -t -p nasl ./nasl_grammar.y"
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    assert(os.path.exists("./nasl_grammar.y"))
    assert(os.path.exists("./nasl_grammar.tab.c"))
    assert(os.path.exists("./nasl_grammar.tab.h"))
    assert(os.path.exists("./nasl_grammar.output"))

def build():
    build_grammar()

    cflags = get_cflags()
    clibs = get_libs()

    configs = "-D OPENVASSD_CONF=\"\\\"./\\\"\""
    configs += " -D OPENVAS_SYSCONF_DIR=\"\\\"./\\\"\""
    configs += " -D GVM_SYSCONF_DIR=\"\\\"./\\\"\""
    configs += " -D OPENVAS_NASL_VERSION=\"\\\"master\\\"\""
    configs += " -D OPENVASLIB_VERSION=\"\\\"master\\\"\""

    objs = []
    for fname in FILES.split(" "):
        filename = fname.strip()
        if len(filename) > 0 and filename.endswith(".c"):
            objectname = filename[:-2] + ".o"
            
            
            cmd = "clang -fPIC %s %s -c %s -o %s" % (cflags, configs, filename, objectname,)
            print(cmd)
            subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
            objs.append(objectname)

    cmd = "ar -rv ../libopenvas.a %s" % " ".join(objs)
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    cmd = "clang -shared -fPIC %s %s -o ../libopenvas.so" % (" ".join(objs), clibs, )
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    
    cmd = "clang -fPIC -I../base -I../util -I../misc -I../ ../libbase.so ../libutil.so ../libopenvas.so ../libnasl_misc.so"
    cmd += " %s %s %s -o ../nasl_cli" % (cflags, clibs, configs)
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    cmd = "clang -fPIC -I../base -I../util -I../misc -I../ ../libbase.so ../libutil.so ../libopenvas.so ../libnasl_misc.so"
    cmd += " %s %s %s nasl-lint.c -o ../nasl_lint" % (cflags, clibs, configs)
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")


def main():
    build()


if __name__ == '__main__':
    main()