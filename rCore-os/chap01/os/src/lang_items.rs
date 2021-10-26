use crate::sbi::shutdown;
use core::panic::PanicInfo;


// 在标准库 std 中提供了 panic 的处理函数 #[panic_handler]，其大致功能是打印出错位置和原因并杀死当前应用。
// 但panic 的处理在核心库 core 中并没有提供，因此我们需要自己先实现一个简陋的 panic 处理函数，这样才能支持“三叶虫”操作系统的编译通过。
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
