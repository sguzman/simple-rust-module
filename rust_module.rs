// SPDX-License-Identifier: GPL-2.0
#![no_std]
#![feature(allocator_api)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(panic_info_message)]

use kernel::prelude::*;

module! {
    type: RustModule,
    name: b"rust_module",
    author: b"salvadorg",
    description: b"Minimal Rust kernel module",
    license: b"GPL",
}

struct RustModule;

impl KernelModule for RustModule {
    fn init() -> Result<Self> {
        pr_info!("Rust kernel module loaded!\n");
        Ok(RustModule)
    }
}

impl Drop for RustModule {
    fn drop(&mut self) {
        pr_info!("Rust kernel module unloaded!\n");
    }
}

