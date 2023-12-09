#![allow(warnings)]
use std::env;
use std::fmt;
use std::fs;

static example_data: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
/*
 0. Lue tiedosto
 1. Lue rivit vas-oik:
   ota numerot ylös structiin jossa numeron arvo sekä lista jossa numeroiden x-y
   ota merkit ylös listaan jossa x-y per merkki

 2. Alusta summa. Käy läpi jokainen numero structi
   Käy läpi structin lista
     katso onko merkkilistassa merkkiä joka koskettaa numeroa
     jos yksi löytyy, älä summaa
   jos ei löytynyt, lisää numeron arvo kokonaissummaan

 3. tulosta
*/

struct Loc {
    x: i32,
    y: i32,
}

impl Loc {
    pub fn touches(&self, loc: &Loc) -> bool {
        (self.x - loc.x).abs() <= 1 && (self.y - loc.y).abs() <= 1
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) ", self.x, self.y)
    }
}

struct Num {
    locs: Vec<Loc>,
    value: i32,
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut locs = String::from("");
        for loc_str in &self.locs {
            locs.push_str(&loc_str.to_string())
        }
        write!(f, "Locs: {} Value: {}", locs, self.value)
    }
}

fn add_number(build: &str, things: &mut Vec<Num>, x: i32, y: i32) {
    let value = build.to_string().parse::<i32>().unwrap();
    let mut locs: Vec<Loc> = Vec::new();
    for loc_x in x - (build.len() as i32)..x {
        locs.push(Loc { x: loc_x as i32, y });
    }
    let num = Num { locs, value };
    things.push(num);
}

fn parse_row(row: &str, y: i32) -> (Vec<Num>, Vec<Loc>) {
    let mut build = "".to_owned();
    let mut numbers: Vec<Num> = Vec::new();
    let mut symbols: Vec<Loc> = Vec::new();
    for x in 0..row.len() {
        let c = row.chars().nth(x).unwrap();
        if c.is_digit(10) {
            build.push(c);
            if x == row.len() - 1 {
                add_number(&build, &mut numbers, x as i32, y);
                build = "".to_owned();
            }
        } else {
            if build.len() > 0 {
                add_number(&build, &mut numbers, x as i32, y);
                build = "".to_owned();
            }
            if c != '.' {
                let symbol = Loc { x: x as i32, y };
                symbols.push(symbol);
            }
        }
    }
    (numbers, symbols)
}

fn parse_rows(rows: Vec<&str>) -> (Vec<Num>, Vec<Loc>) {
    let mut y = 0;
    let mut numbers: Vec<Num> = Vec::new();
    let mut symbols: Vec<Loc> = Vec::new();
    for row in rows {
        let (mut row_numbers, mut row_symbols) = parse_row(row, y);
        y += 1;
        numbers.append(&mut row_numbers);
        symbols.append(&mut row_symbols);
    }
    (numbers, symbols)
}

fn get_sum(numbers: &Vec<Num>, symbols: &Vec<Loc>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        let mut found = false;
        'loc_loop: for loc in &number.locs {
            for symbol in symbols {
                if loc.touches(&symbol) {
                    found = true;
                    break 'loc_loop;
                }
            }
        }
        if found {
            sum += number.value;
        }
    }
    sum
}

fn compute(data: &str) {
    let rows = data.split("\n").collect::<Vec<&str>>();
    let (numbers, symbols) = parse_rows(rows);
    let answer = get_sum(&numbers, &symbols);
    println!("{}", answer);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() > 1) {
        let file_path = &args[1];
        println!("Reading from file '{}' ...", file_path);
        let data = fs::read_to_string(file_path).expect("Could not read file!\n");
        compute(&data);
    } else {
      compute(example_data);
    }
}
