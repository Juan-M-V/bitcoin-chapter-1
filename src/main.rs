use bitcoin_chapter1::FieldElement;
use bitcoin_chapter1::FieldElement2;

fn main() {
    let doce: FieldElement2<17> = FieldElement2(12);
    let uno: FieldElement2<5> = FieldElement2(1);
    let cinco: FieldElement2<17> = FieldElement2(5); 

    println!("12 + cinco en Z17 es = {:?}", doce + cinco)
}