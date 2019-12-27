use crate::mal::types::MalVal::{Bool, Int, List, Nil, Str, Sym, Vector};
use crate::mal::types::MalErr::ErrString;
use crate::mal::types::MalErr;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use crate::mal::reader::State::Start;


#[derive(Debug, Clone)]
struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    // 阅读下一项
    fn next(&mut self) -> Result<String, MalErr> {
        self.pos = self.pos + 1;
        Ok(self
            .tokens
            .get(self.pos - 1)
            .ok_or(ErrString(String::from("underflow")))?
            .to_string())
    }
    // 看一眼下一项
    fn peek(&self) -> Result<String, MalErr> {
        Ok(self
            .tokens
            .get(self.pos)
            .ok_or(ErrString(String::from("underflow")))?
            .to_string())
    }
}

// token 识别状态
enum State{
    Start, // 开始状态
    Sym(String), // 特殊符号
    Comment(String), //注释
    Others(String)
}

// token化
fn tokenize(str: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut code = String::from(str);
    let mut state:State = Start;
    loop{
        match code.pop() {
            Some(t) =>{
                match t {
                    '`' => {
                        //todo
                    }
                    ' '=>{
                        
                    }
                    '\n'=>{

                    }
                    _ => {
                        
                    }
                }
            }
            None => {
                // 应该把当前状态没有识别结束的值 保存到vec中
                break;
            }
        }
    }
    res
}