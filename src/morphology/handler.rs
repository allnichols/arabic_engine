use actix_web::{ HttpResponse, Responder};
use crate::morphology::analyze;

pub async fn analyze_handler() -> impl Responder {
    let sentence = vec!["الدَّرْسَ", "الطَّالِبُ", "كَتَبَ"];
    let morphs: Vec<_> = sentence
                .iter()
                .map(|w| analyze(w))
                .collect();

    for morph in &morphs {
        println!("{:?}", morph);
    }

    HttpResponse::Ok().json(morphs)

}