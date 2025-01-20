mod game;

use std::io;

fn main() {
    println!("ジャンケンを始めましょう！");
    // 選択肢を表示
    println!("0: グー, 1: チョキ, 2: パー");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let user_choice: u32 = input.trim().parse().expect("無効な入力");

    let (computer_choice, result) = game::play_janken(user_choice);

    // 数値を文字列に変換
    let choices = ["グー", "チョキ", "パー"];

    println!("あなたの選択: {}", choices[user_choice as usize]);
    println!("コンピュータの選択: {}", choices[computer_choice as usize]);
    println!("{}", result);
}
