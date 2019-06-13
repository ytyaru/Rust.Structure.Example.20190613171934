/*
 * Rustの構造体（プログラム例）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let rect = Rectangle { width: 3, height: 4 };
//    println!("rect: {}", rect);   // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`    note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//    println!("rect: {:?}", rect); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`    note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
