use std::collections::HashMap;
use std::fs;

type BfMemMap = [i32; 1000];


fn main() {
    let file_name = "test1.txt";


    let mut mem: BfMemMap = [0; 1000];
    let mut mem_ptr: usize = 0;

    let mut loops: BfMemMap = [0;1000];
    let mut loop_ptr: usize = 0;

    type ValueFunction = fn(&mut BfMemMap, &mut usize, &mut BfMemMap, &mut usize) -> ();

    let mut char_map: HashMap<char, ValueFunction>= HashMap::new();
    char_map.insert('>', inc_ptr);
    char_map.insert('<', dec_ptr);
    char_map.insert('+', |m,mr,_l,_lr| m[*mr]+=1);
    char_map.insert('-', |m,mr,_l,_lr| m[*mr]-=1);
    char_map.insert('[', loop_begin);

    let contents = fs::read_to_string(file_name)
        .expect("Could not read file");

    let in_chars = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .join("");

    for c in in_chars.chars() {
        if let Some(func) = char_map.get(&c) {
            func(&mut mem, &mut mem_ptr, &mut loops, &mut loop_ptr);
        }
    }

    println!("{mem_ptr}");
}

fn inc_ptr(_m: &mut BfMemMap, mr: &mut usize, _l: &mut BfMemMap, _lp: &mut usize) {
    if *mr < 999 {
        *mr+=1;
    } else {
        *mr = 0;
    }
}

fn dec_ptr(_m: &mut BfMemMap, mr: &mut usize, _l: &mut BfMemMap, _lp: &mut usize) {
    if *mr > 0 {
        *mr-=1;
    } else {
        *mr = 1000;
    }
} 

fn loop_begin(m: &mut BfMemMap, mr: &mut usize, l: &mut BfMemMap, lp: &mut usize) {
    l[*lp] = m[*mr+1];
    *lp+=1;
}

/* 
fn loop_end(m: &mut BfMemMap, mr: &mut usize, l: &mut BfMemMap, lp: &mut usize) {
}
*/
