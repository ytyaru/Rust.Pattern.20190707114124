/*
 * Rustのパターン。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
