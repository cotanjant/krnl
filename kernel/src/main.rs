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
use limine::request::FramebufferRequest;

#[used]
#[unsafe(link_section = ".requests")]
static LIMINE_BASE_REVISION: [u64; 4] = [
    0xf9562b2d5c95a6f8,
    0x67cf3cbd0c0d4e0a,
    0,
    0,
];

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let framebuffer_response = FRAMEBUFFER_REQUEST
        .response()
        .expect("Framebuffer response missing");

    let framebuffer = framebuffer_response
        .framebuffers()
        .first()
        .expect("No framebuffer");
    let address = framebuffer.address();
    let width = framebuffer.width;
    let height = framebuffer.height;
    let pitch = framebuffer.pitch;
    let bpp = framebuffer.bpp;

    let bytes_per_pixel = bpp / 8; 
    // biti bytea çevirdik bcz 8 bit = 1 byte dolayısıyla 32 bit framebufferda 1 pixel = 4 byte a eşit
    // 64 bit olsaydı 8 byte olurdu yani 64 / 8 = 8

    let rectangle_height= 100usize.min(height as usize);
    
    let rectangle_width = 100usize.min(width as usize);

    for y in 0..rectangle_height {
        for x in 0..rectangle_width {
            let offset = y * pitch as usize + x * bytes_per_pixel as usize;
        
            
            unsafe {
                let pixel = (address as *mut u8)
                    .add(offset);
                *pixel = 0x00; // blue
                *pixel.add(1) = 0x00; // green
                *pixel.add(2) = 0xff; // red
            }
        }
    }

    let _ = (   
        address, 
        width, 
        height, 
        pitch,
        bytes_per_pixel
    );
    
    loop {
        core::hint::spin_loop(); // bunu sonra çıkarmalıyız çünkü bu kasıtlı olarak uyarıları devre dışı bırakır.
    }
}