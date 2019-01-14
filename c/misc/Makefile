default: all

# pkg_check_modules (GLIB REQUIRED glib-2.0>=2.42)
# pkg_check_modules (GNUTLS REQUIRED gnutls>=3.2.15)

# pkg_check_modules (LIBGVM_BASE REQUIRED libgvm_base>=1.0.0)
# pkg_check_modules (LIBGVM_UTIL REQUIRED libgvm_util>=1.0.0)

# pkg_check_modules (OPENVAS_WMICLIENT libopenvas_wmiclient>=1.0.1)
# pkg_check_modules (OPENVAS_WINCMD libopenvas_wincmd>=1.0.1)
# find_library (PCAP pcap)
# find_program (PCAP_CONFIG pcap-config)

# set (MISC_VERSION_MAJOR 9)
# set (MISC_VERSION_MINOR 1)
# set (MISC_VERSION_PATCH 0)
# set (MISC_VERSION_STRING "${MISC_VERSION_MAJOR}.${MISC_VERSION_MINOR}${MISC_VERSION_SUFFIX}")
# set (MISC_PACKAGE_VERSION "${MISC_VERSION_STRING}${PROJECT_VERSION_GIT}")
# set (MISC_VERSION "${MISC_VERSION_STRING}")

all:
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c bpf_share.c -o bpf_share.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c ftp_funcs.c -o ftp_funcs.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c vendorversion.c -o vendorversion.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c network.c -o network.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c plugutils.c -o plugutils.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" -c pcap.c -o pcap.o

	ar -rv ../libnasl_misc.a bpf_share.o ftp_funcs.o vendorversion.o network.o plugutils.o pcap.o

	clang -shared -fPIC bpf_share.o ftp_funcs.o vendorversion.o network.o plugutils.o pcap.o \
		`pkg-config --libs glib-2.0` \
		`pkg-config --libs gnutls` \
		`pkg-config --libs uuid` \
		`pcap-config --libs` \
		`gpgme-config --libs` \
		-o ../libnasl_misc.so