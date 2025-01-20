mod game;

use std::io;

fn main() {
    println!("ジャンケンを始めましょう！");
    println!("0: グー, 1: チョキ, 2: パー");
    println!("終了するには 'exit' と入力してください。");

    // ジャンケンの勝敗がつくまでループ
    loop {
        // ユーザーの選択を受け取る
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");
        let input = input.trim();

        // 'exit' と入力された場合、ループを抜ける
        if input.eq_ignore_ascii_case("exit") {
            println!("ゲームを終了します。");
            break;
        }

        let user_choice: u32 = input.parse().expect("無効な入力");

        // ジャンケンの結果を取得
        let (computer_choice, result) = game::play_janken(user_choice);

        // 数値を文字列に変換
        let choices = ["グー", "チョキ", "パー"];

        println!("あなたの選択: {}", choices[user_choice as usize]);
        println!("コンピュータの選択: {}", choices[computer_choice as usize]);
        println!("{}", result);

        // 引き分けの場合は再度ジャンケンを行う
        if result != "引き分け！" {
            break; // 引き分けでなければループを抜ける
        } else {
            println!("引き分けです。もう一度やりましょう！");
        }
    }
}
