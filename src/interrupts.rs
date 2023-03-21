use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;
use crate::{print, clrscr, println, print_at, set_color};
use pic8259::ChainedPics;
use spin;
use core::fmt;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub struct KeyboardState {
    shift: bool,
    reading: bool,
    buf_idx: usize,
    buffer: [char; 100]
}

static mut isReading: bool = false;

#[doc(hidden)]
pub fn _read_line() -> [char; 100] {
    use x86_64::instructions::interrupts;   
    interrupts::without_interrupts(|| {    
        KB_STAT.lock().reading = true;
        KB_STAT.lock().buffer = ['\0';100];
        KB_STAT.lock().buf_idx = 0;
        unsafe{isReading = true;}
    });
    
    unsafe {while(isReading) {} }
  let mut ret: [char;100] = ['\0';100];
    interrupts::without_interrupts(|| {  
    ret = KB_STAT.lock().buffer.clone();
    });

    return ret;
}

#[macro_export]
macro_rules! readln {
    () => {{$crate::interrupts::_read_line()}};
}


lazy_static! {
    pub static ref KB_STAT: Mutex<KeyboardState> = Mutex::new(KeyboardState {
        shift: false,
        reading: true,
        buf_idx: 0,
        buffer: ['\0';100]
    });
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, 
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub static mut TICKS: f32 = 0.0;

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    panic!("EXCEPTION: BREAKPOINT {:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    unsafe {
        TICKS += 1.0;

        set_color!(0xf0);
        let seconds = (TICKS*0.06);
        print_at!(0,24,"Tick Number: {}  /  Seconds Elapsed: {:.2}                                                          ", TICKS, seconds);
        set_color!(0xa);

        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}


extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    let mut is_r: bool = false;
    {
    use x86_64::instructions::port::Port;
    use x86_64::instructions::interrupts;   
    interrupts::without_interrupts(|| {    

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if(KB_STAT.lock().reading) {
        let mut key = match scancode {
            16 => 'q',
            17 => 'w',
            18 => 'e',
            19 => 'r',
            20 => 't',
            21 => 'y',
            22 => 'u',
            23 => 'i',
            24 => 'o',
            25 => 'p',
            30 => 'a',
            31 => 's',
            32 => 'd',
            33 => 'f',
            34 => 'g',
            35 => 'h',
            36 => 'j',
            37 => 'k',
            38 => 'l',
            44 => 'z',
            45 => 'x',
            46 => 'c',
            47 => 'v',
            48 => 'b',
            49 => 'n',
            50 => 'm',
            28 => '\n',
            57 => ' ',
            42 => {
                // shift down
                KB_STAT.lock().shift = true;
                '\0'
            }
            170 => {
                // shift up
                KB_STAT.lock().shift = false;
                '\0'
            }
            156 => {
                // enter up key
                let idx2: usize = KB_STAT.lock().buf_idx;
                KB_STAT.lock().buffer[idx2] = '\0';
                KB_STAT.lock().reading = false;
                '\0'
            }
            _ => '\0'
        };
        if(key != '\0' &&  KB_STAT.lock().buf_idx < 100) {
            if(KB_STAT.lock().shift) {
                key = key.to_uppercase().nth(0).unwrap();
            }
            print!("{}", key);
            let idx: usize = KB_STAT.lock().buf_idx;
            KB_STAT.lock().buffer[idx] = key;
            KB_STAT.lock().buf_idx += 1;
        } 
    }

    is_r = KB_STAT.lock().reading;
});


    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
    if(!is_r) {
        unsafe{isReading = false;}
    }
}