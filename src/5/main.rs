/*
 * Rustのパターン。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
fn foo(x: i32) {}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
