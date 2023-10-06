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
    let mut hcat_result = Vec::new();
    let longest1 = img1.iter().map(|x| x.len()).max().unwrap_or(0);
    let longest2 = img2.iter().map(|x| x.len()).max().unwrap_or(0);
    let max = longest1.max(longest2);

    for (i, j) in img1.iter().zip(img2.iter()) {
        let space = " ".repeat(max - i.len());
        let mut concat = format!("{}{}", i, space);
        concat.push_str(j);
        hcat_result.push(concat);
    }

    if img1.len() > img2.len() {
        hcat_result.push(img1[img1.len() - 1].clone());
    } else if img1.len() < img2.len() {
        let space2 = " ".repeat(max);
        let concat2 = format!("{}{}", space2, img2[img2.len() - 1]);
        hcat_result.push(concat2);
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

