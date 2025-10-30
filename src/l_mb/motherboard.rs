// VIRTUAL MOTHERBOARD
use crate::mem::mapping;

pub fn virtual_motherboard() {
    println!("Motherboard: L-MB v1.0 initialized!");
    println!("Decoding address test:");
    let test_addr = 0x1000_1234;
    println!("Address {:#X} belongs to {}", test_addr, mapping::decode_address(test_addr));
    // inside virtual_motherboard()
    let samples = [
    0x0000_0100, // ROM
    0x1000_1234, // RAM
    0x3000_0040, // VRAM
    0x4000_2004, // UART0
    0x4001_0010, // STORAGE_CTL
    0x5000_0100, // BLOCK_STORAGE_WIN
    ];
    for a in samples {
    println!("Address {:#010X} -> {}", a, mapping::decode_address(a));
    }
}


