#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mini_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mini_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    mini_os::init();

    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }
    // let ptr = 0x20517e as *mut u32;
    // unsafe {
    //     let x = *ptr;
    // }
    // println!("read worked");
    // unsafe {
    //     *ptr = 42;
    // }
    // println!("write worked");

    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!(
    //     "Level 4 page table at: {:?}",
    //     level_4_page_table.start_address()
    // );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    mini_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mini_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mini_os::test_panic_handler(info)
}
