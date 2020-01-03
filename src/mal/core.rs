// mal 语言核心库
use alloc::vec::Vec;
use crate::mal::env::Env;
use crate::mal::types::{MalVal,MalArgs,error,func};
use crate::mal::types::MalVal::{Int};
use crate::mal::env::{env_set,env_sets};
use crate::mal::rep;
use crate::vec;

// 处理两个值入参
macro_rules! fn_t_int_int {
    ($ret:ident, $fn:expr) => {{
        |a: MalArgs| match (a[0].clone(), a[1].clone()) {
            (Int(a0), Int(a1)) => Ok($ret($fn(a0, a1))),
            _ => error("expecting (int,int) args"),
        }
    }};
}

pub fn ns() -> Vec<(&'static str,MalVal)> {
    vec![
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