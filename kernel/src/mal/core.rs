// mal 语言核心库
use crate::list;
use crate::mal::env::Env;
use crate::mal::env::{env_set, env_sets};
use crate::mal::printer::pr_seq;
use crate::mal::reader::read_str;
use crate::mal::rep;
use crate::mal::types::MalErr::{ErrMalVal, ErrString};
use crate::mal::types::MalVal::{
    Atom, Bool, Func, Hash, Int, List, MalFunc, Nil, Str, Sym, Vector,
};
use crate::mal::types::{MalArgs, MalRet, MalVal, _assoc, _dissoc, atom, error, func, hash_map};
use crate::vec;
use crate::vector;

use crate::fs::{inode_ext::INodeExt, ROOT_INODE};
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use log::*;

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

macro_rules! fn_is_type {
    ($($ps:pat),*) => {{
        |a:MalArgs| {Ok(Bool(match a[0] {$($ps => true,)* _=>false}))}
    }};
    ($p:pat if $e:expr) => {{
        |a:MalArgs| { Ok(Bool(match a[0] { $p if $e => true, _ => false})) }
    }};
    ($p:pat if $e:expr,$($ps:pat),*) => {{
        |a:MalArgs| { Ok(Bool(match a[0] { $p if $e => true, $($ps => true,)* _ => false})) }
    }};
}

fn cons(a: MalArgs) -> MalRet {
    match a[1].clone() {
        List(v, _) | Vector(v, _) => {
            let mut new_v = vec![a[0].clone()];
            new_v.extend_from_slice(&v);
            Ok(list!(new_v.to_vec()))
        }
        _ => error("cons expects seq as second arg"),
    }
}

fn concat(a: MalArgs) -> MalRet {
    let mut new_v = vec![];
    for seq in a.iter() {
        match seq {
            List(v, _) | Vector(v, _) => new_v.extend_from_slice(v),
            _ => return error("non-seq passed to concat"),
        }
    }
    Ok(list!(new_v.to_vec()))
}

fn nth(a: MalArgs) -> MalRet {
    match (a[0].clone(), a[1].clone()) {
        (List(seq, _), Int(idx)) | (Vector(seq, _), Int(idx)) => {
            if seq.len() <= idx as usize {
                return error("nth:index out of range");
            }
            Ok(seq[idx as usize].clone())
        }
        _ => error("invalid args to nth"),
    }
}

fn first(a: MalArgs) -> MalRet {
    match a[0].clone() {
        List(ref seq, _) | Vector(ref seq, _) if seq.len() == 0 => Ok(Nil),
        List(ref seq, _) | Vector(ref seq, _) => Ok(seq[0].clone()),
        Nil => Ok(Nil),
        _ => error("invalid args to first"),
    }
}

fn rest(a: MalArgs) -> MalRet {
    match a[0].clone() {
        List(ref seq, _) | Vector(ref seq, _) => {
            if seq.len() > 1 {
                Ok(list![seq[1..].to_vec()])
            } else {
                Ok(list![])
            }
        }
        Nil => Ok(list![]),
        _ => error("invalid args to list"),
    }
}

fn apply(a: MalArgs) -> MalRet {
    match a[a.len() - 1] {
        List(ref v, _) | Vector(ref v, _) => {
            let f = &a[0];
            let mut fargs = a[1..a.len() - 1].to_vec();
            // Q: 这个的extend_from_slice的方法的文档
            fargs.extend_from_slice(&v);
            f.apply(fargs)
        }
        _ => error("apply called with no-seq"),
    }
}

// 生成一个符号
fn symbol(a: MalArgs) -> MalRet {
    match a[0] {
        Str(ref s) => Ok(Sym(s.to_string())),
        _ => error("illegal symbol call"),
    }
}

fn map(a: MalArgs) -> MalRet {
    match a[1] {
        List(ref v, _) | Vector(ref v, _) => {
            let mut res = vec![];
            for mv in v.iter() {
                res.push(a[0].apply(vec![mv.clone()])?);
            }
            Ok(list!(res))
        }
        _ => error("map called with no-seq"),
    }
}

