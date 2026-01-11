fn main() {
    let x = String::from("hello");

    takes_ownership(x);
    
    let x = 5;
    makes_copy(x);
}
fn takes_ownership(some_string: String){
    println!("{}",some_string);
}
fn makes_copy(smoe_inteager: i32){
    println!("{}",smoe_inteager);
}
//helloworld