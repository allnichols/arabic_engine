use crate::dependency::{Dependency, Relation};
use crate::morphology::{Morphology, POS, Case};

pub fn parse(sentence: &[Morphology]) -> Vec<Dependency> {
    let mut deps = Vec::new();

    if sentence.is_empty() {
        return deps;
    }

    // verbal sentence - َجملة فعلية
    if sentence[0].pos == POS::Verb {
        deps.push(Dependency { head: 0, dependent: 0, relation: Relation::Root });

        for (i, word) in sentence.iter().enumerate().skip(1) {
            match word.case_ {
                Some(Case::Nominative) => deps.push(Dependency { 
                    head: 0, 
                    dependent: i, 
                    relation: Relation::Subject, 
                }),
                Some(Case::Accusative) => deps.push(Dependency { 
                    head: 0, 
                    dependent: i, 
                    relation: Relation::Object
                }),
                _ => {}
            }
        }
        return deps;
    }

    // nominal sentence
    let mubtada_index = sentence.iter().position(|w|
        w.pos == POS::Noun && w.case_ == Some(Case::Nominative)
    );

    if let Some(i) = mubtada_index {
        deps.push(Dependency { 
            head: i, 
            dependent: i, 
            relation: Relation::Root,
        });

        for (j, _) in sentence.iter().enumerate() {
            if j != i {
                deps.push(Dependency { head: i, dependent: j, relation: Relation::Predicate });
                break;
            }
        }
    }
    deps
}