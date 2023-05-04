mod count_chars;

fn main() {
    let text = "Hello, world!".to_owned();
    println!("{}", text);
    count_chars::count(&text);
}
