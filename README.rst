rust-nasl
==========

:Date: 2019/01/11


.. contents::


前置依赖
----------

.. code:: bash
    
    # Install prerequisites on Debian GNU/Linux 'Stretch' 9
    apt-get install clang gcc make cmake pkg-config \
        libglib2.0-dev libgpgme-dev libgnutls28-dev libgcrypt20-dev \
        libssh-dev libssh-gcrypt-dev libldap2-dev libksba-dev libpcre3-dev \
        libpcap-dev uuid-dev libsnmp-dev libhiredis-dev \
        bison snmp nmap

    # macOS
    brew install libssh glib gio gnutls gpgme libgcrypt libksba 
    brew install libpcap pcre pcre2 hiredis ossp-uuid uuid 
    brew install net-snmp nmap
    brew install bison clang make cmake pkg-config


Build
--------

.. code:: bash
    
    cd c;
    python3 build.py build
    python3 build.py test
    python3 build.py install

    cd ../;
    cargo build -p nasl-sys
    cargo build


Run
--------

.. code:: bash
    
    cargo build

    redis-server ./c/redis_2_4.conf
    redis-cli -s "/tmp/redis.sock" monitor
    
    cargo run -- --include_dir ../openvas-data/nvt-feed/ --mode exec helloworld.nasl 127.0.0.1

