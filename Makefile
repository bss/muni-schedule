APP_NAME := muni-schedule

.PHONE: default
default: osx rpi

rpi:
	vagrant up
	vagrant ssh -c "cd /vagrant && cargo build --target=armv7-unknown-linux-gnueabihf"

osx:
	cargo build

