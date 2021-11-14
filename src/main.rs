extern crate rand;

use rand::Rng;

const BEAKER_SIZE: usize = 4;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const FILLS: &str = "^#$%@&!:><?()+=-~";

fn mv (v: &mut Vec<[usize;BEAKER_SIZE]>, f: usize, t:usize) -> bool {
    if v[f][BEAKER_SIZE-1] > 0 && v[t][0] == 0 {
        
        for i in 0..BEAKER_SIZE {
            if v[f][i] > 0 {
                for j in 0..BEAKER_SIZE {
                    if v[t][j] > 0 {
                        v[t][j-1] = v[f][i];
                        v[f][i] = 0;
                        return true;
                    }
                }
                v[t][BEAKER_SIZE-1] = v[f][i];
                v[f][i] = 0;
                return true;
            }
        }
        false
    } else {
        false
    }
}

fn shuffle (v: &mut Vec<[usize;BEAKER_SIZE]>) {
    let mut random_beakers: Vec<usize> = (0..v.len()).collect();
    loop {
        for _ in 0..200 {
            rand::thread_rng().shuffle(&mut random_beakers);
            mv(v, random_beakers[0], random_beakers[1]);
        }
        if !solved(&v){break;}
    }
}

fn solved(v: &Vec<[usize;BEAKER_SIZE]>) -> bool {
    for tube in v{
        for i in 1..BEAKER_SIZE{
            if tube[0] != tube[i] {
                return false
            }
        }
    }
    true
}

fn show_game(v: &Vec<[usize;BEAKER_SIZE]>){
    let size = v.len();
    let mut out: String;
    
    println!("\n\n\n\nSORT THE LIQUIDS\n");

    for row in 0..BEAKER_SIZE {
        let mut out = "".to_string();
        for b in 0..size {
            out += " |";
            if v[b][row] > 0{
                out += FILLS[v[b][row]..v[b][row]+1].to_string().as_str();
            } else {
                out += " ";
            }
            out += "|";
        }
        println!("{}", out);
    }

    out = "".to_string();
    let mut out_numbers = "".to_string();
    for b in 0..size {
        out += " (_)";
        out_numbers += "  ";
        
        out_numbers += ALPHABET[b..b+1].to_string().as_str();
        out_numbers += " ";
    }
    println!("{}\n{}",out, out_numbers);
    println!("\nType q to quit\nType AB to pour from A to B")
}
fn main() {
    let difficulty = 8;
    if difficulty > FILLS.len()-1 {
        println!("Max difficulty is {}. Goodbye.", FILLS.len()-1);
        return;
    }
    let mut bs: Vec<[usize; BEAKER_SIZE]> = vec![];

    for n in 0..difficulty + 1 {
        bs.push([n; BEAKER_SIZE]);
    }
    shuffle(&mut bs);
    let _ = loop {
        show_game(&bs);
        if solved(&bs) {
            println!("\nYOU ARE A WINNER!");
            break true;
        }
        let mut action = String::new();
        let _ = std::io::stdin().read_line(&mut action).unwrap();
        
        if action == "q\n".to_string() {
            println!("Goodbye!");
            break false;
        } else {
            if ALPHABET.contains(action[0..1].chars().next().unwrap()) {
                let from_col = ALPHABET.chars().position(|c| c == action[0..1].chars().next().unwrap()).unwrap();
                if ALPHABET.contains(action[1..2].chars().next().unwrap()) {
                    let to_col = ALPHABET.chars().position(|c| c == action[1..2].chars().next().unwrap()).unwrap();
                    mv(&mut bs, from_col, to_col);
                }
            }
        }
    }; 
}