// 向hash map 中添加新的 key-value不改变原理的值 返回新的hashmap
fn assoc(a: MalArgs) -> MalRet {
    match a[0] {
        Hash(ref hm, _) => _assoc((**hm).clone(), a[1..].to_vec()),
        _ => error("assoc on non-Hash Map"),
    }
}

fn dissoc(a: MalArgs) -> MalRet {
    match a[0] {
        Hash(ref hm, _) => _dissoc((**hm).clone(), a[1..].to_vec()),
        _ => error("dissoc on non-Hash Map"),
    }
}

// 通过关键字获取 vlaue的值
fn get(a: MalArgs) -> MalRet {
    match (a[0].clone(), a[1].clone()) {
        (Nil, _) => Ok(Nil),
        (Hash(ref hm, _), Str(ref s)) => match hm.get(s) {
            Some(mv) => Ok(mv.clone()),
            None => Ok(Nil),
        },
        _ => error("illegal get args"),
    }
}

// hash map 是否包含某个key
fn contains_q(a: MalArgs) -> MalRet {
    match (a[0].clone(), a[1].clone()) {
        (Hash(ref hm, _), Str(ref s)) => Ok(Bool(hm.contains_key(s))),
        _ => error("illefal contains args"),
    }
}

fn keys(a: MalArgs) -> MalRet {
    match a[0] {
        Hash(ref hm, _) => Ok(list!(hm.keys().map(|k| { Str(k.to_string()) }).collect())),
        _ => error("keys requires Hash Map"),
    }
}

fn vals(a: MalArgs) -> MalRet {
    match a[0] {
        Hash(ref hm, _) => Ok(list!(hm.values().map(|v| { v.clone() }).collect())),
        _ => error("vals requires Hash Map"),
    }
}

fn read_file(a: MalArgs) -> MalRet {
    match &a[0] {
        Str(path) => {
            let rs = ROOT_INODE
                .lookup(path.clone().as_str())
                .unwrap()
                .read_as_string()
                .unwrap();
            Ok(Str(rs))
        }
        _ => error("read_file requires path String"),
    }
}

fn ls_dir(a: MalArgs) -> MalRet {
    if a.len() > 0 {
        match &a[0] {
            Str(name) => {
                // FIXME 处理这段函数
                let list = ROOT_INODE.lookup(name).unwrap().ls_as_vec().unwrap();
                let rs: Vec<MalVal> = list.iter().map(|v| Str(v.to_string()) as MalVal).collect();
                Ok(list!(rs.to_vec()))
            }
            _ => error("ls requires a path string!"),
        }
    } else {
        let tmp = ROOT_INODE.ls_as_vec().unwrap();
        let tmp_rs: Vec<MalVal> = tmp.iter().map(|v| Str(v.to_string()) as MalVal).collect();
        Ok(list!(tmp_rs.to_vec()))
    }
}

