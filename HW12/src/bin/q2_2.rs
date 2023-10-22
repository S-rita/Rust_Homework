trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct PlainText {
    chars: String,
}

#[derive(Clone)]
struct RepeatedText {
    text: Box<dyn Text>,
    count: usize,
}

#[derive(Clone)]
struct JoinedText {
    parts: Vec<Box<dyn Text>>,
    separator: PlainText,
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText { chars: text.to_string() }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.chars.clone()
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

impl RepeatedText {
    fn with_parts(text: &dyn Text, count: usize) -> RepeatedText {
        RepeatedText {
            text: text.clone_box(),
            count,
        }
    }
}

impl Text for RepeatedText {
    fn value(&self) -> String {
        self.text.value().repeat(self.count)
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for RepeatedText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

impl JoinedText {
    fn with_parts(parts: &[Box<dyn Text>], separator: &PlainText) -> JoinedText {
        JoinedText {
            parts: parts.to_vec(),
            separator: separator.clone(),
        }
    }
}

impl Text for JoinedText {
    fn value(&self) -> String {
        let mut result = String::new();
        let mut first = true;

        for part in &self.parts {
            if first {
                first = false;
            } else {
                result.push_str(&self.separator.value());
            }
            result.push_str(&part.value());
        }

        result
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for JoinedText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

#[test]
fn test_text_composition() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}

fn main() {

}
