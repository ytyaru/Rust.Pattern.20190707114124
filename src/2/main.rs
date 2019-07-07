/*
 * Rustのパターン。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
