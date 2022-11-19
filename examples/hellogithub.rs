use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

struct App {
    url: String, // 存放一些数据或者 UI 状态
}
fn main() -> Result<(), io::Error> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App {
        url: String::from(r"https://hellogithub.com/"),
    };
    // 渲染界面
    run_app(&mut terminal, app)?;
    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // 处理按键事件
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => {
                        if 'q' == ch {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    //
    let chunks = Layout::default() // 首先获取默认构造
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
        .direction(Direction::Vertical) // 垂直分割
        .split(f.size()); // 分割整块 Terminal 区域
    let paragraph = Paragraph::new(Span::styled(
        app.url.as_str(),
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("HelloGitHub"))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new("分享 GitHub 上有趣、入门级的开源项目")
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(Block::default().borders(Borders::ALL).title("宗旨"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[1]);
}
