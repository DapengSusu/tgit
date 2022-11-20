use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, path::Path};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Layout, Constraint, Direction},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use tracing::info;

use tgit::{Args, CmdPwd, ShellCommand};
// use tgit::GitStatus;

fn main() -> anyhow::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // set work dir
    let cmd_pwd = CmdPwd::new();
    let curr_dir = CmdPwd::exec_for_output(cmd_pwd.cmd(), &[""])?;
    let args = Args::parse();
    let work_dir = Path::new(&curr_dir.trim())
        .join(args.path_parser()?).as_path().canonicalize()?;

    info!("curr_dir:{}", work_dir.to_string_lossy());

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main block with round corners")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let font_h = 3_u16;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(font_h),
                Constraint::Length(f.size().height - font_h)
            ]
            .as_ref())
        .split(f.size());

    let work_dir = Block::default()
        .borders(Borders::ALL)
        .title("work dir")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    f.render_widget(work_dir, chunks[0]);

    // let paragraph = Paragraph::new(Span::styled(
    //     app.url.as_str(),
    //     Style::default().add_modifier(Modifier::BOLD),
    // ))
    // .block(Block::default().borders(Borders::ALL).title("HelloGitHub"))
    // .alignment(tui::layout::Alignment::Left);
    // f.render_widget(paragraph, chunks[0]);
}
