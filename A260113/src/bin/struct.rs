
struct  User {
    username:String,
    email:String,
    sign_in_conect:u64,
    acrive:bool,
}

fn main() {
    // user1を定義
    let mut user1 = User {
        email: String::from("helloworld@un.co.jp"),
        username: String::from("hello"),
        acrive:true,
        sign_in_conect:1,
    };
    println!("{}",user1.email);
    user1.email = String::from("helloworld@heikou");
    println!("{}",user1.email);
    //user2を定義
    let user2 =build_user("hello@a", "igger");
    println!("{},{}",user2.email,user2.username);
    //user3を定義
    let user3 = User{
        email: String::from("value@c"),
        username: String::from("value"),

        ..user1
        //上と下意味は同じ
        // sign_in_conect: user1.sign_in_conect,
        // acrive: user1.acrive,
    };
    println!("{},{}",user3.acrive,user3.sign_in_conect);
}

fn build_user(email:&str,username:&str)->User {
    User {
        email:email.to_string(),
        username:username.to_string(),
        sign_in_conect: 1,
        acrive: true,
    }
}