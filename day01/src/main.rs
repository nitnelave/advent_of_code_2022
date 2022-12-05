#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate compiler_builtins;

use core::arch::asm;

struct TopN<const N: usize, T: Ord> {
    elements: [T; N],
}

impl<const N: usize, T: Ord> TopN<N, T> {
    fn push(&mut self, mut val: T) {
        for mut v in &mut self.elements {
            if val > *v {
                core::mem::swap(&mut val, &mut v);
            }
        }
    }

    fn max(&self) -> &T {
        self.elements.iter().max().unwrap()
    }

    fn top_n(&self) -> &[T; N] {
        &self.elements
    }
}

fn read_stdin() -> Result<&'static [u8], ()> {
    static mut INPUT_BUFFER: &'static mut [u8] = &mut [0; 100000];
    let mut buffer_size = 0;
    loop {
        let read_code = unsafe {
            read(
                STDIN_FILENO,
                INPUT_BUFFER[buffer_size..].as_mut_ptr(),
                INPUT_BUFFER.len() - buffer_size,
            )
        };
        buffer_size += read_code as usize;
        if read_code == 0 {
            return unsafe { Ok(&INPUT_BUFFER[..buffer_size]) };
        }
    }
}

pub unsafe fn exit(code: i32) -> ! {
    let syscall_number: i64 = 60;
    asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    );
}

pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) {
    let syscall_number: i64 = 1;
    asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        options(nostack)
    );
}

pub unsafe fn read(fd: u32, buf: *mut u8, count: usize) -> i64 {
    let syscall_number: i64 = 0;
    let res;
    asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rax") res,
        // Linux syscalls don't touch the stack at all, so
        // we don't care about its alignment
        //options(nostack)
    );
    res
}

fn int_to_buf(mut input: i64, buf: &mut [u8; 10]) -> &[u8] {
    buf[9] = b'\n';
    let is_negative = input < 0;
    if is_negative {
        input = -input;
    }
    let mut count = 8;
    loop {
        buf[count] = (input % 10) as u8 + b'0';
        input /= 10;
        if input == 0 {
            if is_negative {
                count -= 1;
                buf[count] = b'-';
            }
            return &buf[count..];
        }
        count -= 1;
        if count == 0 {
            panic!();
        }
    }
}

fn parse_int(input: &[u8]) -> i64 {
    let mut res = 0;
    for c in input {
        res *= 10;
        res += (c - b'0') as i64;
    }
    res
}

#[inline]
fn debug<const N: usize>(input: &[u8; N]) {
    unsafe {
        write(2, input.as_ptr(), N);
    }
}

fn print_int(i: i64) {
    let mut buf = [0; 10];
    let output = int_to_buf(i, &mut buf);
    unsafe { write(STDOUT_FILENO, output.as_ptr(), output.len()) };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let contents = read_stdin().unwrap();
    let mut previous_start = 0;
    let mut top_3 = TopN { elements: [0; 3] };
    for i in 0..contents.len() - 1 {
        if contents[i] == b'\n' && contents[i + 1] == b'\n' {
            top_3.push(
                contents[previous_start..i]
                    .split(|c| *c == b'\n')
                    .map(|l| parse_int(l))
                    .sum(),
            );
            previous_start = i + 2;
        }
    }
    print_int(*top_3.max());
    print_int(top_3.top_n().iter().sum::<i64>());
    unsafe { exit(0) };
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    debug(b"panic");
    unsafe { exit(1) }
}
