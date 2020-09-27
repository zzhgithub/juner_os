use uefi::prelude::*;
use uefi::proto::console::text::Input;
use uefi::table::boot::BootServices;

// 无法使用的方法
pub async fn test_input(st: SystemTable<Boot>) {
    loop {
        let events = &mut [st.stdin().wait_for_key_event()];
        &st.boot_services().wait_for_event(events).unwrap();
        match st.stdin().read_key().unwrap().unwrap() {
            Some(key) => info!("{:?}", key),
            None => {}
        };
    }
}

pub async fn input(input: &mut Input, bt: &BootServices) {
    let events = &mut [input.wait_for_key_event()];
    bt.wait_for_event(events).unwrap_err();
    match input.read_key().unwrap().unwrap() {
        Some(key) => info!("{:?}", key),
        None => {}
    }
}
