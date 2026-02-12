export const googleGenerateText = `
use aisdk::{
    core::LanguageModelRequest,
    providers::Google,
};

#[tokio::main]
async fn main() {

    let result = LanguageModelRequest::builder()
        .model(Google::gemini_3_flash_preview())
        .prompt("What is the meaning of life?")
        .build()
        .generate_text() // stream_text() for streaming
        .await
        .unwrap();

    println!("{:?}", result.text());
}
`;

export const openaiGenerateText = `
use aisdk::{
    core::LanguageModelRequest,
    providers::OpenAI,
};

#[tokio::main]
async fn main() {

    let result = LanguageModelRequest::builder()
        .model(OpenAI::gpt_5())
        .prompt("What is the meaning of life?")
        .build()
        .generate_text() // stream_text() for streaming
        .await
        .unwrap();

    println!("{:?}", result.text());
}
`;

export const anthropicGenerateText = `
use aisdk::{
    core::LanguageModelRequest,
    providers::Anthropic,
};

#[tokio::main]
async fn main() {

    let result = LanguageModelRequest::builder()
        .model(Anthropic::claude_opus_4_5())
        .prompt("What is the meaning of life?")
        .build()
        .generate_text() // stream_text() for streaming
        .await
        .unwrap();

    println!("{:?}", result.text());
}
`;

export const openaiStreamText = `
use aisdk::{
    core::{LanguageModelRequest, LanguageModelStreamChunkType},
    providers::OpenAI,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut stream = LanguageModelRequest::builder()
        .model(OpenAI::gpt_5())
        .prompt("What is the meaning of life?")
        .build()
        .stream_text()
        .await?;

	while let Some(chunk) = stream.next().await {
		if let LanguageModelStreamChunkType::Text(text) = chunk {
			println!("Streaming text: {}", text);
		}
	}

    Ok(())
}
`;

export const anthropicStreamText = `
use aisdk::{
    core::{LanguageModelRequest, LanguageModelStreamChunkType},
    providers::Anthropic,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut stream = LanguageModelRequest::builder()
        .model(Anthropic::claude_opus_4_5())
        .prompt("What is the meaning of life?")
        .build()
        .stream_text()
        .await?;

	while let Some(chunk) = stream.next().await {
		if let LanguageModelStreamChunkType::Text(text) = chunk {
			println!("Streaming text: {}", text);
		}
	}

    Ok(())
}
`;

export const googleStreamText = `
use aisdk::{
    core::{LanguageModelRequest, LanguageModelStreamChunkType},
    providers::Google,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut stream = LanguageModelRequest::builder()
		.model(Google::gemini_3_flash_preview())
		.prompt("What is the meaning of life?")
		.build()
		.stream_text()
		.await?;

	while let Some(chunk) = stream.next().await {
		if let LanguageModelStreamChunkType::Text(text) = chunk {
			println!("Streaming text: {}", text);
		}
	}

    Ok(())
}
`;
