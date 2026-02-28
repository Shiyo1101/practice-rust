use std::io;

fn main() {
    let hands = ["グー", "チョキ", "パー"];
    let judges = [
        [0, -1, 1], // グー
        [1, 0, -1], // チョキ
        [-1, 1, 0], // パー
    ];
    let mut wins = 0;

    println!("じゃんけんゲームを始めます。");

    loop {
        println!("--------------------------------------------------");
        println!("出す手を選んでください (0: グー, 1: チョキ, 2: パー):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました。");

        let player_hand: usize = match input.trim().parse() {
            Ok(num) if num < 3 => num,
            _ => {
                println!("無効な入力です。0, 1, または 2 を入力してください。");
                continue;
            }
        };

        let computer_hand: usize = rand::random_range(0..hands.len());
        println!(
            "あなたの手: {}, コンピュータの手: {}",
            hands[player_hand], hands[computer_hand]
        );

        // 勝敗の判定
        let judge = judges[player_hand][computer_hand];
        if judge == 0 {
            println!("引き分けです。");
        } else if judge == 1 {
            println!("あなたの勝ちです！");
            wins += 1;
        } else {
            println!("コンピュータの勝ちです。");
            break;
        }

        if wins == 3 {
            println!("おめでとうございます！3連勝しました！");
            break;
        }
    }
}
