use std::io;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;

fn main() {
    println!("[number-baseball] Guess the three numbers. Get it right within 9 rounds.");

    let mut rng = rand::thread_rng();
    let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut irs = Irs::default();

    if let Err(e) = irs.shuffle(&mut input, &mut rng) {
        println!("Shuffle failed. terminate. <reason:{e}>");
        return;
    }

    let correct: &[i32] = &input[1..=3];

    for n in 1..=9 {
        println!("[Round {n}] Enter 3 numbers separated by spaces (EX: 1 2 3)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let inputs: Vec<i32> = guess.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        if inputs.len() != 3 {
            println!("denied. just input 3 numbers");
            continue;
        }

        let mut strike = 0;
        let mut ball = 0;
        let mut out = 3;

        for i in 0..=2 {
            let index = correct.iter().position(|&x| x == inputs[i]);            
            if !index.is_none() {
                if index.unwrap() == i {
                    strike += 1;
                }
                else {
                    ball += 1;
                }

                out -= 1;
            }
        }

        if strike == 3 {
            println!("[Victory] Success guess 3 numbers");
            return;
        } else {
            println!("[Answer] <{strike} Strike, {ball} Ball, {out} Out>");
        }
    }

    println!("[Game Over] failed Guess 3 numbers");

}