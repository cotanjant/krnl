// Copyright (C) 2026 Deniz Dayan
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.


#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;
    let msg = b"Hello from rust";
    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            *vga.add(i * 2) = byte;
            *vga.add(i * 2 + 1) = 0x0f;
        }
    }
    loop {}
}