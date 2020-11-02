use lazy_static::lazy_static;
use ps2_mouse::{Mouse, MouseState};
use spin::Mutex;
use x86_64::instructions::port::PortReadOnly;

lazy_static! {
    pub static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

// 初始化鼠标配置
pub fn init_mouse() {
    MOUSE.lock().init().unwrap();
    MOUSE.lock().set_on_complete(on_complete);
}

// This will be fired when a packet is finished being processed.
fn on_complete(mouse_state: MouseState) {
    // 当你要进行鼠标的操作时 要处理这行
    // println!("{:?}", mouse_state);
}

pub fn mouse() {
    let mut port = PortReadOnly::new(0x60);
    let packet = unsafe { port.read() };
    MOUSE.lock().process_packet(packet);
}
