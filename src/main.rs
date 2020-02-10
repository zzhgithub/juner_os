#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(box_syntax)]
extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use linked_list_allocator::LockedHeap;
use log::*;

mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod allocator;
pub mod stdio;
pub mod serial;
pub mod mal;

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

// 函数入口
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
    // init log
    init_log();
    // PICS(中断控制器) 初始化
    unsafe { interrupts::PICS.lock().initialize() };
    // 允许时间中断
    x86_64::instructions::interrupts::enable();
    
    println!("init end");
}


fn init_log(){
    struct SimpleLogger;
    impl Log for SimpleLogger {
        fn enabled(&self,metadata: &Metadata)-> bool{
            true
        }

        fn log(&self,record: &Record){
            println!("[{:>5}] {}",record.level(),record.args());
        }
        
        fn flush(&self){
        }
    }

    static LOGGER: SimpleLogger = SimpleLogger;
    set_logger(&LOGGER).unwrap();
    set_max_level(LevelFilter::Trace);
}


pub fn test(){
    use crate::mal::rep;
    use crate::mal::env::Env;
    use crate::mal::env::{env_new,env_sets};
    use crate::mal::types::MalArgs;
    use crate::mal::types::func;// 这个方法可以用 rust的闭包生成一个lisp的函数
    use crate::mal::types::format_error;
     
    // FIXME: 这里的注释识别有一些问题！！！ 不能正常的识别双引号
    let kernel_env:Env = env_new(None);
    use crate::mal::core::load_core;
    load_core(&kernel_env);
    
    let code = vec![
        // "(= 2 2)",
        // "(def! plus3 (lamdba [x] (+ 3 x)))",
        // "(plus3 3)",
        // "(not false)",
        // "(do (+ 1 2) (* 3 3) 5)",
        // "\"Lisp is so
        // good  for me\"",
        // "(eval (read-string \"(+ 1 3)\"))",
        // "(def! test-eval (list + 3 3))",
        // "(eval test-eval)",
        // "(prn abc)",
        // "(prn (quote abc))",
        // "(prn 'abc)",
        // "(def! lst '(2 3))",
        // "(quasiquote (1 (unquote lst)))",
        // "(quasiquote (1 (splice-unquote lst)))",
        // "`(1 ~lst)",
        // "`(1 ~@lst)",
        // "(cons [1] [2 3])",
        // "(cons 1 [2 3])",
        // "(concat [1 2] (list 3 4) [5 6])",
        // "(concat [1 2])"
        // "(defmacro! unless (lamdba (pred a b) `(if ~pred ~b ~a)))",
        // "(unless false 7 8)",
        // "(macroexpand (unless false 7 8))"
        // "(nth [1 2 3] 0)",
        // "(nth '(1 2 3) 1)",
        // "(first '((1 2) 2 3))",
        "(count '(1 2 (2 3)))",
        "(count [1 2 3])",
        "(empty? '())",
        "(empty? nil)",
    ];

    for line in code {
        match rep(line,&kernel_env){
            Ok(out) => println!("{}",out),
            Err(e) => println!("{}",format_error(e)),
        }
    }
    
}
