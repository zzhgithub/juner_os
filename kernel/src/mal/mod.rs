use crate::alloc::string::{String,ToString};
use crate::mal::reader::read_str;
use hashbrown::HashMap;
use alloc::rc::Rc;
use alloc::vec::Vec;


pub mod types;
pub mod reader;
pub mod env;
pub mod printer;
pub mod core;

use crate::mal::types::MalVal::{List,Sym,Str,Vector,Hash,Nil,Int,MalFunc,Bool,Func};
use crate::mal::types::{error,MalRet,MalArgs,MalVal,MalErr};
use crate::mal::types::MalErr::{ErrMalVal,ErrString};
use crate::mal::env::Env;
use crate::mal::env::{env_get,env_set,env_new,env_bind,env_find};
use crate::vec;
use crate::vector;
use crate::list;

// 输入-求值-打印 不循环
pub fn rep(str: &str, env: &Env) -> Result<String, MalErr> {
    let ast = read_str(str.to_string())?;
    let exp = eval(ast, env.clone())?;
    Ok(exp.pr_str(true))
}

// 对符号列表支持临时求值的 (quote 的升级版)
fn quasiquote(ast: &MalVal) -> MalVal {
    match ast {
        List(ref v,_) | Vector(ref v, _) if v.len() > 0 => {
            let a0 = &v[0];
            match a0 {
                Sym(ref s) if s == "unquote" => v[1].clone(),
                _ => match a0 {
                    List(ref v0,_) | Vector(ref v0,_) if v0.len() > 0 => match v0[0] {
                        Sym(ref s) if s == "splice-unquote" => list![
                            Sym("concat".to_string()),
                            v0[1].clone(),
                            quasiquote(&list!(v[1..].to_vec()))
                        ],
                        _ => list![
                            Sym("cons".to_string()),
                            quasiquote(a0),
                            quasiquote(&list!(v[1..].to_vec()))
                        ],
                    },
                    _ => list![
                        Sym("cons".to_string()),
                        quasiquote(a0),
                        quasiquote(&list!(v[1..].to_vec()))
                    ],
                },
            }
        }
        _ => list![Sym("quote".to_string()),ast.clone()]
    }
}

