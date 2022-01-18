#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mini_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use mini_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use mini_os::memory;
    use x86_64::structures::paging::Translate;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    mini_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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
