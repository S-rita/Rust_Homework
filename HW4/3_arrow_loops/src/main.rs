fn main() {

}

//3.1

fn make_arrow1 (v: usize) -> String {
    let mut x = String::new();

    for i in 1..=v {
        x.push_str(&"*".repeat(i));
        x.push('\n');
    }

    for i in (1..v).rev() {
        x.push_str(&"*".repeat(i));
        x.push('\n');
    }

    x
}

#[test]
fn test_make_arrow1() {
    assert_eq!(make_arrow1(0), "");
    assert_eq!(make_arrow1(3), "*\n**\n***\n**\n*\n");
    assert_eq!(make_arrow1(5), "*\n**\n***\n****\n*****\n****\n***\n**\n*\n");
}



//3.2

fn make_arrow2(v: usize) -> String {
    let mut x = String::new();

    for i in 0..v {
        for _ in 0..(v - i - 1) {
            x.push(' ');
        }

        for _ in 0..(2 * i + 1) {
            x.push('*');
        }

        x.push('\n');
    }

    for i in (0..v - 1).rev() {
        for _ in 0..(v - i - 1) {
            x.push(' ');
        }

        for _ in 0..(2 * i + 1) {
            x.push('*');
        }

        x.push('\n');
    }

    x
}

#[test]
fn test_make_arrow2() {
    assert_eq!(make_arrow2(3), "  *\n ***\n*****\n ***\n  *\n");
    assert_eq!(make_arrow2(5), "    *\n   ***\n  *****\n *******\n*********\n *******\n  *****\n   ***\n    *\n");
}


//3.3

fn make_arrow1_recursive (v: usize) -> String {
    if v == 0 {
        String::new()
    } else {
        let mut x = make_arrow1_recursive(v - 1);

        x.push_str(&"*".repeat(v));
        x.push('\n');

        x.push_str(&"*".repeat(v - 1));
        x.push('\n');
        
        x
    }

}

#[test]
fn test_make_arrow1_recursive () {
    assert_eq!(make_arrow1(0), "");
    assert_eq!(make_arrow1(3), "*\n**\n***\n**\n*\n");
    assert_eq!(make_arrow1(5), "*\n**\n***\n****\n*****\n****\n***\n**\n*\n");
}


fn make_arrow2_recursive(v: usize, i: usize, j: usize) -> String {
    if i == 0 {
        String::new();
    }

    let mut x = String::new();

    let s = if i < v {
        v - i - 1
    } else {
        i - v + 1
    };

    for _ in 0..s {
        x.push(' ');
    }

    for _ in 0..(2 * v - 1 - 2 * s) {
        x.push('*');
    }

    x.push('\n');

    x.push_str(&make_arrow2_recursive(v, i + 1, 0));

    x
}

#[test]
fn test_make_arrow2_recursive() {
    assert_eq!(make_arrow2(3), "  *\n ***\n*****\n ***\n  *\n");
    assert_eq!(make_arrow2(5), "    *\n   ***\n  *****\n *******\n*********\n *******\n  *****\n   ***\n    *\n");
}



