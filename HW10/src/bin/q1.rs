fn main(){
}

fn vflip(img: &[String]) -> Vec<String> {
    let mut v_result: Vec<String> = Vec::new();
    for i in img {
        v_result.insert(0, i.to_string());
    }
    v_result
}

fn hflip(img: &[String]) -> Vec<String> {
    let max_len = img.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut h_result = Vec::new();
    for i in img {
        let mut char = String::new();
        for c in i.chars() {
            char.insert(0, c);
        }
        while char.len() < max_len {
            char.insert(0, ' ');
        }
        h_result.push(char)
    }
    h_result
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vflip(&data), ["<==", "#####", "<--"]);
    assert_eq!(hflip(&data), ["  --<", "#####", "  ==<"]);
}
