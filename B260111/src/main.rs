use core::num;


fn main() {
    // let mut count = 0;
    // 'counting_up:loop {
    //     println!("The count:{}",count);
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaing:{}",remaining);
    //         if remaining == 9{
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count +=1;
    // }
    // println!("End count is :{}",count);
    let x =[10,20,30,40,50];

    for element in x{
        println!("the x is:{}", element );
    }

    for number in (1..4).rev(){
        println!("{}",number);
    }

    println!("LIFTOFF!!!");
}
// fn Fire_number(){
//     let mut number = 3;

//     while number != 0{
//         println!("{}!",number);

//         number -= 1;
//     }
//     println!("LIFTOFF!");
// }
