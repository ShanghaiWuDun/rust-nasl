#!/usr/bin/env python3
#coding: utf8

import os
import sys
import argparse
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

    return " ".join(cflags) + " -I./"

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

    need_update = os.path.getmtime("./nasl/nasl_grammar.y") > os.path.getmtime("./nasl/nasl_grammar.tab.c") \
    or os.path.getmtime("./nasl/nasl_grammar.y") > os.path.getmtime("./nasl/nasl_grammar.tab.h") \
    or os.path.getmtime("./nasl/nasl_grammar.y") > os.path.getmtime("./nasl/nasl_grammar.output")

    c_file = os.path.exists("./nasl/nasl_grammar.tab.c")
    c_header = os.path.exists("./nasl/nasl_grammar.tab.h")
    c_output = os.path.exists("./nasl/nasl_grammar.output")

    if need_update or (not c_file or not c_header or not c_output):
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

    # HAVE_LIBKSBA  nasl_cert.c
    # HAVE_NETSNMP  nasl_snmp.c
    # NASL_DEBUG
    configs = "-D OPENVASSD_CONF=\"\\\"./\\\"\""
    configs += " -D GVM_PID_DIR=\"\\\"./\\\"\""
    configs += " -D OPENVAS_SYSCONF_DIR=\"\\\"./\\\"\""
    configs += " -D GVM_SYSCONF_DIR=\"\\\"./\\\"\""
    configs += " -D OPENVAS_NASL_VERSION=\"\\\"5.06\\\"\""
    configs += " -D OPENVASLIB_VERSION=\"\\\"5.06\\\"\""
    
    c_files = set()
    objs = set()
    headers = set()

    for project in ("base", "util", "misc", "nasl"):
        files = os.listdir(project)
        for filename in files:
            filename = "%s/%s" % ( project, filename )
            
            if "nasl-lint.c" in filename:
                continue
            if "nasl.c" in filename:
                continue

            if filename.endswith(".c"):
                objectname = filename[:-2] + ".o"
                need_update = os.path.getmtime(filename) > os.path.getmtime(objectname)
                if not os.path.exists(objectname) or need_update:
                    cmd = "clang -fPIC %s %s -c %s -o %s" % (cflags, configs, filename, objectname,)
                    print(cmd)
                    subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
                objs.add(objectname)
                c_files.add(filename)
            elif filename.endswith(".h"):
                headers.add(filename)

    objs = list(objs)
    headers = list(headers)
    c_files = list(c_files)

    # header_wrap = ""
    # for h in headers:
    #     header_wrap += "#include \"%s\"\n" % h
    # open("libnasl.h", "w").write(header_wrap)

    if not os.path.exists("./libnasl.a") or os.path.getmtime("./") > os.path.getmtime("./libnasl.a"):
        cmd = "ar -rcsv libnasl.a %s" % " ".join(objs)
        print(cmd)
        subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")
    
    if not os.path.exists("./nasl_interpreter") or os.path.getmtime("./") > os.path.getmtime("./nasl_interpreter"):
        if sys.platform == "darwin":
            cmd = "clang -fPIC %s %s %s libnasl.a nasl/nasl.c -o nasl_interpreter" % ( cflags, clibs, configs, )
        elif sys.platform == "linux":
            cmd = "clang -fPIC %s %s %s %s nasl/nasl.c -o nasl_interpreter" % ( cflags, clibs, configs, " ".join(objs), )
        print(cmd)
        subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

    if not os.path.exists("./libnasl.dylib") or os.path.getmtime("./") > os.path.getmtime("./libnasl.dylib"):
        if sys.platform == "darwin":
            cmd = "clang -shared -fPIC %s %s %s libnasl.a nasl/nasl.c -o libnasl.dylib" % ( cflags, clibs, configs, )
        elif sys.platform == "linux":
            cmd = "clang -shared -fPIC %s %s %s %s nasl/nasl.c -o libnasl.so" % ( cflags, clibs, configs, " ".join(objs))
        print(cmd)
        subprocess.run(cmd, stdout=subprocess.PIPE, check=True, shell=True).stdout.decode("utf-8")

def check():
    import ctypes

    if sys.platform == "darwin":
        libname = "./libnasl.dylib"
    elif sys.platform == "linux":
        libname = "./libnasl.so"

    dll = ctypes.cdll.LoadLibrary(libname)
    print("\n@Tests:")
    print("nasl_version: ", dll.nasl_version())


def install():
    assert(os.path.exists("./libnasl.a"))
    
    if sys.platform == "darwin":
        assert(os.path.exists("./libnasl.dylib"))
    elif sys.platform == "linux":
        assert(os.path.exists("./libnasl.so"))

    assert(os.path.exists("./nasl_interpreter"))
    
    os.system("sudo cp ./libnasl.dylib /usr/local/lib")
    os.system("sudo cp ./libnasl.a /usr/local/lib")
    os.system("sudo cp ./nasl_interpreter /usr/local/bin")


def clear():
    for project in ("base", "util", "misc", "nasl"):
        files = os.listdir(project)
        for filename in files:
            filename = "%s/%s" % ( project, filename )

            if filename.endswith(".o") or filename.endswith(".gch"):
                os.remove(filename)

    try:
        os.remove("./nasl_interpreter")
    except:
        pass
    try:
        os.remove("./libnasl.h")
    except:
        pass
    try:
        os.remove("./libnasl.a")
    except:
        pass

    if sys.platform == "darwin":
        os.remove("./libnasl.dylib")
    elif sys.platform == "linux":
        os.remove("./libnasl.so")


def main():
    parser = argparse.ArgumentParser(description='build script')
    parser.add_argument('mode', type=str, help="enum('build', 'clear')")

    args = parser.parse_args()

    if args.mode == "build":
        build()
    elif args.mode == "clear":
        try:
            clear()
        except:
            pass
    elif args.mode == "test":
        check()
    elif args.mode == "install":
        install()

if __name__ == '__main__':
    main()
