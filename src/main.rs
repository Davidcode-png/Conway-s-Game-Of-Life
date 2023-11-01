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
    
    //let mut rng = rand::thread_rng();
    map[4 as usize][4 as usize] = 1;
    map[5 as usize][9 as usize] = 1; 
    map[6 as usize][3 as usize] = 1;
    map[4 as usize][7 as usize] = 1;
    map[5 as usize][5 as usize] = 1; 
    map[6 as usize][8 as usize] = 1;
    map[4 as usize][8 as usize] = 1;
    map[3 as usize][8 as usize] = 1;
    map[2 as usize][8 as usize] = 1;
    map[1 as usize][8 as usize] = 1;
   loop{

        display(&mut map, count);
        calculate_next_generation(&mut map);
        count += 1;
        println!("Generation: {}", count);
        thread::sleep(Duration::from_millis(160));
        clearscreen::clear().expect("Failed to clear screen");
    }
}


fn initialize_grid(map: &mut [[u8; 50]; 50]) {
   
    
    
}

fn count_neighbors(grid: &mut [[u8; 50]; 50], row: usize, col: usize) -> i32 {
    let mut count = 0;
    let mut test_row:i32 ;
    let mut test_col:i32 ;
    let mut value: u8;
    for delta_row in [(row as i32) - 1, 0, (row as i32) + 1].iter().cloned(){
        for delta_col in [(col as i32) - 1, 0, (row as i32) + 1].iter().cloned(){
            if delta_row == 0 && delta_col == 0 {
                continue;
            }
            if delta_row < 0 || delta_col < 0 {
                continue;
            }

            test_row = (row as i32 + delta_row) % 50;
            test_col = (col as i32 + delta_col) % 50;
            //println!("ROWS VALUES ARE {} {}", test_row, test_col);
            value = grid[test_row as usize][test_col as usize];
            //println!("Value is {}", value);
            //thread::sleep_ms(70);
            //println!();
            count += value as i32;
            
        }
    } 
    //println!("Neighbors are {}, row -> {}, col -> {}", count, row, col);
    count
}


fn calculate_next_generation(map: &mut [[u8; 50]; 50]) {
    let mut new_map = [[0; 50]; 50];

    for i in 0..50 {
        for j in 0..50 {
            let neighbors = count_neighbors(map, i, j);
            if neighbors < 2 || neighbors > 3 {map[i][j] = 0;}
            if neighbors == 2 || neighbors == 3 {map[i][j] = 1;}
            
        }
    }

    //for i in 0..50 {
       // for j in 0..50 {
     //       map[i][j] = new_map[i][j];
      //  }
    //}
}

fn display(map: &[[u8; 50]; 50], count: u64) {
    let squares = SquareSymbols::new();
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    println!("Generation: {}", count);
    for row in map {
        for &cell in row.iter() {
            if cell == 0 {
                print!("{}", squares.yellow);
            } else {
                print!("{}", squares.red);
            }
        }
        println!();
    }
}  
