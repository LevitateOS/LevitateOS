use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use std::io::{self, stdout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

mod llm;

// Which panel has focus
#[derive(Clone, Copy, PartialEq)]
enum Focus {
    Checklist,
    Chat,
    Input,
}

// Installation stages
#[derive(Clone)]
struct Stage {
    name: &'static str,
    hints: &'static [&'static str],
    done: bool,
    expanded: bool,
}

impl Stage {
    fn new(name: &'static str, hints: &'static [&'static str]) -> Self {
        Self { name, hints, done: false, expanded: true }
    }
}

// Chat message
#[derive(Clone)]
struct Message {
    role: Role,
    text: String,
}

#[derive(Clone, PartialEq)]
enum Role {
    User,
    Assistant,
}

// App state
struct App {
    focus: Focus,
    stages: Vec<Stage>,
    stage_cursor: usize,
    messages: Vec<Message>,
    chat_scroll: usize,
    input: String,
    should_quit: bool,
    last_ctrl_c: Option<Instant>,
    status_message: Option<String>,
    llm: Arc<llm::LlmServer>,
    llm_rx: Receiver<String>,
    llm_tx: Sender<String>,
    waiting_for_llm: bool,
}

impl App {
    fn new(llm: llm::LlmServer) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            focus: Focus::Input,
            stages: vec![
                Stage::new("Disk Configuration", &[
                    "\"list disks\"",
                    "\"use the whole 500gb drive\"",
                    "\"encrypted root partition\"",
                    "\"dual boot with windows\"",
                ]),
                Stage { expanded: false, ..Stage::new("System Installation", &[
                    "\"install the system\"",
                    "\"copy to disk\"",
                ])},
                Stage { expanded: false, ..Stage::new("System Configuration", &[
                    "\"hostname is my-laptop\"",
                    "\"timezone los angeles\"",
                    "\"keyboard us\"",
                    "\"language english\"",
                ])},
                Stage { expanded: false, ..Stage::new("User Setup", &[
                    "\"create user vince with sudo\"",
                    "\"set password\"",
                    "\"add user to wheel group\"",
                ])},
                Stage { expanded: false, ..Stage::new("Bootloader", &[
                    "\"install bootloader\"",
                    "(automatic after disk config)",
                ])},
                Stage { expanded: false, ..Stage::new("Finalize", &[
                    "\"done\"",
                    "\"reboot now\"",
                    "\"exit without reboot\"",
                ])},
            ],
            stage_cursor: 0,
            messages: vec![
                Message {
                    role: Role::Assistant,
                    text: "# Welcome to LevitateOS Installer!\n\n**LLM:** `FunctionGemma 2B`\n**LoRA:** `levitate-installer`\n\nType what you want to do in natural language.\nTry: `list disks` to see available drives.".into(),
                },
            ],
            chat_scroll: 0,
            input: String::new(),
            should_quit: false,
            last_ctrl_c: None,
            status_message: None,
            llm: Arc::new(llm),
            llm_rx: rx,
            llm_tx: tx,
            waiting_for_llm: false,
        }
    }

    fn submit_input(&mut self) {
        if self.input.trim().is_empty() || self.waiting_for_llm {
            return;
        }

        let user_text = std::mem::take(&mut self.input);

        // Add user message immediately
        self.messages.push(Message {
            role: Role::User,
            text: user_text.clone(),
        });

        // Add thinking placeholder
        self.messages.push(Message {
            role: Role::Assistant,
            text: "_Thinking..._".to_string(),
        });

        self.waiting_for_llm = true;
        self.chat_scroll = usize::MAX;

        // Query LLM in background thread
        let llm = Arc::clone(&self.llm);
        let tx = self.llm_tx.clone();
        thread::spawn(move || {
            let response = match llm.query(&user_text) {
                Ok(resp) => {
                    if resp.success {
                        resp.response.unwrap_or_else(|| "No response".to_string())
                    } else {
                        resp.error.unwrap_or_else(|| "Unknown error".to_string())
                    }
                }
                Err(e) => format!("Error: {}", e),
            };
            let _ = tx.send(response);
        });
    }

    fn check_llm_response(&mut self) {
        if let Ok(response) = self.llm_rx.try_recv() {
            // Replace the "Thinking..." message with actual response
            if let Some(msg) = self.messages.last_mut() {
                if msg.role == Role::Assistant && msg.text == "_Thinking..._" {
                    msg.text = response;
                }
            }
            self.waiting_for_llm = false;
            self.chat_scroll = usize::MAX;
        }
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Clear status message on any key
        self.status_message = None;

        // Handle Ctrl+C
        if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
            if let Some(last) = self.last_ctrl_c {
                if last.elapsed().as_secs() < 2 {
                    self.should_quit = true;
                    return;
                }
            }
            self.last_ctrl_c = Some(Instant::now());
            self.status_message = Some("Press Ctrl+C again to quit".into());
            return;
        }

        match code {
            KeyCode::Esc => self.should_quit = true,

            // Panel navigation
            KeyCode::Left => {
                self.focus = match self.focus {
                    Focus::Input => Focus::Checklist,
                    Focus::Chat => Focus::Checklist,
                    Focus::Checklist => Focus::Checklist,
                };
            }
            KeyCode::Right => {
                self.focus = match self.focus {
                    Focus::Checklist => Focus::Input,
                    Focus::Chat => Focus::Input,
                    Focus::Input => Focus::Input,
                };
            }
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Checklist => Focus::Chat,
                    Focus::Chat => Focus::Input,
                    Focus::Input => Focus::Checklist,
                };
            }

            // Panel-specific actions
            KeyCode::Up => match self.focus {
                Focus::Checklist => {
                    self.stage_cursor = self.stage_cursor.saturating_sub(1);
                }
                Focus::Chat => {
                    self.chat_scroll = self.chat_scroll.saturating_sub(1);
                }
                Focus::Input => {
                    self.focus = Focus::Chat;
                }
            },
            KeyCode::Down => match self.focus {
                Focus::Checklist => {
                    if self.stage_cursor < self.stages.len() - 1 {
                        self.stage_cursor += 1;
                    }
                }
                Focus::Chat => {
                    self.chat_scroll = self.chat_scroll.saturating_add(1);
                }
                Focus::Input => {}
            },

            KeyCode::Enter => match self.focus {
                Focus::Checklist => {
                    // Toggle expanded
                    self.stages[self.stage_cursor].expanded =
                        !self.stages[self.stage_cursor].expanded;
                }
                Focus::Chat => {
                    self.focus = Focus::Input;
                }
                Focus::Input => {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        self.input.push('\n');
                    } else {
                        self.submit_input();
                    }
                }
            },

            KeyCode::Backspace if self.focus == Focus::Input => {
                self.input.pop();
            }
            KeyCode::Char(c) if self.focus == Focus::Input => {
                self.input.push(c);
            }

            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Show loading screen
    terminal.draw(|frame| {
        let area = frame.area();
        let text = Paragraph::new("Loading LLM server...\n\nThis may take a moment.")
            .block(Block::default().title(" LevitateOS Installer ").borders(Borders::ALL))
            .alignment(Alignment::Center);
        frame.render_widget(text, area);
    })?;

    // Start LLM server (blocking, but TUI is visible)
    let llm = llm::LlmServer::start("vendor/models/FunctionGemma")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut app = App::new(llm);
    while !app.should_quit {
        app.check_llm_response();
        terminal.draw(|frame| ui(frame, &app))?;
        handle_events(&mut app)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::horizontal([
        Constraint::Percentage(35),
        Constraint::Percentage(65),
    ])
    .split(area);

    render_stages(frame, layout[0], app);
    render_chat(frame, layout[1], app);
}

