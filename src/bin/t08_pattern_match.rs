//! Topic 8 — match, matches!, if let, patterns.

#[derive(Copy, Clone)]
enum E {
    A(u8),
    B,
}

fn main() {
    let e = E::A(7);
    if let E::A(n) = e {
        println!("A({n})");
    }

    let ok = matches!(e, E::A(7));
    println!("matches: {ok}");

    let e2 = E::B;
    match e {
        E::A(0..=10) => println!("small A"),
        E::A(_) => println!("other A"),
        E::B => println!("B"),
    }
    println!("e2 is B: {}", matches!(e2, E::B));
}
