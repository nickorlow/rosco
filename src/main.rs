#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_txt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    println!("Hello, World!");
    set_color!(0xa);
    println!("Helno, World!");
    
    panic!("OS not built yet!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_panic(_info);
    loop {}
}

fn print_panic(info: &PanicInfo) {
    clrscr!();
    set_color!(0x1f);
    print!("                                                                                ");
    print!("                                                                                ");
    
    print!  ("                                ");
    set_color!(0x71);
    print!  (" Kernel Panic! ");
    set_color!(0x1f);
    print!  ("                                 ");
    
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                  ROSCO has encountered a fatal error. Error:                   ");
    print!("             {}                     " , info);
    print!("                                                                                ");
    print!("           This is due to a fault in the OS. Please report it to                ");
    print!("                  https://github.com/nickorlow/rosco                            ");
    for n in 10..20 {
        print!("                                                                                ");
    }
    print!("                      THERE IS NO CRYING IN PINTOS!                             ");
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                                                                                ");
}