fn render_stages(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Checklist;
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let mut items: Vec<ListItem> = Vec::new();

    for (i, stage) in app.stages.iter().enumerate() {
        let is_selected = i == app.stage_cursor && focused;

        // Checkbox + name + expand indicator
        let checkbox = if stage.done { "[x]" } else { "[ ]" };
        let expand_icon = if stage.expanded { "▼" } else { "▶" };

        let style = if is_selected {
            Style::default().fg(Color::Yellow).bold()
        } else if stage.done {
            Style::default().fg(Color::Green)
        } else {
            Style::default().bold()
        };

        let header = Line::from(vec![
            Span::styled(format!("{} {} ", checkbox, expand_icon), style),
            Span::styled(stage.name, style),
        ]);
        items.push(ListItem::new(header));

        // Hints (only if expanded)
        if stage.expanded {
            for hint in stage.hints {
                let hint_style = if is_selected {
                    Style::default().fg(Color::Yellow).dim()
                } else {
                    Style::default().dim()
                };
                let hint_line = Line::from(vec![
                    Span::raw("      "),
                    Span::styled(*hint, hint_style),
                ]);
                items.push(ListItem::new(hint_line));
            }
        }

        items.push(ListItem::new(""));
    }

    let title = if focused {
        " Installation Steps (↑↓ move, Enter toggle) "
    } else {
        " Installation Steps "
    };

    let list = List::new(items)
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style));

    frame.render_widget(list, area);
}

