use bitcoin_chapter1::FieldElement;

fn main() {
    let field_elem = FieldElement {prime: 2, num: 1};
    println!("{:?}", field_elem);
}
