# AOC_Utility

AOC_Utility is a Rust utility library made for advent of code. This utility is not made of solutions but rather tools to remove tasks that are repeated on a consistent basis. 

# Available Functions
```
pub fn split_words(s: &String) -> Vec<&str>

pub fn split_lines(s: &String) -> Vec<&str>

pub fn split_char(s: &String, split_c: char) -> Vec<&str>

pub fn vec_str_to_ints(str_vec: &Vec<&str>) -> Vec<usize>
```

## Documentation

```pub fn split_words(s: &String) -> Vec<&str>```

This function takes in a reference to a string. Then it splits the string into splices by delimiting based on spaces. All spaces are removed no matter the amount of spaces between words.

```
pub fn split_lines(s: &String) -> Vec<&str>
```
This function takes in a reference to a string. Then it splits the string into splices by delimiting based on ```\n```, new lines. All new lines are removed no matter the amount of new lines between lines.
