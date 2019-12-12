use lazy_static::lazy_static;
use crate::println;
use crate::print;
use crate::stdio;

pub const BUFFER_SIZE: u32 = 1024;

pub fn lisp_repl(){
    println!("Welcome to the LISP REPL:");
    print!("Kernel>>");
    loop {
        let glance:u8 = stdio::get_char();
        let glance_char = glance as char;
        print!("{}",glance_char);
    };
}
