use std::io;

fn main(){

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("失敗しました");

    let input_string = input_string.trim_end();

    let mut index = 0;
    let max_string = input_string.len();

    loop{
        let word;
        (word, index) = words(&input_string, index);
        println!("{}",word);
        if index >= max_string {
            break;
        }
        index +=1;
    }
}

fn words(s:&str,n:usize)->(&str,usize){
    let bytes = s.as_bytes();

    for (i,&iter) in bytes.iter().enumerate().skip(n){
        if iter ==b' '{
            return (&s[n..i],i);
        }
    }
    (&s[n..],s.len())
}