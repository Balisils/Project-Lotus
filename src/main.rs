mod l_cpu;
mod l_mb;
mod mem;

fn main() {
    println!("//====================//");
    println!("Welcome to lvm-32 v0.1.0");
    println!("Created by: www.underpin.studio");
    println!("Authored by: Omkar Namjoshi");
    println!("//====================//");
    println!("PROPERTY OF UNDERPIN STUDIO");
    println!("ALL RIGHTS RESERVED");
    println!("//====================//");
    println!("Lotus Virtual Machine - LVM-32 v1.0");
    println!("//====================//");

    l_mb::motherboard::virtual_motherboard();

    let mut cpu = l_cpu::cpu::CPU::new();
    cpu.run();

    mem::memory::init();

    println!("//====================//");
}
