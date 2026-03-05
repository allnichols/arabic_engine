use serde::{Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PartOfSpeech {
    Noun,     // ism
    Verb,     // fi'l
    Particle, // حرف جر
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Tense {
    Past,
    Present,
    Future,
    Command,
}

#[derive(Debug, Clone, Serialize)]
pub struct MorphologyResult {
    pub part_of_speech: PartOfSpeech,
    pub tense: Option<Tense>,
    pub word: String
}

pub fn analyze(word: &str) -> MorphologyResult {
    let mut result = MorphologyResult {
        part_of_speech: PartOfSpeech::Particle,
        tense: None,
        word: word.to_string(),
    };

    if is_noun(word) {
        result.part_of_speech = PartOfSpeech::Noun;
    } else if is_verb(word) {
        result.part_of_speech = PartOfSpeech::Verb;
        result.tense = Some(determine_verb_tense(word));
    } else if is_particle(word) {
        result.part_of_speech = PartOfSpeech::Particle;
    }

    result
}

fn is_noun(word: &str) -> bool {
    let tanween = ['ً', 'ٌ', 'ٍ'];

    // check beginning of the word for definite article "ال"
    let chars: Vec<char> = word.chars().collect();
    if chars.len() >= 2 && chars[0] == 'ا' && chars[1] == 'ل' {
        return true;
    }

    // check end of the word for feminine marker "ة" or tanween
    if let Some(last) = chars.last() {
        if *last == 'ة' || tanween.contains(last) {
            return true;
        }
    }

    false
}

fn is_verb(word: &str) -> bool {
    let verb_prefixes = ['ي', 'ت', 'أ', 'ن'];
    let chars: Vec<char> = word.chars().collect();

    if let Some(last) = chars.last() {
        if *last == 'َ' {
            return true
        }
    }
    

    if let Some(first) = word.chars().next() {
        verb_prefixes.contains(&first)
    } else {
        false
    }
}

fn is_particle(word: &str) -> bool {
    !is_noun(word) && !is_verb(word)
}

fn determine_verb_tense(word: &str) -> Tense {
    let chars: Vec<char> = word.chars().collect();
    if let Some(first) = chars.first() {
        match first {
            'ي' | 'ت' | 'أ' | 'ن' => Tense::Present,
            'س' => Tense::Future,
            'ا' => {
                Tense::Command
            },
            _ => Tense::Past,
        }
    } else {
        Tense::Past
    }
}