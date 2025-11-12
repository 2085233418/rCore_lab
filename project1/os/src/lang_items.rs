use core::panic::PanicInfo;

/// panic处理函数（裸机环境必须实现）
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 打印panic信息后关机
    crate::println!("\nPanic: {}", info);
    crate::sbi::shutdown();
}

/// 内存分配器（暂时用空实现，后续可替换为实际分配器）
#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    panic!("Allocation failed!");
}

// 导入内存分配器 trait（如果需要使用alloc）
extern crate alloc;
use alloc::alloc::GlobalAlloc;
struct DummyAllocator;
unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}
#[global_allocator]
static DUMMY_ALLOCATOR: DummyAllocator = DummyAllocator;