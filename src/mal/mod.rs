use crate::println;
use crate::alloc::string::{String,ToString};
use crate::mal::types::format_error;
use crate::mal::reader::read_str;

pub mod types;
pub mod reader;
pub mod env;
pub mod printer;

use crate::mal::types::MalVal;
use crate::mal::types::MalVal::{List};
use crate::mal::types::MalRet;
use crate::mal::env::Env;

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

// 求值
fn eval(ast: MalVal, env: Env) -> MalRet {
    match ast.clone(){
        List(l,_)=>{
            if l.len() == 0 {
                return ast;
            }
            let a0 = &l[0];
            // todo 处理list 调用第一个符号
        },
        //todo
    }
}
