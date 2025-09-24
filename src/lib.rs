#![no_std]
#![no_main]

use core::panic::PanicInfo;

use x86_64::instructions::{hlt, interrupts, port::Port};

unsafe fn serial_print_byte(byte: u8) {
    while (unsafe { Port::<u8>::new(0x3F8 + 5).read() } & 0x20) == 0 {}
    unsafe { Port::<u8>::new(0x3F8).write(byte) };
}

unsafe fn serial_print_str(s: &str) {
    for byte in s.bytes() {
        unsafe { serial_print_byte(byte) };
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.start")]
pub extern "C" fn _start() -> ! {
    interrupts::disable();

    unsafe {
        serial_print_str("Hello from Kernel!\n");
    }

    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        serial_print_str("KERNEL PANIC!\n");
    }
    loop {
        hlt();
    }
}
