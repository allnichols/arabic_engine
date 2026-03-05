use actix_web::{web, App, HttpServer, HttpResponse, Responder};

pub use morphology::{MorphologyResult, PartOfSpeech};

#[path = "morphology/morphology.rs"]
mod morphology;

#[path = "morphology/handler.rs"]
mod morphology_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let sentence = vec!["الدَّرْسَ", "الطَّالِبُ", "كَتَبَ"];
    // let sentence_with_particle = vec!["ذَهَبَ", "خَالِدٌ", "إِلَى", "الْمَتْجَرِ"];
    // let sentence_with_present_tense = vec!["يَذْهَبُ", "خَالِدٌ", "إِلَى", "الْمَتْجَرِ"];
    // let sentence_with_command = vec!["اِذْهَبْ", "خَالِدٌ", "إِلَى", "الْمَتْجَرِ"]; // work on finding the command tense

    // let morphs: Vec<_> = sentence
    //             .iter()
    //             .map(|w| morphology::analyze(w))
    //             .collect();


    HttpServer::new(|| {
        App::new()
            .route("/analyze", web::get().to(morphology_handler::analyze_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
