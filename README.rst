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


Build
--------

.. code:: bash
    
    cd c;
    python3 build.py
    sudo cp ./c/libnasl.a /usr/local/lib
    sudo cp ./c/libnasl.so /usr/local/lib
    sudo cp ./c/nasli /usr/local/bin

    # Build Rust lib
    cargo build -p nasl-sys

