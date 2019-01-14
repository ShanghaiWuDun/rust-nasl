default: all

# Requires.private: glib-2.0 >= 2.42.1
	
all:
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" array.c -c -o array.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" credentials.c -c -o credentials.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" cvss.c -c -o cvss.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" drop_privileges.c -c -o drop_privileges.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" hosts.c -c -o hosts.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" logging.c -c -o logging.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" networking.c -c -o networking.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" nvti.c -c -o nvti.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" pidfile.c -c -o pidfile.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" prefs.c -c -o prefs.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" proctitle.c -c -o proctitle.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" pwpolicy.c -c -o pwpolicy.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" settings.c -c -o settings.o
	clang -fPIC `pkg-config --cflags glib-2.0` -D GVM_PID_DIR="\"./\"" -D GVM_SYSCONF_DIR="\"/etc/openvase/\"" strings.c -c -o strings.o
	ar -rv ../libbase.a \
		array.o credentials.o cvss.o drop_privileges.o hosts.o logging.o networking.o nvti.o \
		pidfile.o prefs.o proctitle.o pwpolicy.o settings.o strings.o 
	clang -shared -fPIC array.o credentials.o cvss.o drop_privileges.o hosts.o logging.o networking.o nvti.o \
		pidfile.o prefs.o proctitle.o pwpolicy.o settings.o strings.o \
		`pkg-config --libs glib-2.0` -lpcre -o ../libbase.so
	