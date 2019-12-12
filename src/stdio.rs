use alloc::{collections::vec_deque::VecDeque, string::String, sync::Arc};
use spin::Mutex;

use lazy_static::lazy_static;

#[derive(Default)]
pub struct Stdin {
    buf: Mutex<VecDeque<char>>,
}

impl Stdin {

    pub fn push(&self,c:char){
        self.buf.lock().push_back(c);
    }

    pub fn pop(&self) -> char{
        loop {
            let mut buf_lock = self.buf.lock();
            match buf_lock.pop_front() {
                Some(c) => return c,
                None => {
                    //
                }
            }
        }
    }
}

lazy_static! {
    pub static ref STDIN: Arc<Stdin> = Arc::new(Stdin::default());
}
