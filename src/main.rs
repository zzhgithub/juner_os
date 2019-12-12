#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use linked_list_allocator::LockedHeap;

mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod lisp;
pub mod memory;
pub mod allocator;
pub mod stdio;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

// 函数入口
entry_point!(kernel_main);

fn kernel_main(boot_info:&'static BootInfo)-> !{
    use x86_64::structures::paging::mapper::MapperAllSizes;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;
    use memory::BootInfoFrameAllocator;
    println!("Hello World {}", ",my friends!");
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // new: initialize a mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map)};
    // init heap 初始化堆
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    test();
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    println!("init start");
    // 中断表初始化
    interrupts::init_idt();
    // 设置段表和 TSS
    gdt::init();
    // PICS(中断控制器) 初始化
    unsafe { interrupts::PICS.lock().initialize() };
    // 允许时间中断
    x86_64::instructions::interrupts::enable();
    
    println!("init end");
}

pub fn test(){
    // 测试输入打印循环
    lisp::lisp_repl();
    // use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
    //  // allocate a number on the heap
    //  let heap_value = Box::new(41);
    //  println!("heap_value at {:p}", heap_value);

    //  // create a dynamically sized vector
    //  let mut vec = Vec::new();
    //  for i in 0..500 {
    //      vec.push(i);
    //  }
    //  println!("vec at {:p}", vec.as_slice());
    //  // create a reference counted vector -> will be freed when count reaches 0
    //  let reference_counted = Rc::new(vec![1, 2, 3]);
    //  let cloned_reference = reference_counted.clone();
    //  println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    //  core::mem::drop(reference_counted);
    //  println!("reference count is {} now", Rc::strong_count(&cloned_reference));
}
