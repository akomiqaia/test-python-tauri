use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let model = Llama::phi_3().await?;
  let mut chat = Chat::builder(model)
    .with_system_prompt("You are a professional academic AI assistant and can help to do RAG tasks.")
    .build();

  loop {
    chat.add_message(prompt_input("\n> ")?)
      .to_std_out()
      .await?;
  }
}
