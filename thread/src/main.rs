use std::thread;
use std::time::Duration;
fn main() {
    let mut goal = 2;
   
    let multiply_goal = thread::spawn(move || {
        for _ in 1..15 {
            goal = goal * 2;
            println!("Goal = {}",goal);
            thread::sleep(Duration::from_millis(20));
        }
    });
    multiply_goal.join().unwrap();
    println!("Goal = {}",goal);
}
