fn main(){
    let st = String::from("hello, world!");
    let result = first_word(&st);
    println!("最初の単語は {} 文字です", result);
}
fn first_word(string_lets: &String) -> usize {
    let bytes = string_lets.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    string_lets.len()
}