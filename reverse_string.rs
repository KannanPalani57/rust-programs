

fn main() {
    let word = "Reverse String";
    
    let word_chars: Vec<char> = word.chars().collect();
    
    
    
    let mut length_word = word.len();
    
    println!("{:?} {}", word_chars, length_word);
    
        
    let mut reversed_word = String::new();

    
    loop {

            
        if length_word == 0 {
            break;
        }    
        
        length_word -= 1;
                
        reversed_word.push(word_chars[length_word]);
    }
    
    println!("reversed_word {}", reversed_word);
    
    
}
