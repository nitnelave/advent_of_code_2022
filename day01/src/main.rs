#![no_std]
#![no_main]
#![feature(lang_items)]

use core::arch::asm;

const BIG_BUFFER_SIZE: usize = 1 << 16;
const SMALL_BUFFER_SIZE: usize = 8;

#[repr(align(64))]
struct SmallBuffer([u8; SMALL_BUFFER_SIZE]);
#[repr(align(64))]
struct BigBuffer([u8; BIG_BUFFER_SIZE]);

struct TopN<T: Ord + Copy> {
    elements: [T; 3],
}

impl<T: Ord + Copy> TopN<T> {
    fn push(&mut self, mut val: T) {
        for mut v in &mut self.elements {
            if val > *v {
                core::mem::swap(&mut val, &mut v);
            }
        }
    }

    fn max(&self) -> T {
        //self.elements.iter().max().unwrap()
        let tmp = if self.elements[0] > self.elements[1] {
            self.elements[0]
        } else {
            self.elements[1]
        };
        if tmp > self.elements[2] {
            tmp
        } else {
            self.elements[2]
        }
    }

    fn top_n(&self) -> &[T; 3] {
        &self.elements
    }
}

fn read_stdin(input_buffer: &mut BigBuffer) -> &[u8] {
    let read_code = read(STDIN_FILENO, &mut input_buffer.0);
    return unsafe { input_buffer.0.get_unchecked(..read_code) };
}

const STDIN_FILENO: u32 = 0;
const STDOUT_FILENO: u32 = 1;

fn exit(code: i32) -> ! {
    let syscall_number: u32 = 60;
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") code,
            options(noreturn)
        );
    }
}
fn write(fd: u32, buf: &[u8]) {
    let syscall_number: u32 = 1;
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            // Linux syscalls don't touch the stack at all, so
            // we don't care about its alignment
            options(nostack)
        );
    }
}

fn read(fd: u32, buf: &mut [u8]) -> usize {
    let syscall_number: u32 = 0;
    let res;
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") fd,
            in("rsi") buf.as_mut_ptr(),
            in("rdx") buf.len(),
            lateout("rax") res,
            // Linux syscalls don't touch the stack at all, so
            // we don't care about its alignment
            options(nostack)
        );
    }
    res
}

fn int_to_buf(mut input: u32, buf: &mut SmallBuffer) -> &[u8] {
    buf.0[SMALL_BUFFER_SIZE - 1] = b'\n';
    let mut count = SMALL_BUFFER_SIZE - 2;
    unsafe {
        loop {
            *buf.0.get_unchecked_mut(count) = (input % 10) as u8 + b'0';
            input /= 10;
            if input == 0 {
                return buf.0.get_unchecked(count..);
            }
            count -= 1;
        }
    }
}

fn print_int(i: u32) {
    static mut BUF: SmallBuffer = SmallBuffer([0; SMALL_BUFFER_SIZE]);
    let output = int_to_buf(i, unsafe { &mut BUF });
    write(STDOUT_FILENO, output);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    static mut INPUT_BUFFER: BigBuffer = BigBuffer([0; BIG_BUFFER_SIZE]);
    let contents = unsafe { read_stdin(&mut INPUT_BUFFER) };
    let mut elf_sum = 0;
    let mut num = 0;
    let mut top_3 = TopN { elements: [0; 3] };
    unsafe {
        for i in 0..contents.len() - 1 {
            if contents[i] == b'\n' {
                elf_sum += num;
                num = 0;
                if *contents.get_unchecked(i + 1) == b'\n' {
                    top_3.push(elf_sum);
                    elf_sum = 0;
                }
            } else {
                num *= 10;
                num += (contents[i] - b'0') as u32;
            }
        }
    }
    print_int(top_3.max());
    let sum = top_3.top_n()[0] + top_3.top_n()[1] + top_3.top_n()[2];
    print_int(sum);
    exit(0);
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
