pub fn split_words(s: &String) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    let bytes: &[u8] = s.trim().as_bytes();
    let mut start: usize = 0;

    for (i, c) in bytes.iter().enumerate() {
        if *c==b' ' && start!=i {
            words.push(&s[start..i]);
            start = i+1;
        } else if *c==b' ' {
            start = i+1
        }
    }
    words.push(&s[start..]);
    return words;
}

pub fn split_lines(s: &String) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    let bytes: &[u8] = s.trim().as_bytes();
    let mut start: usize = 0;

    for (i, c) in bytes.iter().enumerate() {
        if *c==b'\n' && start!=i {
            words.push(&s[start..i]);
            start = i+1;
        } else if *c==b'\n' {
            start = i+1
        }
    }
    words.push(&s[start..]);
    return words;
}

pub fn split_char(s: &String, split_c: char) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    let bytes: &[u8] = s.trim().as_bytes();
    let mut start: usize = 0;
    let c8: u8 = split_c as u8;

    for (i, c) in bytes.iter().enumerate() {
        if *c==c8 && start!=i {
            words.push(&s[start..i]);
            start = i+1;
        } else if *c==c8 {
            start = i+1
        }
    }
    words.push(&s[start..]);
    return words;
}

pub fn vec_str_to_ints(str_vec: &Vec<&str>) -> Vec<usize> {
    let mut ints: Vec<usize> = Vec::new();

    for num in str_vec {
        ints.push(num.trim().parse()
        .expect("One of the items is not a number"));
    }

    return ints;
}