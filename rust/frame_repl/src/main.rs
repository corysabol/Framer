use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::stream::StreamExt;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // BLE SETUP ========
    // Prompt the FrameServer to connect to the Frame device

    // Set up TUI
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up channels for communication
    let (tx, mut rx) = mpsc::channel(100);
    let (input_tx, mut input_rx) = mpsc::channel(100);

    // Spawn a task to handle BLE notifications

    // Spawn a thread to handle input events
    std::thread::spawn(move || loop {
        if let Ok(event) = event::read() {
            if let Err(_) = input_tx.blocking_send(event) {
                break;
            }
        }
    });

    let mut input = String::new();
    let mut output = Vec::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let output_items: Vec<ListItem> = output
                .iter()
                .map(|s: &String| ListItem::new(Spans::from(s.clone())))
                .collect();

            let output_widget = List::new(output_items)
                .block(Block::default().title("Output").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White));

            f.render_widget(output_widget, chunks[0]);

            let input_widget = Paragraph::new(input.as_ref())
                .block(Block::default().title("Input").borders(Borders::ALL));
            f.render_widget(input_widget, chunks[1]);
        })?;

        tokio::select! {
            Some(event) = input_rx.recv() => {
                if let Event::Key(key) = event {
                    match key.code {
                        KeyCode::Enter => {
                            if !input.is_empty() {
                                device.write(&send, input.as_bytes(), btleplug::api::WriteType::WithResponse).await?;
                                output.push(format!("> {}", input));
                                input.clear();
                            }
                        }
                        KeyCode::Char(c) => {
                            if key.kind == KeyEventKind::Press { input.push(c); }
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Some(response) = rx.recv() => {
                println!("{:?}", response);
                output.push(response);
                if output.len() > 1000 {
                    output.remove(0);
                }
            }
            else => break,
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
