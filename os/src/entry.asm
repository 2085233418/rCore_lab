.section .text.entry
.globl _start
_start:
    la sp, boot_stack_top  # 设置栈顶
    call rust_main         # 跳转到Rust的入口函数

.section .bss.stack
.globl boot_stack
boot_stack:
    .space 4096 * 16  # 64KB栈空间
.globl boot_stack_top
boot_stack_top: