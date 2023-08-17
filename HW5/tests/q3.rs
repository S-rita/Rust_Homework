fn main(){
    
}

//----------------------3.1

fn extract_quoted_words (x: &str) -> Vec<String> {
    let mut result = Vec::new();
    let split: Vec<&str> = x.split_whitespace().collect();

    for i in split {
        let char: Vec<char> = i.chars().collect();
        if char.len() >= 2 {
            if char[0] == '*' && char[char.len() - 1] == '*' {
                let w = &i[1..];
                let words = &w[..w.len() - 1];
                result.push(words.to_string())
            }
        } else if char.len() == 2 {
            if i == "**" {
                result.push("".to_string())
            }
        }
    }
    result
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words(&("C ** *C++* *Java *Python* Rust*")),
        vec!["", "C++", "Python"]
    );
    assert_eq!(
        extract_quoted_words(&("*** *' ** oi* *ei*")),
        vec!["*", "", "ei"]
    );
}

//----------------------3.2

fn extract_quoted_words_r (x: &[&str]) -> Vec<String> {
    if x.len() == 0 {
        Vec::new()
        
    } else {
        let split: Vec<&str> = x[0].split(" ").collect();
        let mut result: Vec<String> = extract_quoted_words_r(&x[1..]);

        for i in split {
            let char: Vec<char> = i.chars().collect();

            if char.len() >=2 {
                if char[0] == '*' && char[char.len() - 1] == '*' {
                    let w = &i[1..];
                    let words = &w[..w.len() - 1];
                    result.push(words.to_string())
                }
            } else if char.len() == 2 {
                if i == "**" {
                    result.push("".to_string())
                }
            }
        }
        result
    }
}


#[test]
fn test_extract_quoted_words_r() {
    assert_eq!(extract_quoted_words_r(&[""]), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words_r(&["C ** *C++* *Java *Python* Rust*"]),
        vec!["", "C++", "Python"]
    );
    assert_eq!(
        extract_quoted_words_r(&["*** *' ** oi* *ei*"]),
        vec!["*", "", "ei"]
    );
}