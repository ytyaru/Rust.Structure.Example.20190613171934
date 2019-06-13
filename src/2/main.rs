/*
 * Rustの構造体（プログラム例）。
 * CreatedAt: 2019-06-13
 */
fn main() {
    let dimension = (3, 4);
    println!("w: {} h: {} area: {}", dimension.0, dimension.1, area(dimension));
}
fn area(dimension: (u32, u32)) -> u32 { dimension.0 * dimension.1 }
