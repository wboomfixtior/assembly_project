use core::arch::global_asm;

// Multiply x by 6 using shifts and adds

global_asm!(include_str!("link.s"));

unsafe extern "C"{
    unsafe fn return3() -> u64;
}

fn main() {
    println!("Hello, world!");
    unsafe {
        println!("assembly returns {}", return3())

    }
   
}

