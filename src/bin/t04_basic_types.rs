//! Topic 4 — Basic types: numbers, char, bool, unit, statements vs expressions, functions.

fn square(n: i32) -> i32 {
    n * n // expression — no semicolon
}

fn main() {
    let _unit: () = ();
    let c: char = '🦀';
    let ok: bool = true;

    let x = {
        let a = 2u32;
        a + 3 // block expression value
    };

    println!("char={c}, bool={ok}, block={x}, square(7)={}", square(7));
}
