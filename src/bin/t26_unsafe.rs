//! Topic 26 — Unsafe Rust (inline `asm!` only when you target a platform and read the reference).

fn main() {
    let mut x: u32 = 1;
    let p: *mut u32 = &mut x;
    unsafe {
        *p = 2;
    }
    println!("x={x} (raw pointer write in unsafe block)");
    println!("TODO: read the Rustonomicon chapters before large unsafe experiments.");
}
