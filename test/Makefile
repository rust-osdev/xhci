IMG = build/os.img
EFI = ../target/x86_64-unknown-uefi/debug/xhci-test.efi

.PHONY:	test clean

test: $(IMG)
	qemu-system-x86_64	\
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-device qemu-xhci,id=xhci\
		-serial stdio \
		-no-reboot \
		-display none \
		-drive format=raw,file=$(IMG);	\
	if [ $$? -eq 33 ];\
	then\
		echo "Test passed";\
	else\
		echo "Test failed";\
		exit 1;\
	fi

$(IMG): $(EFI) build
	dd if=/dev/zero of=$@ bs=512 count=93750
	mformat -i $@ -h 200 -t 500 -s 144::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(EFI) ::/efi/boot/bootx64.efi

$(EFI): $(wildcard src/*.rs) Cargo.toml
	cargo build

build:
	mkdir build

clean:
	rm -rf build
	cargo clean
