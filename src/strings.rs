use std::collections::HashMap;

pub fn split_words(s: &String) -> Vec<&str> {
    let words: Vec<&str> = s.trim().split(' ').collect();
    words
}

pub fn split_lines(s: &String) -> Vec<&str> {
    let lines: Vec<&str> = s.trim().split('\n').collect();
    lines
}

pub fn grid_2d_char(s: &String) -> Vec<Vec<char>> {
    let lines: Vec<&str>  = s.trim().split('\n').collect();
    let grid: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();

    grid
}

pub fn grid_2d_ints(s: &String) -> Vec<Vec<u32>> {
    let lines: Vec<&str>  = s.trim().split('\n').collect();

    let grid: Vec<Vec<u32>> = lines.into_iter().map(|line| line.trim().chars().map(|ch| ch.to_digit(10).unwrap()).collect()).collect();

    grid
}

pub fn vec_str_to_ints(str_vec: &Vec<&str>) -> Vec<usize> {
    let mut ints: Vec<usize> = Vec::new();

    for num in str_vec {
        ints.push(num.trim().parse()
        .expect("One of the items is not a number"));
    }

    ints
}

pub fn count_chars(s: &String) -> HashMap<char, usize> {
    let mut str_hash_map: HashMap<char, usize> = HashMap::new();

    for ch in s.chars() {
        str_hash_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    str_hash_map
}

pub fn multi_count_chars(vec_ptr_s: &Vec<&str>) -> Vec<HashMap<char, usize>> {
    let mut vec_hash: Vec<HashMap<char, usize>> = Vec::new();
    for line in vec_ptr_s {
        let mut str_hash_map: HashMap<char, usize> = HashMap::new();
        for ch in line.chars() {
            str_hash_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }
        vec_hash.push(str_hash_map);
    }

    vec_hash
}