//是否是宏调用 并且返回AST的入参
fn is_macro_call(ast: &MalVal, env: &Env) -> Option<(MalVal, MalArgs)> {
    match ast {
        List(v, _) => match v[0] {
            Sym(ref s) => match env_find(env, s) {
                Some(e) => match env_get(&e, &v[0]) {
                    Ok(f @ MalFunc { is_macro: true, .. }) => Some((f, v[1..].to_vec())),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

// 宏展开函数
fn macroexpand(mut ast: MalVal, env: &Env) -> (bool, MalRet) {
    let mut was_expanded = false;
    while let Some((mf,args)) = is_macro_call(&ast, env) {
        ast = match mf.apply(args) {
            Err(e) => return (false, Err(e)),
            Ok(a) => a,
        };
        was_expanded = true;
    }
    (was_expanded,Ok(ast))
}


// 求值
fn eval(mut ast: MalVal,mut env: Env) -> MalRet {
    let ret:MalRet;
    'tco: loop {
        ret = match ast.clone(){
            List(l,_)=>{
                if l.len() == 0 {
                    return Ok(ast);
                }
                // 展开尝试并且求值
                match macroexpand(ast.clone(), &env) {
                    (true,Ok(new_ast)) => {
                        ast = new_ast;
                        continue 'tco;
                    }
                    (_,Err(e)) => return Err(e),
                    _ => (), // 理论上不会到这个分支
                }
                let a0 = &l[0];
                match a0 {
                    Sym(ref a0sym) if a0sym == "def!" => {
                        env_set(&env, l[1].clone(), eval(l[2].clone(), env.clone())?)
                    },
                    Sym(ref a0sym) if a0sym == "let*" => {
                        // 对let* 语法进行支持
                        env = env_new(Some(env.clone()));
                        let (a1,a2) = (l[1].clone(),l[2].clone());
                        match a1 {
                            List(ref binds,_) | Vector(ref binds,_) => {
                                let mut binds_iter = binds.iter();
                                'letloop: loop {
                                    match binds_iter.next(){
                                        Some(b) =>{
                                            match binds_iter.next() {
                                                Some(e) => {
                                                    let _ = env_set(
                                                        &env,
                                                        b.clone(),
                                                        eval(e.clone(), env.clone())?
                                                    );
                                                },
                                                None => {
                                                    return error("let* with non-Sym binding");
                                                }
                                            }
                                        },
                                        None => {
                                            break 'letloop;
                                        },
                                    }
                                }
                            },
                            _ => {
                                return error("let* with non-List bindings");
                            },
                        }
                        ast = a2;
                        continue 'tco;
                    }
                    // 定义闭包函数的语法
                    Sym(a0sym) if a0sym == "lambda" => {
                        let (a1,a2) = (l[1].clone(),l[2].clone());
                        Ok(MalFunc {
                            eval: eval,
                            ast: Rc::new(a2),
                            env: env,
                            params: Rc::new(a1),
                            is_macro: false,
                            meta: Rc::new(Nil),
                        })
                    },
                    Sym(ref a0sym) if a0sym == "if" => {
                        let cond = eval(l[1].clone(), env.clone())?;
                        match cond {
                            Bool(false) | Nil if l.len() >= 4 => {
                                ast = l[3].clone();
                                continue 'tco;
                            },
                            Bool(false) | Nil => Ok(Nil),
                            _ if l.len() >= 3 => {
                                ast = l[2].clone();
                                continue 'tco;
                            },
                            _ => Ok(Nil),
                        }
                    },
                    Sym(ref a0sym) if a0sym == "do" => {
                        match eval_ast(&list!(l[1..].to_vec()),&env)?{
                            List(_,_) => {
                                ast = l.last().unwrap_or(&Nil).clone();
                                continue 'tco;
                            }
                            _ => error("invalid do form"),
                        }
                    }
                    Sym(ref a0sym) if a0sym == "quote" => Ok(l[1].clone()),
                    Sym(ref a0sym) if a0sym == "quasiquote" => {
                        ast = quasiquote(&l[1]);
                        continue 'tco;
                    },
                    Sym(ref a0sym) if a0sym == "eval" =>{
                        ast = eval(l[1].clone(), env.clone())?;
                        while let Some(ref e) = env.clone().outer {
                            env = e.clone();
                        }
                        continue 'tco;
                    },
                    // todo 这里实现其他的符号逻辑
                    Sym(ref a0sym) if a0sym == "try*" => match eval(l[1].clone(), env.clone()) {
                        Err(ref e) if l.len() >= 3 => {
                            let exc = match e {
                                ErrMalVal(mv) => mv.clone(),
                                ErrString(s) => Str(s.to_string()),
                            };
                            match l[2].clone() {
                                List(c,_) => {
                                    let catch_env = env_bind(
                                        Some(env.clone()),
                                        list![vec![c[1].clone()]],
                                        vec![exc],
                                    )?;
                                    eval(c[2].clone(), catch_env)
                                },
                                _ => error("invalid catch b,lock"),
                            }
                        }
                        res => res,
                    },
                    // 进行宏定义
                    Sym(ref a0sym) if a0sym == "defmacro!" => {
                        let (a1,a2) = (l[1].clone(),l[2].clone());
                        let r = eval(a2, env.clone())?;
                        match r {
                            MalFunc {
                                eval,
                                ast,
                                env,
                                params,
                                ..
                            }=> Ok(env_set(&env, a1.clone(), MalFunc {
                                eval:eval,
                                ast:ast.clone(),
                                env:env.clone(),
                                params:params.clone(),
                                is_macro:true,
                                meta: Rc::new(Nil),
                                // mate 的作用是什么？
                            })?),
                            _ => error("set_macro on non-function"),
                        }
                    },
                    // 进行宏展开
                    Sym(ref a0sym) if a0sym == "macroexpand" => {
                        match macroexpand(l[1].clone(), &env) {
                            (_, Ok(new_ast)) => Ok(new_ast),
                            (_, e) => return e,
                        }
                    },
                    _ => match eval_ast(&ast, &env)? {
                        List(ref el, _) => {
                            let ref f = el[0].clone();
                            let args = el[1..].to_vec();
                            match f {
                                Func(_,_) => f.apply(args),
                                MalFunc{
                                    ast: mast,
                                    env: menv,
                                    params,
                                    ..
                                } => {
                                    let a = &**mast;
                                    let p = &**params;
                                    env = env_bind(Some(menv.clone()), p.clone(), args)?;
                                    ast = a.clone();
                                    continue 'tco;
                                },
                                _ => error("attempt to call non-function"),
                            }
                        }
                        _ => error("expected a list"),
                    }
                }
            },
            _ => eval_ast(&ast, &env),
            };
            break 'tco;
    }
    ret
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

// 这是个临时的方法其实已经不需要了
// 把一个MalArgs 如此作用于一个rust的fn
pub fn int_op(op: fn(i64, i64) -> i64, a: MalArgs) -> MalRet {
    match (a[0].clone(), a[1].clone()) {
        (Int(a0), Int(a1)) => Ok(Int(op(a0, a1))),
        _ => error("invalid int_op args"),//fixme 函数的运算至少要有两个值 但是应该支持多个值 不要着急 还没有实现宏
    }
}