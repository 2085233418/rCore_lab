use core::fmt::{self, Write};
use super::sbi::console_putchar;  // 导入SBI打印函数

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 逐个字符打印（SBI只支持单字符输出）
        for c in s.bytes() {
            console_putchar(c);
        }
        Ok(())
    }
}

/// 底层打印实现
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// print宏（直接复用你的逻辑）
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // 直接把 $fmt 和 $arg 传给 format_args!，不用额外嵌套
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // 去掉多余的 concat! 嵌套，直接用 format_args! 拼接换行
        $crate::console::print(format_args!("{}\n", format_args!($fmt $(, $($arg)+)?)));
    };
}