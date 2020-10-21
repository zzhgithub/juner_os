#![no_std]

extern crate alloc;

use alloc::vec::Vec;
pub use uefi::proto::console::gop::ModeInfo;
pub use uefi::table::boot::{MemoryAttribute, MemoryDescriptor, MemoryType};
use x86_64::VirtAddr;

/// This structure represents the information that the bootloader passes to the kernel.
/// 引导信息 要传给内核的数据结构
#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
    /// 内存地址 映射 MemoryDescriptor 内存描述符
    pub memory_map: Vec<&'static MemoryDescriptor>,
    /// The offset into the virtual address space where the physical memory is mapped.
    /// 虚拟地址偏移量
    pub physical_memory_offset: u64,
    /// The graphic output information
    /// 图像信息
    pub graphic_info: GraphicInfo,
    /// Physical address of ACPI2 RSDP
    /// 电源管理地址
    pub acpi2_rsdp_addr: u64,
    pub smbios_addr: u64,
}

/// Kernel entry's virtual address.
/// 内核入口 的 虚拟地址
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct KernelEntry(pub VirtAddr);

/// Function signature for kernel entry point.
/// 内核 入口点 方法签名！
pub type KernelEntryFn = extern "sysv64" fn(boot_info: &'static BootInfo) -> !;

/// Graphic output information
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GraphicInfo {
    /// Graphic mode
    pub mode: ModeInfo,
    /// Framebuffer base physical address
    pub fb_addr: u64,
    /// Framebuffer size
    pub fb_size: u64,
}

/// Defines the entry point function.
///
/// The function must have the signature `fn(&'static BootInfo) -> !`.
///
/// This macro just creates A function named `_start`, which the linker will use as the entry
/// point. The advantage of using this macro instead of providing an own `_start` function is
/// that the macro ensures that the function and argument types are correct.
#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        #[export_name = "_start"]
        pub extern "C" fn __impl_start(boot_info: &'static $crate::BootInfo) -> ! {
            // validate the signature of the program entry point
            let f: fn(&'static $crate::BootInfo) -> ! = $path;
            f(boot_info)
        }
    };
}
