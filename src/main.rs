#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
#![feature(alloc)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

extern crate uefi;
extern crate uefi_services;
use crate::alloc::vec::Vec;
use uefi::{prelude::*, table::boot::MemoryType};
use uefi::proto::console::serial::{ControlBits, Serial};

const EFI_PAGE_SIZE: u64 = 0x1000;

fn memory_map(bt: &BootServices) {
    // Get the estimated map size
    let map_size = bt.memory_map_size();

    // Build a buffer bigger enough to handle the memory map
    let mut buffer = Vec::with_capacity(map_size);
    unsafe {
        buffer.set_len(map_size);
    }

    let (_k, desc_iter) = bt
        .memory_map(&mut buffer)
        .expect_success("Failed to retrieve UEFI memory map");

    let descriptors = desc_iter.copied().collect::<Vec<_>>();

    assert!(!descriptors.is_empty(), "Memory map is empty");

    // Print out a list of all the usable memory we see in the memory map.
    // Don't print out everything, the memory map is probably pretty big
    // (e.g. OVMF under QEMU returns a map with nearly 50 entries here).

    info!("efi: usable memory ranges ({} total)", descriptors.len());
    descriptors
        .iter()
        .for_each(|descriptor| match descriptor.ty {
            MemoryType::CONVENTIONAL => {
                let size = descriptor.page_count * EFI_PAGE_SIZE;
                let end_address = descriptor.phys_start + size;
                info!(
                    "> {:#x} - {:#x} ({} KiB)",
                    descriptor.phys_start, end_address, size
                );
            }
            _ => {}
        })
}

/// Ask the test runner to check the current screen output against a reference
///
/// This functionality is very specific to our QEMU-based test runner. Outside
/// of it, we just pause the tests for a couple of seconds to allow visual
/// inspection of the output.
fn check_screenshot(bt: &BootServices, name: &str) {
    if cfg!(feature = "qemu") {
        // Access the serial port (in a QEMU environment, it should always be there)
        let serial = bt
            .locate_protocol::<Serial>()
            .expect_success("Could not find serial port");
        let serial = unsafe { &mut *serial.get() };

        // Set a large timeout to avoid problems with Travis
        let mut io_mode = *serial.io_mode();
        io_mode.timeout = 10_000_000;
        serial
            .set_attributes(&io_mode)
            .expect_success("Failed to configure serial port timeout");

        // Send a screenshot request to the host
        serial
            .write(b"SCREENSHOT: ")
            .expect_success("Failed to send request");
        let name_bytes = name.as_bytes();
        serial
            .write(name_bytes)
            .expect_success("Failed to send request");
        serial.write(b"\n").expect_success("Failed to send request");

        // Wait for the host's acknowledgement before moving forward
        let mut reply = [0; 3];
        serial
            .read(&mut reply[..])
            .expect_success("Failed to read host reply");

        assert_eq!(&reply[..], b"OK\n", "Unexpected screenshot request reply");
    } else {
        // Outside of QEMU, give the user some time to inspect the output
        bt.stall(3_000_000);
    }
}


#[entry]
fn uefi_start(_image_handler: uefi::Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).expect_success("Failed to initialize utils");

    // reset console before doing anything else
    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    // Print out UEFI revision number
    {
        let rev = system_table.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());

        info!("UEFI {}.{}", major, minor);
    }
    memory_map(&system_table.boot_services());
    test::test(&system_table);

    // todo
    loop {}
    Status::SUCCESS
}



pub mod test;