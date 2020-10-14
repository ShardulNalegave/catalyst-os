
# Catalyst OS
A toy OS in rust.

![Hello, World!](images/hello_world.png)
---

## Usage
Currently, there are no prebuilt binaries.
Instead, you can build and run it from source code.

```bash

# Clone the repo
git clone https://github.com/ShardulNalegave/catalyst-os.git catalyst
cd catalyst

rustup override set nightly

# To build the kernel:-
cd kernel
make build

# To run the kernel (Note: this will rebuild the kernel):-
cd kernel
make run

```

---
**Licensed under the [MIT License](./LICENSE)**