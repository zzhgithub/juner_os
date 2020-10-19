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

use bootloader::{entry_point, BootInfo};
use task::{executor::Executor, Task};
entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    memory::heap::init_heap();
    drivers::init_driver(boot_info);
    memory::init_frame(boot_info);
    interrupts::init();
    test!("This is test");
    error!("This is error");
    warn!("This is warn");
    debug!("This is debug");
    println!("This is println");
    let mut executor = Executor::new();
    executor.spawn(Task::new(task::keyboard::print_keypresses()));
    executor.run();
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
