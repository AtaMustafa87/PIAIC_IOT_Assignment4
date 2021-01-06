fn main() {
    let mut score = 2;
    let mut add_2 = ||{
        score = score + 2
    };
    for _ in 0..10 {
        add_2();
    }
    println!("Score = {}",score);
}

