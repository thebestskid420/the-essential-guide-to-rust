//! Topic 28 — Tests: run with `cargo test --bin t28_tests`.

pub fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("Run tests: cargo test --bin t28_tests");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles() {
        assert_eq!(double(21), 42);
    }
}
