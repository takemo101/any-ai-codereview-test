mod game;

use std::io;

fn main() {
    println!("ジャンケンを始めましょう！");
    println!("0: グー, 1: チョキ, 2: パー");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let user_choice: u32 = input.trim().parse().expect("無効な入力");

    let (computer_choice, result) = game::play_janken(user_choice);

    println!("あなたの選択: {}", user_choice);
    println!("コンピュータの選択: {}", computer_choice);
    println!("{}", result);
}
