#[derive(Debug, Clone, PartialEq)]
pub enum POS {
    Verb, 
    Noun,
    Particle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Case {
    Nominative,
    Accusative, 
    Genitive
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbTense {
    Past, 
    Present, 
    Command
}

#[derive(Debug, Clone)]
pub struct Morphology {
    pub surface: String,
    pub pos: POS,
    pub case_: Option<Case>,
    pub definite: bool, 
    pub tanwin: bool,
    pub tense: Option<VerbTense>
}

pub fn analyze(word: &str) -> Morphology {
    let particles = ["و", "ف", "ب", "ك", "ل"];

    if particles.contains(&word) {
        return Morphology { 
            surface: word.to_string(), 
            pos: POS::Particle, 
            case_: None, 
            definite: false, 
            tanwin: false, 
            tense: None 
        };
    }

    // tanwin 
    // damma ُ
    if word.ends_with('ُ') {
        return noun(word, Case::Nominative, true);
    }

    //  fatha
    if word.ends_with('َ') {
        return noun(word, Case::Accusative, false)
    }

    //  kasra
    if word.ends_with('ِ') {
        return noun(word, Case::Genitive,  false);
    }

    // Default fallback
    Morphology {
        surface: word.to_string(),
        pos: POS::Noun,
        case_: None,
        definite: false,
        tanwin: false,
        tense: None,
    }
}

fn noun(word: &str, case_: Case, tanwin: bool) -> Morphology {
    Morphology { 
        surface: word.to_string(), 
        pos: POS::Noun, 
        case_: Some(case_), 
        definite: !tanwin && word.starts_with("ال") , 
        tanwin, 
        tense: None 
    }
}