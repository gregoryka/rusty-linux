#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::{entry_point, BootInfo};
use bootloader_x86_64_common::init_logger;
use bootloader_boot_config::LevelFilter;


fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = _boot_info.framebuffer.as_mut() {
        let info = fb.info();
        init_logger(fb.buffer_mut(), info, LevelFilter::Debug, true, true);
    }
    log::info!("Hello from kernel!");
    loop {}
}

entry_point!(kernel_main);

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}
