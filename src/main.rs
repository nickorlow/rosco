#![no_std]
#![no_main]

use core::panic::PanicInfo;

static BOOT_MSG: &[u8] = b"CONTROL TRANSFERRED TO ROSCO...";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // write boot message to VGA buffer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in BOOT_MSG.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
