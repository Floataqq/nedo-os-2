#!/usr/bin/env sh

cp target/x86_64-nedo_os_2/release/nedo_os_2 isofiles/nedo_os_2
grub-mkrescue -o target/nedo_os_2.iso isofiles
qemu-system-x86_64 -cdrom target/nedo_os_2.iso
