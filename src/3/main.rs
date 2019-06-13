/*
 * Rustの構造体（プログラム例）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let rect = Rectangle { width: 3, height: 4 };
    println!("w: {} h: {} area: {}", rect.width, rect.height, area(&rect));
}
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rect: &Rectangle) -> u32 { rect.width * rect.height }
