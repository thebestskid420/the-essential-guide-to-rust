//! Topic 7 — Flow control: if, loops, break/continue, labels.

fn main() {
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                break 'outer;
            }
        }
    }

    let mut n = 0;
    while n < 3 {
        n += 1;
    }

    let v: Vec<i32> = (0..4).collect();
    println!("done, n={n}, v={v:?}");
}
