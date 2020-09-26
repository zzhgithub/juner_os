use uefi::prelude::*;
use uefi::proto::console::text::Input;

pub fn test(st: &SystemTable<Boot>) {
    info!("Testing console protocols");
    stdout::test(st.stdout());
    let bt = st.boot_services();
    gop::test(&bt);
    pointer::test(&bt);
}


pub mod stdout;
pub mod gop;
pub mod pointer;