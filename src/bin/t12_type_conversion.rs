//! Topic 12 — `as`, From/Into, TryFrom, parse.

fn main() {
    let a: f64 = 9.8;
    let i = a as u8;

    let s = String::from("42");
    let n: i32 = s.parse().expect("digits");

    let u: u32 = u32::from(5u8);

    println!("as cast={i}, parse={n}, from={u}");
}
