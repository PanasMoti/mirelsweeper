use std::{collections::HashSet, fmt::{Display, Write}};
use crate::random;

pub type Position = (usize,usize);

#[derive(Debug)]
pub enum OpenResult {
    Mine,
    NoMine(u8),
}




#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    pub lost: bool,

}


impl Display for Minesweeper {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x,y);
                if !self.open_fields.contains(&pos) {
                    if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟩 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    write!(f," {} ",self.neighboring_mines(pos))?;
                    
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}


impl Minesweeper {
    pub fn new(width:usize,height:usize,mine_count:usize) -> Minesweeper {
        Minesweeper {
            width,height, 
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                
                while mines.len() < mine_count {
                    let px = random::random_range(0, width);
                    let py = random::random_range(0, height);
                    mines.insert((px,py));
                }

                mines
            },
            flagged_fields: HashSet::new(),
            lost : false        
        }
    }

    pub fn neighbors(&self,(x,y):Position) -> Vec<Position> {
        let mut v = Vec::new();
        let min_i = if x == 0 {0} else {-1};
        let max_i = if x == self.width-1 {1} else {2};
        let min_j = if y == 0 {0} else {-1};
        let max_j = if y == self.height-1 {1} else {2};
        for j in min_j..max_j {
            for i in min_i..max_i {
                let px = x as i32;
                let py = y as i32;
                    let qx = (px + i) as usize;
                    let qy = (py + j) as usize;
                    v.push((qx,qy));


            }
        }
        v
    }

    

    pub fn neighboring_mines(&self, pos:Position) -> u8 {
        let n = self.neighbors(pos);
        let mut count:u8 = 0;
        for cell in n {
            if self.mines.contains(&cell) {
                count += 1;
            }
        }
        count
    }

    pub fn toggle_flag(&mut self,pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
        
    }

   

    pub fn open(&mut self,position:Position) -> Option<OpenResult> {
        if self.lost || self.open_fields.contains(&position) || self.flagged_fields.contains(&position) {
            return None;
        }
        self.open_fields.insert(position);
        let is_mine = self.mines.contains(&position);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);
            if mine_count == 0 {
                for neighbor in self.neighbors(position) {
                    self.open(neighbor);
                }
            }

            Some(OpenResult::NoMine(mine_count))
        }
    }

    pub fn check_win(&mut self) -> bool {
        self.flagged_fields == self.mines
    }

    
}