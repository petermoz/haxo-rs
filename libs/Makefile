# Downloads required RPi libraries and extracts them into the current folder.

.PHONY: all
all: download extract

.PHONY: download
download: libasound2_1.2.4-1.1_armhf.deb libasound2-dev_1.2.4-1.1_armhf.deb libfluidsynth2_2.1.7-1.1_armhf.deb libfluidsynth-dev_2.1.7-1.1_armhf.deb

.PHONY: extract
extract: usr/lib/arm-linux-gnueabihf/libasound.so usr/lib/arm-linux-gnueabihf/libfluidsynth.so

libasound2_1.2.4-1.1_armhf.deb:
	wget http://raspbian.raspberrypi.org/raspbian/pool/main/a/alsa-lib/libasound2_1.2.4-1.1_armhf.deb

libasound2-dev_1.2.4-1.1_armhf.deb:
	wget http://raspbian.raspberrypi.org/raspbian/pool/main/a/alsa-lib/libasound2-dev_1.2.4-1.1_armhf.deb

libfluidsynth2_2.1.7-1.1_armhf.deb:
	wget http://raspbian.raspberrypi.org/raspbian/pool/main/f/fluidsynth/libfluidsynth2_2.1.7-1.1_armhf.deb

libfluidsynth-dev_2.1.7-1.1_armhf.deb:
	wget http://raspbian.raspberrypi.org/raspbian/pool/main/f/fluidsynth/libfluidsynth-dev_2.1.7-1.1_armhf.deb


usr/lib/arm-linux-gnueabihf/libasound.so:
	dpkg -x libasound2_1.2.4-1.1_armhf.deb .
	dpkg -x libasound2-dev_1.2.4-1.1_armhf.deb .

usr/lib/arm-linux-gnueabihf/libfluidsynth.so:
	dpkg -x libfluidsynth2_2.1.7-1.1_armhf.deb .
	dpkg -x libfluidsynth-dev_2.1.7-1.1_armhf.deb .
