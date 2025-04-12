#![no_std]
#![feature(allocator_api)]

use kernel::prelude::*;

module! {
    type: RustModule,
    name: b"rust_module",
    author: b"salvadorg",
    description: b"Minimal out-of-tree Rust kernel module",
    license: b"GPL",
}

struct RustModule;

impl KernelModule for RustModule {
    fn init() -> Result<Self> {
        pr_info!("Hello from Rust kernel module!\n");
        Ok(RustModule)
    }
}

impl Drop for RustModule {
    fn drop(&mut self) {
        pr_info!("Rust kernel module unloaded!\n");
    }
}

