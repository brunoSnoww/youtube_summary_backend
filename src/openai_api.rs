use dotenv::dotenv;
use openai_api_rust::{
    chat::{ChatApi, ChatBody},
    Auth, Message, OpenAI, Role,
};
/// Summarizes the given text using OpenAI's Chat Completion API.
///
/// # Arguments
///
/// * `text` - The text to be summarized.
///
/// # Returns
///
/// * `Ok(String)` containing the summary if successful.
/// * `Err` if an error occurs.
pub fn summarize_text(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Load the OpenAI API key from environment variables
    dotenv().ok();
    let auth = Auth::from_env()?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    // Prepare the messages for the chat completion
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(150),
        temperature: Some(0.7),
        top_p: Some(1.0),
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![
            Message {
                role: Role::System,
                content: "You are an assistant that summarizes transcripts.".to_string(),
            },
            Message {
                role: Role::User,
                content: format!(
                    "Please provide a concise summary of the following transcript:\n\n{}",
                    text
                ),
            },
        ],
    };

    // Send the request to OpenAI
    let rs = openai.chat_completion_create(&body).unwrap();
    let choices = rs.choices;

    if choices.is_empty() {
        return Err("No completion choices found".into());
    }

    let message = &choices[0].message.as_ref().ok_or("No message in choice")?;

    Ok(message.content.clone())
}
