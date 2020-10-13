
# Catalyst OS
![Hello, World!](images/hello_world.png)
---

## Installation and Usage
Currently, there are no prebuilt binaries.
Instead, you can build it from the source.

```bash

# Clone the repo
git clone https://github.com/ShardulNalegave/catalyst-os.git catalyst
cd catalyst

rustup override set nightly

# To build the kernel:-
make build_kernel

# To run the kernel (Note this will rebuild the kernel):-
make run_kernel

```

---
**Licensed under the [MIT License](./LICENSE)**