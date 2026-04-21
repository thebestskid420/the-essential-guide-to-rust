//! Topic 9 — Methods and associated functions.

struct Counter(u32);

impl Counter {
    fn new() -> Self {
        Counter(0)
    }

    fn bump(&mut self) {
        self.0 += 1;
    }

    fn value(&self) -> u32 {
        self.0
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    println!("{}", c.value());
}
