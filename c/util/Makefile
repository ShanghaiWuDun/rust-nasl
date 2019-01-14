default: all

# Requires.private: glib-2.0 >= 2.42.1, gio-2.0 >= 2.42.1, zlib >= 1.2.8, gnutls >= 3.2.15, uuid >= 2.25.0

all:
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c authutils.c -o authutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c compressutils.c -o compressutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c fileutils.c -o fileutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c gpgmeutils.c -o gpgmeutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c kb.c -o kb.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c ldaputils.c -o ldaputils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c nvticache.c -o nvticache.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c radiusutils.c -o radiusutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c serverutils.c -o serverutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c sshutils.c -o sshutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c uuidutils.c -o uuidutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c xmlutils.c -o xmlutils.o
	ar -rv ../libutil.a \
		authutils.o compressutils.o fileutils.o gpgmeutils.o kb.o ldaputils.o \
		nvticache.o radiusutils.o serverutils.o sshutils.o uuidutils.o xmlutils.o
	clang -shared -fPIC authutils.o compressutils.o fileutils.o gpgmeutils.o kb.o ldaputils.o \
		nvticache.o radiusutils.o serverutils.o sshutils.o uuidutils.o xmlutils.o \
		`pkg-config --libs glib-2.0` \
		`pkg-config --libs gio-2.0` \
		`pkg-config --libs zlib` \
		`pkg-config --libs gnutls` \
		`pkg-config --libs uuid` \
		-lpcre -o ../libutil.so