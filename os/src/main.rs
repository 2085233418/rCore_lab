#![no_std]
#![no_main]
#![feature(alloc_error_handler)]  // 只保留这个必要的不稳定特性
// 删掉 #![feature(global_asm)] （警告提示已稳定，无需启用）

// 导入 global_asm 宏（按编译器提示添加）
use core::arch::global_asm;
// 嵌入汇编入口（现在能找到宏了）
global_asm!(include_str!("entry.asm"));

mod lang_items;
mod sbi;
mod console;

/// 清零BSS段（必须在使用全局变量前执行）
fn clear_bss() {
    extern "C" {
        fn sbss();  // BSS段起始地址（链接脚本定义）
        fn ebss();  // BSS段结束地址（链接脚本定义）
    }
    // 遍历BSS段，逐个字节置0
    (sbss as usize..ebss as usize).for_each(|addr| {
        unsafe { (addr as *mut u8).write_volatile(0) }
    });
}

/// Rust入口函数（被汇编的_start调用）
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();          // 第一步：清零BSS段
    println!("Hello world!");  // 第二步：打印
    sbi::shutdown();      // 第三步：关机
}