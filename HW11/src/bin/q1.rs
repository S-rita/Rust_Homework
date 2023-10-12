fn make_document(x: &str) -> Vec<String> {
    x.split("\n\n").map(|a| a.to_string()).collect()
}

fn rank_documents(x: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut rank_doc = x.to_vec();
    rank_doc.sort_by(|a, b| b.len().cmp(&a.len()));
    rank_doc
}

fn main() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,—\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
    ";
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    println!("Make document: {:?}", docs);
    println!("-------------------------");
    let rnk_docs = rank_documents(&docs);
    println!("Rank document: {:?}", rnk_docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}
