/*
 * Rustの構造体（プログラム例）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let width = 3;
    let height = 4;
    println!("w: {} h: {} area: {}", width, height, area(width, height));
}
fn area(w: u32, h: u32) -> u32 { w * h }
