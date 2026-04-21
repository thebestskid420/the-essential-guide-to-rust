//! Topic 3 — Variables: `let`, `mut`, shadowing, constants (`const`, `static`).

fn main() {
    let x = 1;
    let mut y = x;
    y += 1;

    let z = "hello";
    let z = z.len(); // shadowing — new type allowed

    const MAX: u32 = 100;
    println!("x={x}, y={y}, z={z}, MAX={MAX}");
}
