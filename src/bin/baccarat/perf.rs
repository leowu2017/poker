extern crate poker;

use poker::games::baccarat::game::Game;
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

static COUNT: AtomicU32 = AtomicU32::new(0);

fn play_loop() {
    const DECK_NUM: usize = 8;
    loop {
        let mut game = Game::new(DECK_NUM);
        for _ in 0..70 {
            game.play_one_round();
            COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }
}

fn eval() {
    let count = COUNT.fetch_and(0, Ordering::Acquire);
    println!("{} games per secs.", count);
}

fn eval_loop() {
    loop {
        thread::sleep(Duration::from_secs(1));
        eval();
    }
}

fn main() {
    let play_handle = thread::spawn(play_loop);
    let eval_handle = thread::spawn(eval_loop);
    play_handle.join().unwrap();
    eval_handle.join().unwrap();
}
