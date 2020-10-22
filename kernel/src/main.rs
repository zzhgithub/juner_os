#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(default_alloc_error_handler)]
#![feature(untagged_unions)]
#![feature(abi_x86_interrupt)]
#![feature(box_syntax)]
#![feature(wake_trait)]

extern crate alloc;

#[macro_use]
pub mod console;

pub mod drivers;
pub mod interrupts;
pub mod lang;
pub mod mal;
pub mod memory;
pub mod task;
pub mod board;
pub mod shell;

use bootloader::{entry_point, BootInfo};
use task::{executor::Executor, Task};
entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    memory::heap::init_heap();    // 初始化堆分配，以启用alloc库
    drivers::init_driver(boot_info); // 初始化串口输出和显示输出
    board::cpu::init_cpu(); // 初始化CPU特性
    board::acpi_table::get_acpi_addr(boot_info); // 从 boot_info中读取acpi_table address
    interrupts::init(); // 初始化Trap frame和中断
    memory::init_frame(boot_info); // 初始化内存Frame
    test!("This is test");
    error!("This is error");
    warn!("This is warn");
    debug!("This is debug");
    println!("This is println");
    shell::init_shell();
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
