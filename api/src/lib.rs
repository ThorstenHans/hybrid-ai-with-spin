use anyhow::Result;
use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::{http_component, llm};

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/api/infer", infer);
    Ok(router.handle(req))
}

fn infer(req: Request, _: Params) -> Result<impl IntoResponse> {
    let model = serde_json::from_slice::<UserQuestion>(req.body())?;
    let prompt = build_prompt(model.question);
    println!("Asking the model: {}", prompt);
    let options = llm::InferencingParams {
        max_tokens: 1000,
        temperature: 0.2,
        ..llm::InferencingParams::default()
    };
    let response =
        llm::infer_with_options(llm::InferencingModel::Llama2Chat, prompt.as_str(), options);

    Ok(match response {
        Ok(v) => {
            println!("Received answer from LLM: {:?}", v);
            ResponseBuilder::new(200)
                .header("content-type", "text/plain")
                .body(v.text)
                .build()
        }
        Err(e) => {
            println!("Error while calling LLM: {}", e);
            Response::new(500, "Internal Server Error")
        }
    })
}

const SYSTEM_PROMPT: &str = r#"You're an AI assistant called Linus responsible for answering only questions in the realm of software engineering and development. 
Answer as short as possible.
Answer in plain text only."#;

fn build_prompt(user_prompt: String) -> String {
    format!("{}\n{}", SYSTEM_PROMPT, user_prompt)
}

#[derive(Deserialize)]
pub struct UserQuestion {
    #[serde(rename = "question")]
    pub question: String,
}
