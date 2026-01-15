use chrono::{Datelike, Local, NaiveDate};
use colored::Colorize;


fn main() {
    let now = Local::now();
    let year = now.year();

    let ferris = r#"

 \\   _~^~~~~~^~_   //
  \) /    o o    \ (/   -------------------
    '_     -     _'     > RUSTだよー       
    / '---------' \     -------------------
    "#;
    println!("{}", ferris.green());

    println!("{}", format!("      === {}年 カレンダー ===", year).green().bold());
    
    for month in 1..=12{
        println!("\n{}", format!("          {}月", month).green().bold());
        println!("{}", "    ---------------------------".yellow());
        println!("{}", "     日  月  火  水  木  金  土 ".green());

        // その月の初日の情報を取得
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");
        let start_weekday = first_day.weekday().num_days_from_sunday();

        // その月の末日を計算
        let last_day = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .unwrap()
        .pred_opt()
        .unwrap()
        .day();

        // 行の描画
        print!("    "); // 左側の余白
        for _ in 0..start_weekday {
            print!("    ");
        }

        for day in 1..=last_day {
            print!("{}", format!("{:>3} ", day).blue());

            // 土曜日で改行（開始位置からのオフセットで計算）
            if (start_weekday + day) % 7 == 0 {
                print!("\n    "); // 次の行の左側余白
            }
        }
        println!("\n{}", "    ---------------------------".yellow());
    }
    
        println!("{}", "\n    Powered by Rust & Ubuntu 25.10".bright_black().italic());
}