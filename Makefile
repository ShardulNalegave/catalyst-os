
build_kernel:
	cargo build --target x86_64-catalyst.json \
		-Z build-std="core,compiler_builtins" \
		-Z build-std-features="compiler-builtins-mem"
