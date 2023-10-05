fn main() {
}

fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut vcat_result = img1.to_vec();
    for i in img2 {
        vcat_result.push(i.to_string());
    }
    vcat_result
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let max_length = img1
        .iter()
        .chain(img2.iter())
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    let mut hcat_result = Vec::new();

    for (line1, line2) in img1.iter().zip(img2.iter()) {
        let pad1 = max_length - line1.len();
        let pad2 = max_length - line2.len();
        let concatenated_line = format!("{}{}{}{}", line1, " ".repeat(pad1), line2, " ".repeat(pad2));
        hcat_result.push(concatenated_line);
    }

    hcat_result
}


#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
        vcat(&data, &data),
        ["<--", "#####", "<==", "<--", "#####", "<=="]
    );
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(hcat(&data, &data[..2]), ["<--  <--", "##########", "<=="]);
    assert_eq!(hcat(&data[..2], &data), ["<--  <--", "##########", "     <=="]);
}

