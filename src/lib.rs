#![allow(dead_code,unused_imports)]

mod random;
mod minesweeper;

use minesweeper::Minesweeper;

use wasm_bindgen::prelude::*;
use std::cell::RefCell;

thread_local! {
    static MINESWEEPER:RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10,10,5));
}
#[wasm_bindgen]
extern "C" {
    fn alert(s:&str);
}

#[wasm_bindgen]
pub fn greet(name:&str) {
    alert(&format!("hello {}!",name));
}


















// testing
#[cfg(test)]
mod tests {
    use crate::random::random_range;
    use crate::minesweeper::*;
    const TEST_WIDTH:usize = 10;
    const TEST_HEIGHT:usize = 10;
    const TEST_MINE_COUNT:usize = 5;
    const TEST_X:usize = 3;
    const TEST_Y:usize = 7;
    use std::str::FromStr;

    #[test]
    fn test_random() {
        for _ in 0..10 {
            println!("{}",random_range(0, 100));
        }
    }


    #[test]
    fn test_debug_minesweeper() {
        let ms = Minesweeper::new(TEST_WIDTH,TEST_HEIGHT,TEST_MINE_COUNT);
        println!("{:?}",ms);
    }
    #[test]
    fn test_display_minesweeper() {
        let ms = Minesweeper::new(TEST_WIDTH,TEST_HEIGHT,TEST_MINE_COUNT);
        println!("{}",ms);
    }
    #[test]
    fn test_open() {
        let mut ms = Minesweeper::new(TEST_WIDTH,TEST_HEIGHT,TEST_MINE_COUNT);
        ms.open((TEST_X,TEST_Y));
        println!("{}",ms);                
    }   

    #[test]
    fn test_flag() {
        let mut ms = Minesweeper::new(TEST_WIDTH,TEST_HEIGHT,TEST_MINE_COUNT);
        
        ms.toggle_flag((TEST_X,TEST_Y));
        
        println!("{}",ms);  
    }

    #[test]
    fn simulate_game() {
        let mut line = String::new();
        println!("Enter the width,height,mine-count");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        line.retain(|c| !c.is_whitespace()); // remove all white space in the string
        let parts = line.split(",").collect::<Vec<&str>>(); // break string to vector 
        let w = usize::from_str(parts[0]).unwrap_or(TEST_WIDTH);
        let h = usize::from_str(parts[1]).unwrap_or(TEST_HEIGHT);
        let c = usize::from_str(parts[2]).unwrap_or(TEST_MINE_COUNT);
        let mut ms = Minesweeper::new(w,h,c);
        println!("{}",ms);

        loop {
            line.clear();
            println!("Enter x,y");
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            line.retain(|c| !c.is_whitespace());// remove all white space in the string
            if line == String::from_str("exit").unwrap() {
                break;
            }

            let parts = line.split(",").collect::<Vec<&str>>(); // break string to vector
            let x = usize::from_str(parts[0]).unwrap_or(TEST_X);
            let y = usize::from_str(parts[1]).unwrap_or(TEST_Y); 
            

            line.clear();
            println!("Enter action: 'open','flag'");
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            line.retain(|c| !c.is_whitespace());// remove all white space in the string
            if line.as_str() == "open" {
                ms.open((x,y));
            } else if line.as_str() == "flag" {
                ms.toggle_flag((x,y));
            } else {
                println!("how did u fuck up?");
            }
            println!("{}",ms); 
            if ms.check_win() {
                println!("WON");
                break;
            } else if ms.is_game_over() {
                println!("LOST");
                break;
            }           
        }
    }

}
