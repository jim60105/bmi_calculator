// https://ithelp.ithome.com.tw/articles/10336131
use std::io;

mod bmi;

fn main() {
    let height = ask_for_input("請輸入身高(cm): ");
    let weight = ask_for_input("請輸入體重(kg): ");

    let bmi = bmi::calculator(height, weight);
    let message = if bmi >= 0.0 && bmi <= 18.5 {
        "過輕"
    } else if bmi > 18.5 && bmi <= 24.0 {
        "正常"
    } else if bmi > 24.0 && bmi <= 27.0 {
        "過重"
    } else {
        "無法計算"
    };

    println!("你的 BMI 是: {:.1}, 你的體重狀況是: {}", bmi, message)
}

fn ask_for_input(message: &str) -> f64 {
    println!("{}", message);

    let mut input = "".to_string();
    io::stdin().read_line(&mut input).expect("無法讀取");
    input.trim().parse().expect("無法轉換")
}
