use spin::Mutex;
use core::fmt;
use lazy_static::lazy_static;

static VGA_WIDTH: usize = 80;
static VGA_HEIGHT: usize = 25;

pub struct VgaBuffer {
    buffer: u32,
    cur_x: usize,
    cur_y: usize,
    color_code: u8,
}

lazy_static! {
    pub static ref VGA_BUF: Mutex<VgaBuffer> = Mutex::new(VgaBuffer {
        cur_x: 0,
        cur_y: 0,
        color_code: 0xe,
        buffer: 0xb8000
    });
}

#[doc(hidden)]
pub fn _clear_screen() {
    VGA_BUF.lock().clr_scr();
}

#[doc(hidden)]
pub fn _set_color(color_cd: u8) {
    VGA_BUF.lock().color_code = color_cd;
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;  

    interrupts::without_interrupts(|| {    
        VGA_BUF.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_at(args: fmt::Arguments, cur_x:usize, cur_y:usize) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;  

    interrupts::without_interrupts(|| {   
        let old_x:usize = VGA_BUF.lock().cur_x;
        let old_y:usize = VGA_BUF.lock().cur_y;
        VGA_BUF.lock().cur_x = cur_x;
        VGA_BUF.lock().cur_y = cur_y;
        VGA_BUF.lock().write_fmt(args).unwrap();
        VGA_BUF.lock().cur_x = old_x;
        VGA_BUF.lock().cur_y = old_y;
    });
}

#[macro_export]
macro_rules! clrscr {
    () => ($crate::vga_txt::_clear_screen());
}

#[macro_export]
macro_rules! set_color {
    ($color_cd:expr) => ($crate::vga_txt::_set_color($color_cd));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_txt::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_at {
    ($cur_x:expr,$cur_y:expr,$($arg:tt)*) => ($crate::vga_txt::_print_at(format_args!($($arg)*), $cur_x, $cur_y));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


impl fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl VgaBuffer {
    fn new_line(&mut self) {
        self.cur_y += 1;
        self.cur_x = 0;
    }

    fn clr_scr(&mut self) {
        let temp_clr = self.color_code;
        for n in 0..VGA_WIDTH {
            for x in 0..VGA_HEIGHT {
                self.write_string(" ");
            }
        }
        self.cur_y = 0;
        self.cur_x = 0;
    }

    pub fn write_string(&mut self, write_me: &str) {
        let vga_buffer = 0xb8000 as *mut u8;
        unsafe {
            let mut i: usize = (self.cur_y * VGA_WIDTH) + self.cur_x;
            for byte in write_me.bytes() {
                // newline handling
                if (b'\n' == byte) {
                    self.new_line();
                    i  = (self.cur_y * VGA_WIDTH) + self.cur_x;
                    continue;
                }

                // format of VGA buffer
                // Bits 0-7: ASCII Code
                // Bits 7-11: Background Color
                // Bits 12-14: Foreground Color
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = self.color_code;
                i += 1;
            }

            self.cur_y = i / VGA_WIDTH;
            self.cur_x = i % VGA_WIDTH;
        }
    }
}

fn scroll_vga() {
    let vga_buffer = 0xb8000 as *mut u8;
    unsafe {
        for ln in 0..VGA_HEIGHT {
            let mut x: usize = ln * VGA_WIDTH;
            let mut y: usize = (ln + 1) * VGA_WIDTH;
            for z in 0..VGA_WIDTH {
                *vga_buffer.offset(x as isize * 2) = *vga_buffer.offset(y as isize * 2);
                *vga_buffer.offset(x as isize * 2 + 1) = *vga_buffer.offset(y as isize * 2 + 1);
                x += 1;
                y += 1;
            }
        }
    }
}
