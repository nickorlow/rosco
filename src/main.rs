#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]

use core::panic::PanicInfo;
mod vga_txt;
pub mod interrupts;
use core::fmt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); 
    
    println!("ROSCO has booted!");
    set_color!(0xa);
    println!("");

  

    loop {
        print!("ROSCO SYS> ");
        let cmd: [char; 100] = readln!();
        parse_command(cmd);
    }
}

fn parse_command(cmd: [char;100]) {
    if(streq("panic usr", cmd)) {
        panic!("User-initiated panic");
    } else if(streq("panic mem", cmd)) {
        unsafe{
            let carr: [char;1] = ['\0'; 1];
            for i in 0..5 {
                if(carr[i] == '\0') {} // memory exception
            }
        }
    } else if(streq("info", cmd)) {
        println!("ROSCO (Rust Operating System for Computers, Obviously!)");
        println!("Inspired by: George Costanza v0.1");
    } else {
        println!("Command not found.");
    }
}

fn streq(str: &str, carr: [char;100]) -> bool {
    let mut x = 0;
    for i in 0..str.len() {
        if(carr[i] != str.chars().nth(i).unwrap()) {
            
            return false;
        }
        x = i;
    }
    
    return true;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_panic(_info);
    loop {}
}

fn print_panic(info: &PanicInfo) {
    use x86_64::instructions::interrupts;  

    interrupts::without_interrupts(|| {  
    clrscr!();
    set_color!(0x1f);
    print!("                                                                                ");
    print!("                                                                                ");
    
    print!  ("                                ");
    set_color!(0x71);
    print! (" Kernel Panic! ");
    set_color!(0x1f);
    print!("                                 ");
    print!("                                                                                ");
    print!("                                                                                ");

    print!("{}",format_args!("{:^80}" , "ROSCO HAS ENCOUNTERED A FATAL ERROR"));
    print!("{}",format_args!("{:^80}" , info.message().unwrap().as_str().unwrap()));
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                                                                                ");
    print!("{}",format_args!("{:^80}" , "THIS IS DUE TO A FAULT IN THE OS. PLEASE REPORT BUGS TO:"));
    print!("{}",format_args!("{:^80}" , "https://github.com/nickorlow/rosco"));

    for n in 13..20 {
        print!("                                                                                ");
    }
    print!("{}",format_args!("{:^80}" , "THERE IS NO CRYING IN PINTOS!"));
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                                                                                ");
    print!("                                                                               x");
});
}