fn main(){
    let s1 = gives_ownrship();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{} {}",s1,s3);
}
fn gives_ownrship()-> String{
    let some_string = String::from("hello");// some_stringが所有権を持つ
    some_string// 戻り値として所有権を「呼び出し元」へ放り出す
}

fn takes_and_gives_back(a_string: String)->String{
    a_string // 受け取ったバトン（所有権）をそのまま返す

}