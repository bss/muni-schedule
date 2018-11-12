APP_NAME := muni-schedule
VAGRANT_USER_HOME := /home/ubuntu

.PHONE: default
default: osx rpi

rpi:
	vagrant up
	vagrant ssh -c "cd /vagrant && OPENSSL_DIR=$(VAGRANT_USER_HOME)/openssl cargo build --target=armv7-unknown-linux-gnueabihf"

osx:
	cargo build

