<!-- markdownlint-disable MD022 MD032 MD047 -->
# The probles in ch1
## 文件结构
    OS
    {
        bootloader{}
        os{
            .cargo
            {
                config.toml
            }
            src
            {
                console.rs
                entry.asm
                lang_items.rs
                linker.ld
                main.rs
            }
            target
            {

            }
            
        }

    }
    
## 各目录/文件功能解释
### 1. bootloader/ 目录
- 核心文件：`rustsbi-qemu.bin`
- 功能：RISC-V 架构下操作系统的**启动固件**，作为内核与硬件之间的“桥梁”。
- 核心作用：内核运行前完成硬件初始化（如 CPU、中断控制器），引导内核加载到指定内存地址，同时提供特权级切换支持，最终将控制权移交内核。

### 2. os/ 目录（内核核心）
#### （1）.cargo/ 目录
- 核心文件：`config.toml`
- 功能：Cargo 的编译配置文件，用于指定内核编译规则。
- 核心作用：定义默认编译目标（RISC-V 64 位裸机环境）、链接器（rust-lld）、编译选项（如指令集扩展、链接脚本路径），确保内核能脱离标准库编译为裸机可执行程序。

#### （2）src/ 目录（内核源码核心）
- `console.rs`：控制台输出模块
  - 功能：封装 SBI 单字符打印接口，实现 Rust 标准 `Write` trait。
  - 核心作用：提供 `print!` 和 `println!` 宏，支持格式化输出（如字符串、数字），方便内核调试信息打印。

- `entry.asm`：汇编入口文件
  - 功能：内核启动时的汇编级初始化。
  - 核心作用：配置栈空间、设置 CPU 初始状态，最终调用 Rust 内核的入口函数（如 `rust_main`）。

- `lang_items.rs`：语言项实现模块
  - 功能：补全裸机环境下的必需语言项。
  - 核心作用：实现 `panic_handler`（panic 异常处理，打印信息并关机）、`alloc_error_handler`（内存分配失败处理），以及占位式内存分配器，确保内核编译通过并能优雅处理异常。

- `linker.ld`：链接脚本文件
  - 功能：控制链接器行为，定义内核的内存布局。
  - 核心作用：指定代码段、数据段、未初始化数据段（bss）的内存地址（如内核加载地址 `0x80200000`），确保内核各部分按预期布局在内存中。

- `main.rs`：内核主入口文件
  - 功能：内核的 Rust 层入口，组织内核核心逻辑。
  - 核心作用：调用初始化逻辑、启动内核主流程，是内核功能的总入口。

- `sbi.rs`：SBI 交互接口模块
  - 功能：内核与 RustSBI 通信的核心接口。
  - 核心作用：通过 `ecall` 指令调用 RustSBI 提供的底层服务（如 `console_putchar` 字符打印、`shutdown` 关机），是内核间接操作硬件的关键桥梁。

#### （3）target/ 目录
- 功能：自动存储编译产物的目录。
- 核心内容：包含编译生成的 ELF 文件（`os`）、裸机二进制文件（`os.bin`）及中间编译产物，无需手动修改。
## 运行时命令
    qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -serial mon:stdio \
    -no-reboot
### 改动位置
- 1 -serial mon:stdio
- 2 -no -reboot
### 退出命令
    Ctrl + A  then  X