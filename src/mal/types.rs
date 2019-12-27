use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;

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
    Func(fn(MalArgs) -> MalRet,Rc<MalVal>), //函数 相当于 lamdba (x)-> M
    MalFunc {
        // eval: fn(ast: MalVal, env: Env) -> MalRet,
        ast: Rc<MalVal>, // 函数 抽象语法树
        // env: Env,    // repl 环境
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