// use std::string;

fn main (){
    let mut s = String::from("hello");

    let s1= &s;
    println!("{}",s1);
    let s2 = calculate_length(&mut s);
    println!("{}",s2);
} 
fn calculate_length(some_string: &mut String)-> &str {
    some_string.push_str(", world");
    some_string
} 






// let x = 5;
// let y = x.clone();
// println!("{} {}",x,y);
//        let mut s = String::from("hello");
//        s.push_str(", world");

//     let mut s = String::from("Hello"); // ヒープに確保
//     s.push_str(", Ubuntu!");          // 柔軟にサイズを拡張
//     println!("{}", s);                // "Hello, Ubuntu!" と表示