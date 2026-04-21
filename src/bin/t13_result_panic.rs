//! Topic 13 — panic! and Result / ?.

fn might_fail(x: i32) -> Result<i32, &'static str> {
    if x < 0 {
        Err("negative")
    } else {
        Ok(x * 2)
    }
}

fn caller() -> Result<(), &'static str> {
    let y = might_fail(3)?;
    println!("ok: {y}");
    let _ = might_fail(-1)?; // propagates
    Ok(())
}

fn main() {
    if let Err(e) = caller() {
        println!("handled: {e}");
    }
}
