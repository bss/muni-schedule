# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/xenial64"

  config.vm.provider "virtualbox" do |vb|
    vb.gui = false
    vb.memory = "1536"
  end
  
  config.vm.provision "shell", privileged: true, inline: <<-SHELL
    set -xe
    export DEBIAN_FRONTEND=noninteractive

    apt-get update
    apt upgrade -yq
    apt-get install -yq curl gcc-arm-linux-gnueabihf pkg-config

    # Make perl not shit itself
    locale-gen en_US en_US.UTF-8
    update-locale LANG="en_US.UTF-8" LC_ALL="en_US.UTF-8"
  SHELL
  # libssl-dev

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    set -xe
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env
    rustup target add armv7-unknown-linux-gnueabihf

    mkdir -p ~/.cargo
    echo "[target.armv7-unknown-linux-gnueabihf]" > ~/.cargo/config
    echo "linker = \"arm-linux-gnueabihf-gcc\"" >> ~/.cargo/config

    wget https://www.openssl.org/source/openssl-1.1.0f.tar.gz
    tar xzf openssl-1.1.0f.tar.gz
    pushd openssl-1.1.0f
    CROSS_COMPILE="arm-linux-gnueabihf-" ./Configure --prefix=$HOME/openssl linux-armv4 -march=armv7-a -Wa,--noexecstack
    make
    make install
    popd
  SHELL
end
