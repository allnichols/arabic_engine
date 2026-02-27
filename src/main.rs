use arabic_engine::morphology::analyze;
use arabic_engine::grammar::parse;
fn main() {
    let sentence = vec!["الدَّرْسَ", "الطَّالِبُ", "كَتَبَ"];

    let morphs: Vec<_> = sentence
                .iter()
                .map(|w| analyze(w))
                .collect();

    let deps = parse(&morphs);

    println!("Morphology:");
    for m in &morphs {
        println!("{:?}", m);
    }

    println!("\nDependencies");
    for d in &deps {
        println!("{:?}", d);
    }

}
