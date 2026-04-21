//! Topic 18 — Closures and iterators.

fn main() {
    let add = |a: i32, b: i32| a + b;
    let mut sum = 0;
    let mut apply = |x: i32| {
        sum += x;
        sum
    };
    apply(1);
    apply(2);
    println!("closure sum={sum}, add(2,3)={}", add(2, 3));

    let v: Vec<i32> = (1..=5).filter(|n| n % 2 == 0).map(|n| n * 10).collect();
    println!("{v:?}");
}
