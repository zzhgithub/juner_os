use alloc::rc::Rc;
use alloc::string::{String,ToString};
use crate::mal::types::MalErr::{ErrString,ErrMalVal};
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use crate::mal::types::MalVal::{Hash,Str,Nil,Func,Bool,Int,Sym,List,Vector,MalFunc,Atom};
use core::fmt;
use crate::mal::env::{Env,env_bind};

#[derive(Debug,Clone)]
pub enum MalVal{
    Nil,
    Bool(bool), //布尔类型
    Int(i64),   // int类型
    Str(String), // 字符串类型
    Sym(String),
    List(Rc<Vec<MalVal>>, Rc<MalVal>),  // 列表类型
    Vector(Rc<Vec<MalVal>>, Rc<MalVal>), // 向量类型
    Hash(Rc<HashMap<String,MalVal>>,Rc<MalVal>), // hashMap 类型
    Func(fn(MalArgs) -> MalRet,Rc<MalVal>), //函数 相当于 lambda (x)-> M
    MalFunc {
        eval: fn(ast: MalVal, env: Env) -> MalRet,
        ast: Rc<MalVal>, // 函数 抽象语法树
        env: Env,    // repl 环境
        params: Rc<MalVal>,  // 参数值  TODO： 其实可以单值然后用柯里化
        is_macro: bool,    // 是否是宏
        meta: Rc<MalVal>,   // 元数据
    },
    Atom(Rc<RefCell<MalVal>>) //原子
}

// Mal 报错结构
#[derive(Debug)]
pub enum MalErr {
    ErrString(String),
    ErrMalVal(MalVal),
}

// Mal 入参
pub type MalArgs = Vec<MalVal>;
// Mal 出参结构
pub type MalRet = Result<MalVal, MalErr>;

#[macro_export]
macro_rules! list {
    ($seq:expr) => {{
      List(Rc::new($seq),Rc::new(Nil))
    }};
    [$($args:expr),*] => {{
      let v: Vec<MalVal> = vec![$($args),*];
      List(Rc::new(v),Rc::new(Nil))
    }}
}

#[macro_export]
macro_rules! vector {
    ($seq:expr) => {{
      Vector(Rc::new($seq),Rc::new(Nil))
    }};
    [$($args:expr),*] => {{
      let v: Vec<MalVal> = vec![$($args),*];
      Vector(Rc::new(v),Rc::new(Nil))
    }}
}

#[macro_export]
macro_rules! vec {
    ($elem:expr;$n:expr) => {
        $crate::alloc::vec::from_elem($elem, n)
    };
    ($($x:expr),*) => {
        <[_]>::into_vec(box[$($x),*])
    };
    ($($x:expr,)*) => {$crate::vec![$($x),*]}
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => ($crate::alloc::fmt::format(format_args!($($arg)*)))
}

// type utility functions
  
//抛出错误
pub fn error(s: &str) -> MalRet {
    Err(ErrString(s.to_string()))
}

//格式化错误输出
pub fn format_error(e: MalErr) -> String {
    match e {
        ErrString(s) => s.clone(),
        ErrMalVal(mv) => mv.pr_str(true),
    }
}

// 把参数 变成hashmap
pub fn _assoc(mut hm: HashMap<String, MalVal>, kvs: MalArgs) -> MalRet {
    if kvs.len() % 2 != 0 {
        return error("odd number of elements");
    }
    let mut itre = kvs.iter();
    loop{
        let k = itre.next();
        match k {
            Some(Str(s))=>{
                match itre.next() {
                    Some(v) => {
                        hm.insert(s.to_string(), v.clone());
                    },
                    // 这里应该永远也不会发生
                    None => return error("key to value,vlaue is not a MalVal"),
                }
            },
            None => break,
            _ => return error("key is not string"),
        }
    }
    Ok(Hash(Rc::new(hm), Rc::new(Nil)))
}

// 创建hashmap
pub fn hash_map(kvs: MalArgs) -> MalRet {
    let hm: HashMap<String, MalVal> = HashMap::new();
    _assoc(hm, kvs)
}

// 创建一个函数
pub fn func(f: fn(MalArgs) -> MalRet) -> MalVal {
    Func(f, Rc::new(Nil))
}

