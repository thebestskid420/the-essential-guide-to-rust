//! Topic 19 — Newtype pattern and DST basics.

struct Meters(u32);

impl Meters {
    fn new(n: u32) -> Self {
        Meters(n)
    }
}

fn main() {
    let m = Meters::new(3);
    let Meters(raw) = m;
    println!("raw meters: {raw}");

    let s: &str = "DST str slice";
    let sl: &[i32] = &[1, 2, 3];
    println!("{s} {:?}", sl);
}
