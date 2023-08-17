fn main() {  
}

//----------------------1.1

fn count_vowels(x: &str) -> usize {
    let mut count = 0;

    for i in x.chars() {
        match i {
            'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
                count += 1 ;
            }
            _ => {}
        }
    }
    count
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("a1'2E  *23 I1o2 U2"), 5);
}



//----------------------1.2

fn count_vowels_r(x: &str) -> usize {
    if x.is_empty() {
        return 0;
    } else {
        let char = x.chars().next().unwrap();
        let count = count_vowels_r(&x[1..]);

        let vowel = match char {
            'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => true,
            _ => false,
        };

        if vowel {
            1 + count
        } else {
            count
        }
    }
}

#[test]
fn test_vowels_count1_r() {
    assert_eq!(count_vowels_r(""), 0);
    assert_eq!(count_vowels_r("abEcd"), 2);
    assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("a1'2E  *23 I1o2 U2"), 5);
}



//----------------------1.3

fn count_vowels_v2(x: &str) -> Vec<(String, usize)> {
    let mut result = Vec::new();
    let split = x.split_whitespace();

    for i in split {
        let mut count: usize = 0;
        for c in i.chars() {
            match c {
                'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
                    count += 1 ;
                }
            _ => {}
            }
        }
        result.push((i.to_string(), count));
    }
    result
}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3),
            ("7x8U3y5z".to_string(), 1)
        ]
    );
    assert_eq!(
        count_vowels_v2("1a3ba'bd1 us1=2dj 23jAS_22"),
        [
            ("1a3ba'bd1".to_string(), 2),
            ("us1=2dj".to_string(), 1),
            ("23jAS_22".to_string(), 1)
        ]
    );
}







