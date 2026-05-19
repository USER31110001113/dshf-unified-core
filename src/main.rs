mod engine;
use engine::DSHFUnifiedEngine;
use std::fs::File;
use std::io::{Read, BufReader};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind}, 
    execute, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DSHFUnifiedEngine::new(4096);
    let mut system_log;

    if std::path::Path::new("brain.bin").exists() {
        engine.load_memory("brain.bin")?;
        system_log = String::from("Loaded active multi-channel intelligence wave fields from brain.bin.");
    } else {
        let core_syntax = "the structural framework details an organic process accurately. a system functions cleanly.";
        let core_semantic = "sn2_mechanisms require simultaneous backside_attack dynamics causing an inversion_of_configuration at the spatial carbon center. aprotic_solvents accelerate rates while steric_hindrance retards transition geometries.";
        let core_instruct = "analyze systemic failure bounds step by step using explicit first principles metrics without empty jargon.";
        
        engine.ingest_to_channel(core_syntax, "syntax");
        engine.ingest_to_channel(core_semantic, "semantic");
        engine.ingest_to_channel(core_instruct, "instruct");

        if std::path::Path::new("corpus.txt").exists() {
            let file = File::open("corpus.txt")?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            engine.ingest_to_channel(&contents, "semantic");
            system_log = format!("Ingested custom data stream containing {} patterns into SEMANTIC tensor.", contents.split_whitespace().count());
        } else {
            system_log = String::from("Cold initialization successful. Tri-channel matrices seeded via core assets.");
        }
    }

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut input_text = String::new();
    let mut ai_response = String::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)].as_ref())
                .split(f.size());

            let input_block = Paragraph::new(format!("> {}", input_text))
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title(" Holographic Tri-Channel Command Input "));
            f.render_widget(input_block, chunks[0]);

            let output_block = Paragraph::new(ai_response.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title(" Autonomous Synthesized Tutorial Output "));
            f.render_widget(output_block, chunks[1]);

            let log_block = Paragraph::new(system_log.as_str())
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL).title(" Core Engine Status "));
            f.render_widget(log_block, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => {
                        input_text.push(c);
                    }
                    KeyCode::Backspace => {
                        input_text.pop();
                    }
                    KeyCode::Enter => {
                        let cmd = input_text.trim().to_string();
                        input_text.clear();

                        if cmd == "exit" { break; }
                        if cmd.is_empty() { continue; }
                        
                        if cmd.starts_with("ingest ") {
                            let parts: Vec<&str> = cmd.splitn(3, ' ').collect();
                            if parts.len() == 3 {
                                let channel = parts[1];
                                let content = parts[2];
                                engine.ingest_to_channel(content, channel);
                                system_log = format!("Injected data into channel [{}]: '{}'", channel.to_uppercase(), content);
                            } else {
                                system_log = String::from("Format error. Usage: ingest <syntax/semantic/instruct> <content>");
                            }
                        } else if cmd.starts_with("train ") {
                            let parts: Vec<&str> = cmd.split_whitespace().collect();
                            if parts.len() == 4 {
                                system_log = engine.reinforce_channel_token(parts[1], parts[2], parts[3] == "true");
                            } else {
                                system_log = String::from("Format error. Usage: train <channel> <token> <true/false>");
                            }
                        } else if cmd.starts_with("prompt ") {
                            let target = cmd.replacen("prompt ", "", 1);
                            // Directly invoke the autonomous generator core
                            ai_response = engine.generate_native_answer(&target);
                            system_log = format!("Engine executing autonomous generation loop for asset: '{}'", target);
                        } else {
                            ai_response = String::from("Unknown command context. Use 'prompt <concept>' to run autonomous synthesis.");
                            system_log = format!("Ignored direct injection vector: '{}'", cmd);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    engine.save_memory("brain.bin")?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}