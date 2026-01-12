fn main() {
    let my_string = String::from("hello, world");

    //let word = first_word(&my_string[..],0);

    //println!("word:{}",word);

    //let my_string_literal = "hello, world";

    //let word = first_word(&my_string_literal[..],0);

    //println!("word:{}",word);

    let (word,index_word) = first_word(&my_string);    

    println!("The first word is:{}",word);

    let word = secound_word(&my_string, index_word+1);

    println!("The Secound word is:{}", word);
} 
fn first_word(s:&str)->(&str,usize){
    let bytes = s.as_bytes();

    for (i,&iter) in bytes.iter().enumerate() {
        if iter ==b' '{
            return (&s[0..i],i);
        }
    }
    let n =s.len();
    (&s[..],n)
}
fn secound_word(s:&str,n:usize)->&str{
    let bytes = s.as_bytes();

    for (i,&iter) in bytes.iter().enumerate().skip(n){
        if iter ==b' '{
            return &s[n..i];
        }
    }
    &s[n..]
}

// fn Secound_world(s:&String)->(usize,usize) {

// }