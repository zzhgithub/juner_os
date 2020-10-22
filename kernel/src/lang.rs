use core::alloc::Layout;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    print!("\n\n");
    print!(31;r"



IIIIIIII          III            OO     III     II        IIIIIIIIIIII
II     II        II  II                 IIII    II        II
II     II       II    II         II     II II   II        II
II     II      IIIIIIIIII        II     II  II  II        II
IIIIIIII      II        II       II     II   II II        II
II           II          II      II     II    IIII        II
II          II            II     II     II     III        IIIIIIIIIIIII



");
    println!("{}", info);
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort!");
}

#[lang = "oom"]
pub fn oom(layout: Layout) -> ! {
    panic!("Memory allocation of {} bytes failed", layout.size());
}

#[lang = "eh_personality"]
#[no_mangle]
pub fn eh_personality() -> ! {
    loop {}
}
