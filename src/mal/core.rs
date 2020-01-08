// mal 语言核心库
use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::{String,ToString};
use crate::mal::env::Env;
use crate::mal::types::{MalVal,MalArgs,error,func};
use crate::mal::types::MalVal::{Int,Str,Bool,Nil,List};
use crate::mal::env::{env_set,env_sets};
use crate::mal::rep;
use crate::vec;
use crate::list;
use crate::mal::reader::read_str;

// 处理两个值入参
macro_rules! fn_t_int_int {
    ($ret:ident, $fn:expr) => {{
        |a: MalArgs| match (a[0].clone(), a[1].clone()) {
            (Int(a0), Int(a1)) => Ok($ret($fn(a0, a1))),
            _ => error("expecting (int,int) args"),
        }
    }};
}

macro_rules! fn_str {
    ($fn:expr) => {{
        |a: MalArgs| match a[0].clone() {
            Str(a0) => $fn(a0),
            _ => error("expecting (str) arg"),
        }
    }};
}


pub fn ns() -> Vec<(&'static str,MalVal)> {
    vec![
        ("=", func(|a| Ok(Bool(a[0] == a[1])))),
        ("read-string",func(fn_str!(|s| {read_str(s)}))),
        ("list",func(|a| Ok(list!(a)))),
        ("<", func(fn_t_int_int!(Bool, |i, j| { i < j }))),
        ("<=", func(fn_t_int_int!(Bool, |i, j| { i <= j }))),
        (">", func(fn_t_int_int!(Bool, |i, j| { i > j }))),
        (">=", func(fn_t_int_int!(Bool, |i, j| { i >= j }))),
        ("+", func(fn_t_int_int!(Int, |i, j| { i + j }))),
        ("-", func(fn_t_int_int!(Int, |i, j| { i - j }))),
        ("*", func(fn_t_int_int!(Int, |i, j| { i * j }))),
        ("/", func(fn_t_int_int!(Int, |i, j| { i / j }))),
    ]
}

fn mal() ->Vec<&'static str> {
    vec![
        "(def! not (lamdba (a) (if a false true)))",
    ]
}


// 加载核心函数 使用rust进行定义
pub fn load_core(env:&Env) {
    for (k,v) in ns() {
        env_sets(&env, k, v);
    }
    load_core_lib(&env);
}

// 加载mal核心函数库
// 这个库的目的是使用mal 语言自己实现扩展
fn load_core_lib(env:&Env) {
    for s in mal() {
        let _ = rep(s,&env);
    }
}