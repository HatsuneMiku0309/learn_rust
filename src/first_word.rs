pub fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn first_word_optimization(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[allow(dead_code)]
// #[cfg(test)]
pub fn learn_demo() {
    let str = "नमस्ते";
    println!("[{FILE_NAME}][29] :: char ");
    for b in str.chars() {
        print!("  {}", b);
    }
    println!("");
    println!("[{FILE_NAME}][34] :: bytes");
    for b in str.bytes() {
        print!("  {}", b);
    }
    println!("");
}

static FILE_NAME: &str = "first_word.rs";
