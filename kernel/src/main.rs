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

pub mod board;
pub mod drivers;
pub mod fs;
pub mod interrupts;
pub mod lang;
pub mod mal;
pub mod memory;
pub mod shell;
pub mod task;

use bootloader::{entry_point, BootInfo};
use task::{executor::Executor, Task};
use log::*;
entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    init_log();
    memory::heap::init_heap(); // 初始化堆分配，以启用alloc库
    memory::init_frame(boot_info); // 初始化内存Frame
    drivers::init_driver(boot_info); // 初始化串口输出和显示输出
    board::cpu::init_cpu(); // 初始化CPU特性
    board::acpi_table::get_acpi_addr(boot_info); // 从 boot_info中读取acpi_table address
    interrupts::init(); // 初始化Trap frame和中断
    drivers::bus::pci::init();
    test!("This is test");
    error!("This is error");
    warn!("This is warn");
    debug!("This is debug");
    println!("This is println");
    fs::init();
    shell::init_shell();
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


fn init_log() {
    struct SimpleLogger;
    impl Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            println!("[{:>5}] {}", record.level(), record.args());
        }

        fn flush(&self) {}
    }

    static LOGGER: SimpleLogger = SimpleLogger;
    set_logger(&LOGGER).unwrap();
    set_max_level(LevelFilter::Trace);
}