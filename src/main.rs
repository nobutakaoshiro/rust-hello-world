fn main() {
    let name = "Taro";
    // dbg!(name);
    let result = build_message(&name);
    // dbg!(result);
    println!("{}", result);
}

fn build_message(name: &str) -> String {
    format!("{}, {}!", "Hello", name)
}