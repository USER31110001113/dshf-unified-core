use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_ui(frame: &mut Frame, results: &[(String, f64)]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame.size());

    let text = results.iter()
        .map(|(t, a)| format!("{:<20} | Strength: {:.4}", t, a))
        .collect::<Vec<String>>()
        .join("\n");

    let block = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Holographic Resonance Map "));
    
    frame.render_widget(block, chunks[0]);
}