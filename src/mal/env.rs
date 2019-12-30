use alloc::rc::Rc;
use alloc::string::{String,ToString};
use core::cell::RefCell;
use hashbrown::HashMap;

use crate::mal::types::MalErr::ErrString;
use crate::mal::types::MalVal::{List, Nil, Sym, Vector};
use crate::mal::types::{error, MalErr, MalRet, MalVal};

use crate::format;

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

// 在环境中绑定符号
pub fn env_sets(env:&Env,key:&str,val:MalVal){
    env.data.borrow_mut().insert(key.to_string(),val);
}