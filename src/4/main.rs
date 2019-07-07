/*
 * Rustのパターン。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    let (x, y) = (1, 2, 3); // error[E0308]: mismatched types
}
