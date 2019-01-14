#!/usr/bin/env python3
#coding: utf8

import os
import sys
import subprocess


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
        "pkg-config --cflags uuid",
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
        "pkg-config --libs uuid",
    ]

    for cmd in cmds:
        clib = subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
        clibs.append(clib.replace("\n", ""))

    return " ".join(clibs) + " -lssh -lpcre -lm -lz "


def build_grammar():
    assert(os.path.exists("./nasl/nasl_grammar.y"))

    c_file = os.path.exists("./nasl/nasl_grammar.tab.c")
    c_header = os.path.exists("./nasl/nasl_grammar.tab.h")
    c_output = os.path.exists("./nasl/nasl_grammar.output")

    if not c_file or not c_header or not c_output:
        cmd = "cd nasl && bison -d -v -t -p nasl ./nasl_grammar.y"
        print(cmd)
        subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    assert(os.path.exists("./nasl/nasl_grammar.tab.c"))
    assert(os.path.exists("./nasl/nasl_grammar.tab.h"))
    assert(os.path.exists("./nasl/nasl_grammar.output"))


def build():
    build_grammar()

    cflags = get_cflags()
    clibs = get_libs()
    
    configs = "-D OPENVASSD_CONF=\"\\\"./\\\"\""
    configs += " -D GVM_PID_DIR=\"\\\"./\\\"\""
    configs += " -D OPENVAS_SYSCONF_DIR=\"\\\"./\\\"\""
    configs += " -D GVM_SYSCONF_DIR=\"\\\"./\\\"\""
    configs += " -D OPENVAS_NASL_VERSION=\"\\\"master\\\"\""
    configs += " -D OPENVASLIB_VERSION=\"\\\"master\\\"\""
    
    objs = set()
    headers = set()

    for project in ("base", "util", "misc", "nasl"):
        files = os.listdir(project)
        for filename in files:
            filename = "%s/%s" % ( project, filename )
            
            if "nasl-lint.c" in filename:
                continue

            if filename.endswith(".c"):
                objectname = filename[:-2] + ".o"
                if not os.path.exists(objectname):
                    # -fPIC
                    cmd = "clang -fPIC %s %s -c %s -o %s" % (cflags, configs, filename, objectname,)
                    print(cmd)
                    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
                objs.add(objectname)
            elif filename.endswith(".h"):
                headers.add(filename)

    objs = list(objs)
    headers = list(headers)

    header_wrap = ""
    for h in headers:
        header_wrap += "#include \"%s\"\n" % h
    open("libnasl.h", "w").write(header_wrap)


    cmd = "ar -rcsv libnasl.a %s" % " ".join(objs)
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
    
    # clang -fPIC -static \
    # `pkg-config --libs glib-2.0` \
    # `pkg-config --libs gio-2.0` \
    # `pkg-config --libs gnutls` \
    # `ksba-config --libs` \
    # `gpgme-config --libs` \
    # `libgcrypt-config --libs` \
    # `pkg-config --libs hiredis` \
    # `pkg-config --libs uuid` \
    # `pcap-config --libs` \
    # -lssh -lm -lz \
    # libabc.a -o abc
    
    static_libs = [
        # "/usr/lib/x86_64-linux-gnu/libglib-2.0.a",
        # "/usr/lib/x86_64-linux-gnu/libgio-2.0.a",
        # "-lglib-2.0 -l gio-2.0",
        # "/usr/lib/x86_64-linux-gnu/libgnutls.a",
        "/usr/lib/x86_64-linux-gnu/libksba.a",
        "/usr/lib/x86_64-linux-gnu/libassuan.a",
        # "/usr/lib/x86_64-linux-gnu/libgpg-error.a",
        # "/usr/lib/x86_64-linux-gnu/libgpgme.a",
        # "/usr/lib/x86_64-linux-gnu/libgcrypt.a",
        "/usr/lib/x86_64-linux-gnu/libhiredis.a",
        # "/usr/lib/x86_64-linux-gnu/libuuid.a",
        # "/usr/lib/x86_64-linux-gnu/libpcap.a",
        "/usr/lib/x86_64-linux-gnu/libssh.a",
        # "/usr/lib/x86_64-linux-gnu/libm.a",
        # "/usr/lib/x86_64-linux-gnu/libz.a",
    ]
    

    cmd = "clang -shared -fPIC %s %s %s -o libnasl.so" % ( cflags, clibs, " ".join(objs))
    # cmd = "clang -shared -fPIC %s %s libnasl.a -o libnasl.so" % ( cflags, clibs,)
    # cmd = "clang -fPIC -shared %s %s %s -o libnasl.so" % ( " ".join(objs), " ".join(static_libs), clibs)
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    cmd = "clang -fPIC %s %s libnasl.a -o nasli" % ( cflags, clibs, )
    print(cmd)
    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")


def check():
    import ctypes

    dll = ctypes.cdll.LoadLibrary("./libnasl.so")
    print("\n@Tests:")
    print(dll.nasl_version())
    

def main():
    build()
    check()


if __name__ == '__main__':
    main()
