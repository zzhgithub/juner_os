use crate::println;
use crate::alloc::string::{String,ToString};
use crate::mal::types::format_error;
use crate::mal::reader::read_str;
use hashbrown::HashMap;
use alloc::rc::Rc;


pub mod types;
pub mod reader;
pub mod env;
pub mod printer;

use crate::mal::types::MalVal::{List,Sym,Str,Vector,Hash,Nil,Int};
use crate::mal::types::{error,MalRet,MalArgs,MalVal,MalErr};
use crate::mal::env::Env;
use crate::mal::env::env_get;
use crate::vec;
use crate::vector;
use crate::list;

pub fn repl(){
    loop{
        //todo
    }
}

// 输入-求值-打印 不循环
pub fn rep(str: &str, env: &Env) -> Result<String, MalErr> {
    let ast = read_str(str.to_string())?;
    let exp = eval(ast, env.clone())?;
    Ok(exp.pr_str(true))
}

// 求值
fn eval(ast: MalVal, env: Env) -> MalRet {
    match ast.clone(){
        List(l,_)=>{
            if l.len() == 0 {
                return Ok(ast);
            }
            let a0 = &l[0];
            match a0 {
                Sym(ref a0sym) if a0sym == "def!" => {
                    //todo 这里要处理定义一个函数的问题
                    Ok(Nil)
                },
                Sym(ref a0sym) if a0sym == "let*" => {
                    Ok(Nil)
                }
                // todo 这里实现其他的符号逻辑
                _ => match eval_ast(&ast, &env)? {
                    List(ref el, _) => {
                        let ref f = el[0].clone();
                        f.apply(el[1..].to_vec())
                    }
                    _ => error("expected a list"),
                }
            }
        },
        _ => eval_ast(&ast, &env),
    }
}

// 对下级的AST求值
fn eval_ast(ast: &MalVal, env: &Env) -> MalRet {
    match ast {
        Sym(_) => Ok(env_get(&env, &ast)?),
        List(v,_) => {
            let mut lst:MalArgs = vec![];
            for a in v.iter() {
                lst.push(eval(a.clone(),env.clone())?)
            }
            Ok(list!(lst))
        },
        Vector(v,_) => {
            let mut lst:MalArgs = vec![];
            for a in v.iter() {
                lst.push(eval(a.clone(),env.clone())?)
            }
            Ok(vector!(lst))
        },
        Hash(hm,_) => {
            let mut new_hm:HashMap<String,MalVal> = HashMap::default();
            for (k,v) in hm.iter() {
                new_hm.insert(k.to_string(), eval(v.clone(),env.clone())?);
            }
            Ok(Hash(Rc::new(new_hm),Rc::new(Nil)))
        },
        _ => Ok(ast.clone()),
    }
}

// 把一个MalArgs 如此作用于一个rust的fn
pub fn int_op(op: fn(i64, i64) -> i64, a: MalArgs) -> MalRet {
    match (a[0].clone(), a[1].clone()) {
        (Int(a0), Int(a1)) => Ok(Int(op(a0, a1))),
        _ => error("invalid int_op args"),//fixme 函数的运算至少要有两个值 但是应该支持多个值 不要着急 还没有实现宏
    }
}