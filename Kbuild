# Object file for the entire module
obj-m := rust_module.o

# Compose the module from our files
rust_module-objs := rust_module_rs.o license_shim.o

# Tell the kernel how to compile the Rust source file
RUST_TARGET := rust_module_rs.o

