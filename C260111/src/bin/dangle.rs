fn main(){
    let  reference_to_nothig = dangle();

    println!("{}",reference_to_nothig);
}

fn dangle() -> &String {
    let s = String::from("hello") ;

    &s
}