// 创造一个原子
pub fn atom(mv:&MalVal) ->MalVal {
    Atom(Rc::new(RefCell::new(mv.clone())))
}

// 实现比较方法 判断两个 MalVal 是否相等
impl PartialEq for MalVal {
    fn eq(&self, other: &MalVal) -> bool {
        match (self, other) {
            (Nil, Nil) => true,
            (Bool(ref a), Bool(ref b)) => a == b,
            (Int(ref a), Int(ref b)) => a == b,
            (Str(ref a), Str(ref b)) => a == b,
            (Sym(ref a), Sym(ref b)) => a == b,
            (List(ref a, _), List(ref b, _))
            | (Vector(ref a, _), Vector(ref b, _))
            | (List(ref a, _), Vector(ref b, _))
            | (Vector(ref a, _), List(ref b, _)) => a == b,
            (Hash(ref a, _), Hash(ref b, _)) => a == b,
            (MalFunc { .. }, MalFunc { .. }) => false, // 两个函数永远也不能相同！
            _ => false,
        }
    }
}

// 删除hash map 中指定的key, 并且返回 删除后的map,不改变原来的值
pub fn _dissoc(mut hm: HashMap<String, MalVal>, ks: MalArgs) -> MalRet {
    for k in ks.iter() {
        match k {
            Str(ref s) => {
                hm.remove(s);
            }
            _ => return error("key is not string"),
        }
    }
    Ok(Hash(Rc::new(hm), Rc::new(Nil)))
}

impl MalVal {

    pub fn apply(&self, args: MalArgs) -> MalRet {
        match *self {
            Func(f, _) => f(args),
            MalFunc {
                eval,
                ref ast,
                ref env,
                ref params,
                ..
            } => {
                let a = &**ast;
                let p = &**params;
                // 给环境中的变量绑定值 相当于形参绑定实参  然后返回一个子环境 注意这个子环境是可以追溯到母环境的绑定的
                let fn_env = env_bind(Some(env.clone()), p.clone(), args)?;
                Ok(eval(a.clone(), fn_env)?)
            }
            _ => error("attempt to call non-function"),
        }
    }
    
    // 获取一个原子所对应的值
    pub fn deref(&self) -> MalRet {
        match self {
            Atom(a) => Ok(a.borrow().clone()),
            _ => error("attempt to deref a non-Atom")
        }        
    }

    // 重新绑定原子， 使他指向这个新的ast对象，下面的new
    pub fn reset_bang(&self, new: &MalVal) -> MalRet {
        match self {
            Atom(a) => {
                *a.borrow_mut() = new.clone();
                Ok(new.clone())
            }
            _ => error("attempt to reset! a non-Atom"),
        }
    }

    // 对于一个atom 输入一个函数 然后把atom的值加在最开始作为输入使用进行求值 并且更新原子的值
    pub fn swap_bang(&self, args: &MalArgs) -> MalRet {
        match self {
            Atom(a) => {
                let f = &args[0];
                let mut fargs = args[1..].to_vec();
                fargs.insert(0, a.borrow().clone());
                *a.borrow_mut() = f.apply(fargs)?;
                Ok(a.borrow().clone())
            }
            _ => error("attempt to swap! a non-Atom"),
        }
    }

    // 判断对象是否为空
    pub fn empty_q(&self) -> MalRet {
        match self {
            List(l,_) | Vector(l,_) => Ok(Bool(l.len()==0)),
            Nil => Ok(Bool(true)),
            _ => error("invalid empty value!"),
        }
    }

    pub fn count(&self) -> MalRet {
        match self{
            List(l, _) | Vector(l, _) => Ok(Int(l.len() as i64)),
            Nil => Ok(Int(0)),
            _ => error("invalid type for count"),
        }
    }

    // 将Str转换成关键字
    pub fn keyword(&self) -> MalRet {
        match self {
            Str(s) if s.starts_with("\u{29e}") => Ok(Str(s.to_string())),
            Str(s) => Ok(Str(format!("\u{29e}{}", s))),
            _ => error("invalid type for keyword"),
        }
    }

}
