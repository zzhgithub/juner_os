use alloc::rc::Rc;
use alloc::string::{String,ToString};
use core::cell::RefCell;
use hashbrown::HashMap;
use alloc::vec::Vec;

use crate::mal::types::MalErr::ErrString;
use crate::mal::types::MalVal::{List, Nil, Sym, Vector,Func,Int};
use crate::mal::types::{error, MalErr, MalRet, MalVal};

use crate::format;
use crate::list;

#[derive(Debug)]
pub struct EnvSturct {
    data: RefCell<HashMap<String,MalVal>>,
    pub outer: Option<Env>,
}

pub type Env = Rc<EnvSturct>;

pub fn env_new(outer: Option<Env>)->Env{
    Rc::new(EnvSturct{
        data: RefCell::new(HashMap::default()),
        outer: outer,
    })
}

// 查找符号所在环境
pub fn env_find(env: &Env, key: &str) -> Option<Env> {
    match (env.data.borrow().contains_key(key), env.outer.clone()) {
        (true, _) => Some(env.clone()),
        (false, Some(o)) => env_find(&o, key),
        _ => None,
    }
}

// 再环境中查找符号
pub fn env_get(env: &Env, key: &MalVal) -> MalRet {
    match key {
        Sym(ref s) => match env_find(env, s) {
            Some(e) => Ok(e
                .data
                .borrow()
                .get(s)
                .ok_or(ErrString(format!("'{}' not found", s)))?
                .clone()),
            _ => error(&format!("'{}' not found", s)),
        },
        _ => error("Env.get called with non-Str"),
    }
}

// 在环境中绑定符号 MalVal是move进来的 适合使用Rust直接定义函数
pub fn env_sets(env:&Env,key:&str,val:MalVal){
    env.data.borrow_mut().insert(key.to_string(),val);
}


// 给环境中的符号绑定 抽象语法树！
pub fn env_set(env: &Env, key: MalVal, val: MalVal) -> MalRet {
    match key {
        Sym(ref s) => {
            env.data.borrow_mut().insert(s.to_string(), val.clone());
            Ok(val)
        }
        _ => error("Env.set called with non-Str"),
    }
}

// 给符号绑定 对应的抽象语法树 并且返回一个待使用的Env子环境， 类似与形参绑定实参 
pub fn env_bind(outer: Option<Env>, mbinds: MalVal, exprs: Vec<MalVal>) -> Result<Env, MalErr> {
    let env = env_new(outer);
    match mbinds {
        List(binds, _) | Vector(binds, _) => {
            for (i, b) in binds.iter().enumerate() {
                match b {
                    // 这个特性保证 & 后面的其他参数都进入到一个list里面
                    Sym(s) if s == "&" => {
                        env_set(&env, binds[i + 1].clone(), list!(exprs[i..].to_vec()))?;
                        break;
                    }
                    _ => {
                        env_set(&env, b.clone(), exprs[i].clone())?;
                    }
                }
            }
            Ok(env)
        }
        _ => Err(ErrString("env_bind binds not List/Vector".to_string())),
    }
}