pub fn ns() -> Vec<(&'static str, MalVal)> {
    vec![
        ("=", func(|a| Ok(Bool(a[0] == a[1])))),
        ("read-string", func(fn_str!(|s| { read_str(s) }))),
        ("list", func(|a| Ok(list!(a)))),
        ("<", func(fn_t_int_int!(Bool, |i, j| { i < j }))),
        ("<=", func(fn_t_int_int!(Bool, |i, j| { i <= j }))),
        (">", func(fn_t_int_int!(Bool, |i, j| { i > j }))),
        (">=", func(fn_t_int_int!(Bool, |i, j| { i >= j }))),
        ("+", func(fn_t_int_int!(Int, |i, j| { i + j }))),
        ("-", func(fn_t_int_int!(Int, |i, j| { i - j }))),
        ("*", func(fn_t_int_int!(Int, |i, j| { i * j }))),
        ("/", func(fn_t_int_int!(Int, |i, j| { i / j }))),
        (
            "prn",
            func(|a| {
                print!(93; "{}",pr_seq(&a, true, "", "", ""));
                Ok(Nil)
            }),
        ),
        ("cons", func(cons)),
        ("concat", func(concat)),
        ("nth", func(nth)),
        ("first", func(first)),
        ("rest", func(rest)),
        ("count", func(|x| x[0].count())), // 获取列表 或者向量的长度
        ("empty?", func(|a| a[0].empty_q())), // 判断一个符号是否为空
        ("throw", func(|a| Err(ErrMalVal(a[0].clone())))), // 主动的抛出异常
        ("apply", func(apply)),
        ("map", func(map)),
        ("nil?", func(fn_is_type!(Nil))),
        ("ture?", func(fn_is_type!(Bool(true)))),
        ("false?", func(fn_is_type!(Bool(false)))),
        ("symbol?", func(fn_is_type!(Sym(_)))),
        // 原子操作
        ("atom", func(|a| Ok(atom(&a[0])))),
        ("atom?", func(fn_is_type!(Atom(_)))),
        ("reset!", func(|a| a[0].reset_bang(&a[1]))),
        ("deref", func(|a| a[0].deref())),
        ("swap!", func(|a| a[0].swap_bang(&a[1..].to_vec()))),
        // 生成一个符号
        ("symbol", func(symbol)),
        // 生成一个关键字 一个关键字是一个:开头的字符串!
        ("keyword", func(|a| a[0].keyword())),
        (
            "keyword?",
            func(fn_is_type!(Str(ref s) if s.starts_with("\u{29e}"))),
        ),
        // 判断是否是整形数字
        ("number?", func(fn_is_type!(Int(_)))),
        (
            "lambda?",
            func(fn_is_type!(MalFunc{is_macro,..} if !is_macro,Func(_,_))),
        ),
        (
            "macro?",
            func(fn_is_type!(MalFunc{is_macro,..} if is_macro)),
        ),
        // 生成字符串并且打印
        ("pr-str", func(|a| Ok(Str(pr_seq(&a, true, "", "", " "))))),
        // 生成字符串不进行打印
        ("str", func(|a| Ok(Str(pr_seq(&a, false, "", "", ""))))),
        // 判断一个符号是否是 列表 或者 向量
        ("sequential?", func(fn_is_type!(List(_, _), Vector(_, _)))),
        ("list", func(|a| Ok(list!(a)))),
        ("list?", func(fn_is_type!(List(_, _)))),
        ("vector", func(|a| Ok(vector!(a)))),
        ("vector?", func(fn_is_type!(Vector(_, _)))),
        // 哈希表支持的方法
        ("hash-map", func(|a| hash_map(a))),
        ("map?", func(fn_is_type!(Hash(_, _)))),
        ("assoc", func(assoc)),
        ("dissoc", func(dissoc)),
        ("get", func(get)),
        ("contains?", func(contains_q)),
        ("keys", func(keys)),
        ("vals", func(vals)),
        // 添加文件操作
        ("read-file", func(read_file)),
        ("ls", func(ls_dir)),
    ]
}

fn mal() -> Vec<&'static str> {
    vec![
        "(prn \"load core lisp Lib!\")",
        "(def! *gensym-counter* (atom 0))",
        "(def! gensym (lambda [] (symbol (str \"G__\"(swap! *gensym-counter* (lambda [x] (+ 1 x)))))))",
        "(defmacro! or (v (& xs) (if (empty? xs) nil (if (= 1 (count xs)) (first xs) (let* (condvar (gensym)) `(let* (~condvar ~(first xs)) (if ~condvar ~condvar (or ~@(rest xs)))))))))",
        "(def! not (lambda (a) (if a false true)))",
        "(defmacro! cond (lambda (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw \"odd number of forms to cond\")) (cons 'cond (rest (rest xs)))))))",
        "(def! load-file (lambda (f) (eval (read-string (str \"(do \" (read-file f) \"nil)\" )))))",
        // 初始化时添加进入系统入口
        "(load-file \"entry.jmal\")",
    ]
}

// 加载核心函数 使用rust进行定义
pub fn load_core(env: &Env) {
    for (k, v) in ns() {
        env_sets(&env, k, v);
    }
    load_core_lib(&env);
}

// 加载mal核心函数库
// 这个库的目的是使用mal 语言自己实现扩展
fn load_core_lib(env: &Env) {
    for s in mal() {
        let _ = rep(s, &env);
    }
}
