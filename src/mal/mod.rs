use crate::println;
use crate::alloc::string::{String,ToString};
use crate::mal::types::format_error;
use crate::mal::reader::read_str;

pub mod types;
pub mod reader;
pub mod env;
pub mod printer;

pub fn repl(){
    loop{
        //todo
    }
}

// 输入-求值-打印 不循环
pub fn rep(code:&str) {
    let ast = read_str(code.to_string());
    match ast {
        Ok(s)=>{println!("{}",s.pr_str(true))},
        Err(e)=>{println!("{}",format_error(e))}
    }
}
