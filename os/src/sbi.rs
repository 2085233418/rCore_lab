// SBI调用编号（参考RustSBI规范）
const SBI_CONSOLE_PUTCHAR: usize = 1;  // 打印单个字符
const SBI_SHUTDOWN: usize = 8;         // 关机

/// 发起SBI调用
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,  // SBI调用编号存在x17
            options(nomem, nostack)
        );
    }
    ret
}

/// 打印单个字符（供console模块调用）
pub fn console_putchar(c: u8) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c as usize, 0, 0);
}

/// 关机（无返回）
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("Shutdown failed!");
}