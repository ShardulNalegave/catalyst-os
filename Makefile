
build_kernel:
	cargo bootimage --target x86_64-catalyst.json \
		-Z build-std="core,compiler_builtins" \
		-Z build-std-features="compiler-builtins-mem"

run_kernel: build_kernel
	qemu-system-x86_64 -drive format=raw,file=P:\catalyst\target\x86_64-catalyst\debug\bootimage-catalyst.bin