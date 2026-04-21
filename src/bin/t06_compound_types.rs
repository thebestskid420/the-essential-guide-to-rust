//! Topic 6 — String, array, slice, tuple, struct, enum.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Msg {
    Quit,
    Move(i32, i32),
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &arr[1..];
    let pair: (i32, &str) = (42, "answer");
    let p = Point { x: 1, y: 2 };
    let m = Msg::Move(3, 4);
    let q = Msg::Quit;

    let owned = String::from("hi");
    let move_txt = match &m {
        Msg::Move(dx, dy) => format!("move {dx},{dy}"),
        Msg::Quit => "quit".into(),
    };
    println!(
        "slice={slice:?}, pair={pair:?}, xy=({},{}), {move_txt}, quit={q:?}, {owned}",
        p.x, p.y
    );
}
