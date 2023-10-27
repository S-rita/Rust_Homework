#[derive(Clone)]
enum Text {
    Plain(String),
    Repeated(Box<Text>, usize),
    Joined(Vec<Box<Text>>, Box<Text>),
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t, count) => t.value().repeat(*count),
            Text::Joined(parts, separator) => parts
                .iter()
                .map(|part| part.value())
                .collect::<Vec<String>>()
                .join(&separator.value()),
        }
    }
}

impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> {
        Box::new(t.clone())
    }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        self
    }
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let tvec: Vec<Box<Text>> = vec![t1.into(), t2.into(), t3.into(), t4.into()];
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}

fn main() {}
