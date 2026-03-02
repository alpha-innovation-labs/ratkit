use opencode_sdk::runtime::ManagedRuntime;
use opencode_sdk::types::message::{Message, Part, ToolState};
use opencode_sdk::types::session::Session;

fn format_duration(ms: i64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        let seconds = ms / 1000;
        let remaining_ms = ms % 1000;
        if seconds < 60 {
            format!("{}.{:03}s", seconds, remaining_ms)
        } else {
            let minutes = seconds / 60;
            let remaining_seconds = seconds % 60;
            format!("{}m {}s", minutes, remaining_seconds)
        }
    }
}

fn format_output(session: &Session, messages: &[Message]) {
    let total_duration_ms = session
        .time
        .as_ref()
        .map(|t| t.updated - t.created)
        .unwrap_or(0);

    println!("\n============================================================");
    println!("Session: {}", session.title);
    println!("ID: {}", session.id);
    println!(
        "Directory: {}",
        session.directory.as_deref().unwrap_or("Unknown")
    );
    println!("Version: {}", session.version);
    if let Some(time) = &session.time {
        println!("Created: {}", time.created);
        println!("Updated: {}", time.updated);
    }
    println!("============================================================");

    println!("\nMessages:\n");

    for msg in messages {
        let role = msg.info.role.to_uppercase();
        print!("[{}] {}", role, msg.info.time.created);

        if let Some(completed) = msg.info.time.completed {
            let duration_ms = completed - msg.info.time.created;
            print!(" ({})", format_duration(duration_ms));
        }
        println!();

        for part in &msg.parts {
            match part {
                Part::Text { text, .. } => {
                    println!("{}", text);
                }
                Part::Tool {
                    tool, state, input, ..
                } => {
                    println!("[Tool: {}]", tool);
                    if let Some(input_val) = input.get("input").and_then(|v| v.as_str()) {
                        println!("{}", input_val);
                    }
                    if let Some(tool_state) = state {
                        match tool_state {
                            ToolState::Completed(completed) => {
                                println!("Output: {}", completed.output);
                            }
                            ToolState::Error(err) => {
                                println!("Error: {}", err.error);
                            }
                            _ => {}
                        }
                    }
                }
                Part::StepFinish {
                    cost,
                    tokens,
                    reason,
                    ..
                } => {
                    print!("[Step: {} | cost: ${:.4}]", reason, cost);
                    if let Some(tok) = tokens {
                        print!(" [Tokens: in={} out={}", tok.input, tok.output);
                        if tok.reasoning > 0 {
                            print!(" reasoning={}", tok.reasoning);
                        }
                        print!("]");
                    }
                    println!();
                }
                _ => {}
            }
        }
    }

    println!("\nTotal Duration: {}ms", total_duration_ms);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting OpenCode Managed Runtime...");

    let runtime = ManagedRuntime::start_for_cwd().await?;
    println!(
        "Managed runtime ready at: http://localhost:{}",
        runtime.server().port()
    );

    let client = runtime.client();

    println!("Listing sessions...");
    let sessions = client.sessions().list().await?;

    if sessions.is_empty() {
        println!("No sessions found.");
        return Ok(());
    }

    println!("Found {} session(s):", sessions.len());

    for session in &sessions {
        println!(
            "  - {} (ID: {}, Updated: {})",
            session.title,
            session.id,
            session
                .time
                .as_ref()
                .map(|t| t.updated.to_string())
                .unwrap_or_default()
        );
    }

    let latest_session = sessions
        .iter()
        .max_by_key(|s| s.time.as_ref().map(|t| t.updated).unwrap_or(0))
        .expect("At least one session exists");

    println!(
        "\nFetching conversation for: {} ({})",
        latest_session.title, latest_session.id
    );

    let session = client.sessions().get(&latest_session.id).await?;
    let messages = client.messages().list(&latest_session.id).await?;

    println!("Found {} message(s).\n", messages.len());

    format_output(&session, &messages);

    println!("\n[Demo would continue here with TUI - TUI creation commented out]");

    Ok(())
}
