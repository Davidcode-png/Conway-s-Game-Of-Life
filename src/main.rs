extern crate rand;
extern crate clearscreen;

use rand::Rng;
use std::{
    io:: {stdout, Write},
    thread,
    time::Duration,
};

use clearscreen::clear;



struct SquareSymbols {
    black: &'static str,
    white: &'static str,
    red: &'static str,
    blue: &'static str,
    brown: &'static str,
    orange: &'static str,
    yellow: &'static str,
    green: &'static str,
}

impl SquareSymbols{
    const fn new() -> SquareSymbols{
        SquareSymbols{
            black: "\u{25A0}",
            white: "\u{25A1}",
            blue: "\u{1F7E6}",
            red: "\u{1F7E5}",
            brown: "\u{1F7EB}",
            orange: "\u{1F7E7}",
            yellow: "\u{1F7E8}",
            green: "\u{1F7E9}",
        }
    }
}

fn main() {
    let mut count: u64 = 0;
    let mut map: [[u8; 50]; 50] = [[0; 50]; 50];
    loop{

        display(count, &mut map);
        count += 1;
        println!("Generation: {}", count);
        thread::sleep(Duration::from_millis(160));
        clearscreen::clear().expect("Failed to clear screen");
    }
}


fn display(count: u64,  map: &mut [[u8; 50]; 50]){
    let squares = SquareSymbols::new();
    
    //let mut map: [[u8; 50]; 50] = [[0; 50]; 50];
    
    //let mut count: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    for i in 0..50 {
        for j in 0..50 {
            // map[i][j] = (count as u8 % 2);
            //map[i][j] = rand::thread_rng().gen_range(0..2); 
            col = count as i32 % 50;
            row = count as i32 / 49; 
            map[row as usize][col as usize] = 1;

        }
    }

    //print!("\x1B[1;1H");
       for row in map{
        for &cell in row.iter(){
            if cell == 0{
                print!("{}", squares.yellow);
            }
            else {
                print!("{}", squares.red);
            }
        }
        println!();
    }
} 
