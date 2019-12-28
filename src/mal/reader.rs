use crate::mal::types::MalVal::{Bool, Int, List, Nil, Str, Sym, Vector};
use crate::mal::types::MalErr::ErrString;
use crate::mal::types::MalErr;
use alloc::rc::Rc;
use alloc::string::{String,ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use crate::mal::reader::State::{Start,StateSym,Comment,Others};
use crate::println;

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
#[derive(Debug,Clone)]
enum State{
    Start, // 开始状态
    StateSym(String), // 特殊符号
    Comment(String), //注释
    Others(String)
}

// token化
fn tokenize(str: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut code = String::from(str);
    let mut state:State = Start;
    loop{
        let pre_state = state.clone();
        match code.pop() {
            Some(t) =>{
                match t {
                    '`' | '\''| '~' | '^' | '@' | '[' | ']' | '(' | ')' | '{' | '}' => {
                        match pre_state {
                            Start => {
                                state = StateSym(t.to_string());
                            }
                            StateSym(s) => {
                                if s == "~" && t == '@' {
                                    res.push(String::from("~@"));
                                }else{
                                    res.push(s);
                                    res.push(t.to_string());
                                    state = Start;
                                }
                            }
                            Comment(s) => {
                                let mut tmp = s.clone();
                                tmp.push(t);
                                state = Comment(tmp);
                            }
                            Others(s) => {
                                res.push(s);
                                state = StateSym(t.to_string());
                            }
                        }
                    }
                    ' '=>{
                        match pre_state {
                            Start => {
                                // do nothing
                            }
                            StateSym(s) => {
                                res.push(s);
                                state = Start;
                            }
                            Comment(s) => {
                                let mut tmp = s.clone();
                                tmp.push(t);
                                state = Comment(tmp);
                            }
                            Others(s) => {
                                res.push(s);
                                state = Start;
                            }
                        }
                    }
                    '\n'=>{
                        match pre_state {
                            Start => {
                                // do nothing
                            }
                            StateSym(s) => {
                                res.push(s);
                                state = Start;
                            }
                            Comment(s) => {
                                res.push(s);
                                state = Start;
                            }
                            Others(s) => {
                                res.push(s);
                                state = Start;
                            }
                        }
                    }
                    ';' => {
                        match pre_state {
                            Start => {
                                state = Comment(String::from(t.to_string()));
                            }
                            StateSym(s) => {
                                res.push(s);
                                state = Comment(String::from(t.to_string()));
                            }
                            Comment(s) => {
                                let mut tmp = s.clone();
                                tmp.push(t);
                                state = Comment(tmp);
                            }
                            Others(s) => {
                                res.push(s);
                                state = Comment(String::from(t.to_string()));
                            }
                        }
                    }
                    _ => {
                        match pre_state {
                            Start => {
                                state = Others(t.to_string());
                            }
                            StateSym(s) => {
                                res.push(s);
                                state = Others(t.to_string());
                            }
                            Comment(s) => {
                                let mut tmp = s.clone();
                                tmp.push(t);
                                state = Comment(tmp);
                            }
                            Others(s) => {
                                let mut tmp = s.clone();
                                tmp.push(t);
                                state = Others(tmp);
                            }
                        }
                    }
                }
            }
            None => {
                break;
            }
        }
    }
     // 应该把当前状态没有识别结束的值 保存到vec中
    match state {
        Start => {
        }
        StateSym(s) => {
            res.push(s);
        }
        Comment(s) => {
            res.push(s);
        }
        Others(s) => {
            res.push(s);
        }
    }
    // 设置成结束？？ 不需要了
    res.reverse();
    res.iter().map(|s| s.chars().rev().collect::<String>()).collect()
}

pub fn read_str(str: String) {
    let tokens = tokenize(&str);
    println!("tokens: {:?}", tokens);
}