fn render_chat(frame: &mut Frame, area: Rect, app: &App) {
    // Input height: 3 lines min, grows with content up to 6
    let input_lines = app.input.lines().count().max(1);
    let input_height = (input_lines + 2).min(6) as u16; // +2 for borders

    // Status bar if there's a message
    let status_height = if app.status_message.is_some() { 1 } else { 0 };

    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(input_height),
        Constraint::Length(status_height),
    ])
    .split(area);

    render_messages(frame, layout[0], app);
    render_input(frame, layout[1], app);

    // Render status message
    if let Some(msg) = &app.status_message {
        let status = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Yellow).bold());
        frame.render_widget(status, layout[2]);
    }
}

fn render_messages(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Chat;
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let inner_area = Block::default()
        .borders(Borders::ALL)
        .inner(area);

    // Build message lines
    let mut lines: Vec<Line> = Vec::new();
    for msg in &app.messages {
        match msg.role {
            Role::User => {
                // User messages: cyan with > prefix
                for line in msg.text.lines() {
                    lines.push(Line::styled(
                        format!("> {}", line),
                        Style::default().fg(Color::Cyan),
                    ));
                }
            }
            Role::Assistant => {
                // Assistant messages: parse as markdown
                let parsed = tui_markdown::from_str(&msg.text);
                for line in parsed.lines {
                    lines.push(line.clone());
                }
            }
        }
        lines.push(Line::raw(""));
    }

    // Calculate scroll
    let visible_height = inner_area.height as usize;
    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll = app.chat_scroll.min(max_scroll);

    let title = if focused {
        " Chat (↑↓ scroll, Enter to input) "
    } else {
        " Chat "
    };

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0))
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style));

    frame.render_widget(paragraph, area);
}

fn render_input(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Input;
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title = if focused { " > Type here (Shift+Enter for newline) " } else { " > " };

    let input_widget = Paragraph::new(app.input.as_str())
        .block(Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style));

    frame.render_widget(input_widget, area);

    // Show cursor only when focused
    if focused {
        // Calculate cursor position for multiline
        let lines: Vec<&str> = app.input.lines().collect();
        let (cursor_x, cursor_y) = if lines.is_empty() {
            (0, 0)
        } else {
            let last_line = lines.last().unwrap_or(&"");
            (last_line.len(), lines.len().saturating_sub(1))
        };

        frame.set_cursor_position(Position::new(
            area.x + cursor_x as u16 + 2,
            area.y + cursor_y as u16 + 1,
        ));
    }
}

fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key.code, key.modifiers);
            }
        }
    }
    Ok(())
}
