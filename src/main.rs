extern crate rand;
extern crate ncurses;

use rand::Rng;
use ncurses::*;
use std::cmp::min;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
const UI_PAIR: i16 = 2;


const BEAKER_SIZE: usize = 4;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const FILLS: &str = "^#$%@&!:><?()+=-~";

type Id = usize;

struct Game {
    difficulty: usize,
    beakers: Vec<[usize; BEAKER_SIZE]>
}
impl Game {
    fn mv (&mut self, f: usize, t:usize) -> bool {
        if self.beakers[f][BEAKER_SIZE-1] > 0 && self.beakers[t][0] == 0 {
            
            for i in 0..BEAKER_SIZE {
                if self.beakers[f][i] > 0 {
                    for j in 0..BEAKER_SIZE {
                        if self.beakers[t][j] > 0 {
                            self.beakers[t][j-1] = self.beakers[f][i];
                            self.beakers[f][i] = 0;
                            return true;
                        }
                    }
                    self.beakers[t][BEAKER_SIZE-1] = self.beakers[f][i];
                    self.beakers[f][i] = 0;
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    fn shuffle (&mut self) {
        let mut random_beakers: Vec<usize> = (0..self.beakers.len()).collect();
        loop {
            for _ in 0..200 {
                rand::thread_rng().shuffle(&mut random_beakers);
                &self.mv(random_beakers[0], random_beakers[1]);
            }
            if !self.solved(){break;}
        }
    }
    
    fn solved(&self) -> bool {
        for tube in &self.beakers {
            for i in 1..BEAKER_SIZE{
                if tube[0] != tube[i] {
                    return false
                }
            }
        }
        true
    }
}

// fn show_game(v: &Vec<[usize;BEAKER_SIZE]>){
//     let size = v.len();
//     let mut out: String;
    
//     println!("\n\n\n\nSORT THE LIQUIDS\n");

//     for row in 0..BEAKER_SIZE {
//         let mut out = "".to_string();
//         for b in 0..size {
//             out += " |";
//             if v[b][row] > 0{
//                 out += FILLS[v[b][row]..v[b][row]+1].to_string().as_str();
//             } else {
//                 out += " ";
//             }
//             out += "|";
//         }
//         println!("{}", out);
//     }

//     out = "".to_string();
//     let mut out_numbers = "".to_string();
//     for b in 0..size {
//         out += " (_)";
//         out_numbers += "  ";
        
//         out_numbers += ALPHABET[b..b+1].to_string().as_str();
//         out_numbers += " ";
//     }
//     println!("{}\n{}",out, out_numbers);
//     println!("\nType q to quit\nType AB to pour from A to B")
// }
//fn backupmain() {
    // let difficulty = 8;
    // if difficulty > FILLS.len()-1 {
    //     println!("Max difficulty is {}. Goodbye.", FILLS.len()-1);
    //     return Ok(());
    // }
    // let mut bs: Vec<[usize; BEAKER_SIZE]> = vec![];

    // for n in 0..difficulty + 1 {
    //     bs.push([n; BEAKER_SIZE]);
    // }
    // shuffle(&mut bs);
    // let _ = loop {
    //     show_game(&bs);
    //     if solved(&bs) {
    //         println!("\nYOU ARE A WINNER!");
    //         break true;
    //     }
    //     let mut action = String::new();
    //     let _ = std::io::stdin().read_line(&mut action).unwrap();
        
    //     if action == "q\n".to_string() {
    //         println!("Goodbye!");
    //         break false;
    //     } else {
    //         if ALPHABET.contains(action[0..1].chars().next().unwrap()) {
    //             let from_col = ALPHABET.chars().position(|c| c == action[0..1].chars().next().unwrap()).unwrap();
    //             if ALPHABET.contains(action[1..2].chars().next().unwrap()) {
    //                 let to_col = ALPHABET.chars().position(|c| c == action[1..2].chars().next().unwrap()).unwrap();
    //                 mv(&mut bs, from_col, to_col);
    //             }
    //         }
    //     }
    // }; 
//}

#[derive(Default)]
struct Ui {
    curr: Option<Id>,
    selected: Option<Id>,
    row: usize,
    col: usize
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize){
        self.row = row;
        self.col = col;
    }
    
    fn label(&mut self, text: &str, pair: i16) {
        ncurses::mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn end_list(&mut self) {
        self.curr = None;
    }
    
    fn end(&mut self) {

    }
}

fn main() {
    let mut selected = -1;

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(UI_PAIR, COLOR_MAGENTA, COLOR_BLACK);

    refresh();
    
    let mut quit = false;

    let mut ui = Ui::default();

    while !quit {
        erase();
        ui.begin(0,0);
        {
            ui.label("Sort the Colors", UI_PAIR);
            
        }
        refresh();
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => ()
        }
    }

    endwin();
}