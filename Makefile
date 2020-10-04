
build:
	cargo bootimage --target x86_64-catalyst.json -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem

run-qemu:
	make build
	qemu-system-x86_64 -drive format=raw,file=target\x86_64-catalyst\debug\bootimage-catalyst.bin