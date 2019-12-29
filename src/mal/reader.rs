use crate::list;
use crate::mal::reader::State::{Comment, Others, Start, StateSym};
use crate::mal::types::error;
use crate::mal::types::MalErr;
use crate::mal::types::MalErr::ErrString;
use crate::mal::types::MalRet;
use crate::mal::types::MalVal;
use crate::mal::types::MalVal::{Bool, Int, List, Nil, Str, Sym, Vector};
use crate::println;
use crate::vec;
use crate::vector;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use log::*;

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
#[derive(Debug, Clone)]
enum State {
    Start,            // 开始状态
    StateSym(String), // 特殊符号
    Comment(String),  //注释
    Others(String),
}

// token化
fn tokenize(str: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut code = String::from(str).chars().rev().collect::<String>();
    let mut state: State = Start;
    loop {
        let pre_state = state.clone();
        match code.pop() {
            Some(t) => {
                match t {
                    '`' | '\'' | '~' | '^' | '@' | '[' | ']' | '(' | ')' | '{' | '}' => {
                        match pre_state {
                            Start => {
                                state = StateSym(t.to_string());
                            }
                            StateSym(s) => {
                                if s == "~" && t == '@' {
                                    res.push(String::from("~@"));
                                } else {
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
                    ' ' => {
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
                    '\n' => {
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
                    ';' => match pre_state {
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
                    },
                    _ => {
                        trace!("Run in Other: {}", t);
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
        Start => {}
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
    res
}

fn is_numbers(s: &str) -> bool {
    
}

fn read_atom(rdr: &mut Reader) -> MalRet {
    let token = rdr.next()?;
    match &token[..] {
        "nil" => Ok(Nil),
        "false" => Ok(Bool(false)),
        "true" => Ok(Bool(true)),
        _ => {

        }
    }
}

fn read_form(rdr: &mut Reader) -> MalRet {
    let token = rdr.peek()?;
    match &token[..] {
        "'" => {
            let _ = rdr.next();
            Ok(list![Sym("quote".to_string()), read_form(rdr)?])
        }
        //todo
        _ => {
            //临时测试代码
            error("tmp")
        }
    }
}

pub fn read_str(str: String) -> MalRet {
    let tokens = tokenize(&str);
    println!("tokens: {:?}", tokens);
    if tokens.len() == 0 {
        return error("no input");
    }
    read_form(&mut Reader {
        pos: 0,
        tokens: tokens,
    })
}
