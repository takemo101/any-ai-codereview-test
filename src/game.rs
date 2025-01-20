use rand::Rng;

pub fn play_janken(user_choice: u32) -> (u32, String) {
    let computer_choice = rand::thread_rng().gen_range(0..=2);
    let result = match (user_choice, computer_choice) {
        (0, 1) | (1, 2) | (2, 0) => "あなたの勝ち！".to_string(),
        (1, 0) | (2, 1) | (0, 2) => "コンピュータの勝ち！".to_string(),
        _ => "引き分け！".to_string(),
    };
    (computer_choice, result)
}
