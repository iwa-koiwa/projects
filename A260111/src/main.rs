use core::num;

fn main() {
    let mut count = 0;
    'counting_up:loop {
        println!("The count:{}",count);
        let mut remaining = 10;
        loop {
            println!("remaing:{}",remaining);
            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count is :{}",count);
}


// fn another_function(x:i32,s:char){
//     println!("the value number: {}",x);
//     println!("{}",s);
// }
// fn floots(num:f64){
//     println!("{}",num);
// }
// fn guess_change(guess:u32){
//      let guess: u32 = "41".parse().expect("Not a number:");
//      println!("{}",guess);
// }//     // println!("{}",guess);
//fn nots_use(){
    // let  x = 5;
    // let x = x + 1;
    // println!("{}",x);
    // {
    //     let x = x * 2;
    //     println!("{}",x);
    // }
    // const MAX_POINT:u32 = 100_000;
    // print!("{}",MAX_POINT);
    // let spaces = "   ";
    // let spaces = spaces.len();
    //let conditon = true;
    // let number = if conditon {5} else {6};
//}
