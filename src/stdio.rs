use alloc::{collections::vec_deque::VecDeque, string::String, sync::Arc};
use spin::Mutex;
use crate::print;

use lazy_static::lazy_static;

#[derive(Default)]
pub struct Stdin {
    buf: Mutex<VecDeque<char>>,
}

impl Stdin {

    // 进入输入缓存
    pub fn push(&self,c:char){
        self.buf.lock().push_back(c);
        // todo
    }

    pub fn pop(&self) -> char{
        loop {
            let mut buf_lock = self.buf.lock();
            match buf_lock.pop_front() {
                Some(c) => return c,
                None => {
                    // TODO 这里要等待 有人使用这个锁 
                    print!("the loop！");
                }
            }
        }
    }
    // 输入缓存 传到字符串
    pub fn to_string(&self) -> String{
        let buf_lock = self.buf.lock();
        buf_lock.iter().cloned().collect::<String>()
    }
    
    // 清空输入缓存
    pub fn clear(&self){
        let mut buf_lock = self.buf.lock();
        buf_lock.clear();
    }

    // 删除一个字符并且 返回剩余的长度
    pub fn back_spacse(&self)-> usize {
        let mut buf_lock = self.buf.lock();
        match buf_lock.pop_front() {
            Some(c) => buf_lock.len(),
            None => 0 as usize,
        }
    }
}

lazy_static! {
    pub static ref STDIN: Arc<Stdin> = Arc::new(Stdin::default());
}

//获取一个字符串 作为u8
pub fn get_char() -> u8 {
    crate::stdio::STDIN.pop() as u8
}