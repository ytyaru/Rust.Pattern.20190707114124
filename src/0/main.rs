/*
 * Rustのパターン。
 * CreatedAt: 2019-07-07
 */
fn main() {
    let v = 0;
    match v {
        0 => println!("これ野比！"),
        100 => println!("天才！"),
        _ => println!("お疲れ様でした。"),
    }
}

