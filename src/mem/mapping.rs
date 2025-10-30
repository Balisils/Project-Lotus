// mem/mapping.rs

// ===== L-MB v1.0 (updated) =====
// ROM:   128 MiB  @ 0x0000_0000
// RAM:   512 MiB  @ 0x1000_0000
// VRAM:  128 MiB  @ 0x3000_0000
// MMIO:  0x4000_0000..0x7FFF_FFFF (UART, timers, storage ctrl, etc.)
// BLKWIN: 256 MiB aperture @ 0x5000_0000 (banks into 3.0 GiB virtual disk)

pub const ROM_BASE: u32 = 0x0000_0000;
pub const ROM_SIZE: u32 = 128 * 1024 * 1024; // 128 MiB Boot ROM

pub const RAM_BASE: u32 = 0x1000_0000;
pub const RAM_SIZE: u32 = 512 * 1024 * 1024; // 512 MiB System RAM

pub const VRAM_BASE: u32 = 0x3000_0000;
pub const VRAM_SIZE: u32 = 128 * 1024 * 1024; // 128 MiB VRAM/FB

// L-Bus MMIO (keep UART here)
pub const UART0_BASE: u32 = 0x4000_2000;

// Block Storage window (aperture into a 3.0 GiB virtual disk).
// The controller will map 256 MiB of the disk at a time using a bank/offset register.
pub const BLKWIN_BASE: u32 = 0x5000_0000;
pub const BLKWIN_SIZE: u32 = 256 * 1024 * 1024; // 256 MiB aperture

// (Optional) base for a small Storage Controller register block
pub const STORAGE_CTL_BASE: u32 = 0x4001_0000;
pub const STORAGE_CTL_SIZE: u32 = 0x0001_0000; // 64 KiB

pub fn decode_address(addr: u32) -> &'static str {
    // half-open ranges [base, end)
    const ROM_END: u32 = ROM_BASE + ROM_SIZE;
    const RAM_END: u32 = RAM_BASE + RAM_SIZE;
    const VRAM_END: u32 = VRAM_BASE + VRAM_SIZE;

    const UART0_END: u32 = UART0_BASE + 0x1000;
    const STORAGE_CTL_END: u32 = STORAGE_CTL_BASE + STORAGE_CTL_SIZE;
    const BLKWIN_END: u32 = BLKWIN_BASE + BLKWIN_SIZE;

    match addr {
        ROM_BASE..ROM_END => "ROM",
        RAM_BASE..RAM_END => "RAM",
        VRAM_BASE..VRAM_END => "VRAM",

        UART0_BASE..UART0_END => "UART0",
        STORAGE_CTL_BASE..STORAGE_CTL_END => "STORAGE_CTL",
        BLKWIN_BASE..BLKWIN_END => "BLOCK_STORAGE_WIN",

        0x4000_0000..0x8000_0000 => "L-BUS_MMIO",
        _ => "UNKNOWN",
    }
}

// If you later add a bank/offset register to the storage controller, expose helpers like:
// pub fn blk_win_bank_offset(bank_index: u32) -> u64 { (bank_index as u64) * BLKWIN_SIZE as u64 }

pub fn init() {
    println!("Memory map initialized:");
    println!("  ROM   {:08X}..{:08X}", ROM_BASE, ROM_BASE + ROM_SIZE - 1);
    println!("  RAM   {:08X}..{:08X}", RAM_BASE, RAM_BASE + RAM_SIZE - 1);
    println!("  VRAM  {:08X}..{:08X}", VRAM_BASE, VRAM_BASE + VRAM_SIZE - 1);
    println!("  UART0 {:08X}..{:08X}", UART0_BASE, UART0_BASE + 0x0FFF);
    println!("  SCTL  {:08X}..{:08X}", STORAGE_CTL_BASE, STORAGE_CTL_BASE + STORAGE_CTL_SIZE - 1);
    println!("  BLKWIN{:08X}..{:08X} (256 MiB aperture into 3.0 GiB disk)", BLKWIN_BASE, BLKWIN_BASE + BLKWIN_SIZE - 1);
}
