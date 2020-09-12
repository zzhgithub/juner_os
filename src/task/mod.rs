use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

pub mod simple_executor;
pub mod keyboard;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(usize);

// 该Task结构是一个包裹了一个堆上的，固定的，且动态派发的输出为空类型()的future输出
pub struct Task {
    futrue: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(futrue: impl Future<Output = ()> + 'static) -> Task {
        Task {
            futrue: Box::pin(futrue),
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.futrue.as_mut().poll(context)
    }

}
