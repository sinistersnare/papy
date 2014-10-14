extern crate papy;

use papy::{tokenize_str, Definition, Comment, Item, LangNumber, LangName, Other};

#[test]
fn test_tokenizer() {
    println!("def: {}", tokenize_str("def thing x y: x y + end"));
    assert!(tokenize_str("def thing x y: x y + end") == vec![Definition("def thing x y: x y + end")])
    assert!(tokenize_str("#def thing x y: x y + end") == vec![Comment("#def thing x y: x y + end")])
    assert!(tokenize_str("1") == vec![Item(LangNumber(1))])
    assert!(tokenize_str("name") == vec![Item(LangName("name"))])
    assert!(tokenize_str("name!") == vec![Item(LangName("name!"))])

}
