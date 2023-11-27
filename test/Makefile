.PHONY:	test clean

test: build/os.img
	qemu-system-x86_64	\
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-serial stdio \
		-no-reboot \
		-display none \
		-drive format=raw,file=build/os.img;	\
	if [ $$? -eq 33 ];\
	then\
		echo "Test passed";\
	else\
		echo "Test failed";\
		exit 1;\
	fi

build/os.img: target/x86_64-unknown-uefi/debug/xhci-test.efi build
	dd if=/dev/zero of=build/os.img bs=512 count=93750
	mformat -i build/os.img -h 200 -t 500 -s 144::
	mmd -i build/os.img ::/efi
	mmd -i build/os.img ::/efi/boot
	mcopy -i build/os.img target/x86_64-unknown-uefi/debug/xhci-test.efi ::/efi/boot/bootx64.efi

target/x86_64-unknown-uefi/debug/xhci-test.efi: src/main.rs
	cargo build

build:
	mkdir build

clean:
	rm -rf build
	cargo clean
