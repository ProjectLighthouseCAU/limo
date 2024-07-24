use crate::{context::Context, path::VirtualPathBuf};
use anyhow::Result;
use clap::{command, Parser};
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::{select, StreamExt};
use lighthouse_client::protocol::{Frame, InputEvent, Model, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Painter, Rectangle, Shape},
        Block, BorderType, Padding, Widget,
    },
    Terminal,
};
use std::io::stdout;

const QUIT_KEY: char = 'q';

#[derive(Parser)]
#[command(bin_name = "display")]
struct Args {
    #[arg(
        default_value = ".",
        help = "The resource to display as an image stream"
    )]
    path: VirtualPathBuf,
}

pub async fn invoke(args: &[String], ctx: &mut Context) -> Result<String> {
    let args = Args::try_parse_from(args)?;
    let path = ctx.cwd.join(args.path);

    let mut stream = ctx.lh.stream(&path.as_lh_vec(), ()).await?.fuse();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut reader = EventStream::new().fuse();
    loop {
        select! {
            msg = reader.next() => match msg {
                Some(Ok(Event::Key(e))) => match e.code {
                    KeyCode::Char(QUIT_KEY) => break,
                    _ => if let Some(code) = key_code_to_js(e.code) {
                        ctx.lh.put(&path.as_lh_vec(), Model::InputEvent(InputEvent {
                            source: 0,
                            key: Some(code),
                            button: None,
                            is_down: matches!(e.kind, KeyEventKind::Press | KeyEventKind::Repeat),
                        })).await?;
                    }
                },
                None | Some(Err(_)) => break,
                _ => {},
            },
            msg = stream.next() => match msg {
                None | Some(Err(_)) => break,
                Some(Ok(msg)) => if let Model::Frame(lh_frame) = msg.payload {
                    terminal.draw(|frame| {
                        let canvas = display_canvas(
                            lh_frame,
                            format!("{} ({}: quit)", path, QUIT_KEY),
                        );
                        frame.render_widget(canvas, frame.size());
                    })?;
                }
            },
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    ctx.lh.stop(&path.as_lh_vec()).await?;

    Ok(String::new())
}

fn display_canvas(lh_frame: Frame, title: String) -> impl Widget {
    Canvas::default()
        .block(
            Block::bordered()
                .title(title)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .marker(Marker::Block)
        .paint(move |ctx| ctx.draw(&Display { lh_frame }))
}

struct Display {
    lh_frame: Frame
}

impl Shape for Display {
    fn draw(&self, painter: &mut Painter) {
        for y in 0..LIGHTHOUSE_ROWS {
            for x in 0..LIGHTHOUSE_COLS {
                let c = self.lh_frame.get(x, y);
                painter.paint(x, y, Color::from_u32(
                    ((c.red as u32) << 16) | (c.green as u32) << 8 | c.blue as u32,
                ));
            }
        }
    }
}

fn key_code_to_js(key_code: KeyCode) -> Option<i32> {
    match key_code {
        KeyCode::Backspace => Some(8),
        KeyCode::Enter => Some(13),
        KeyCode::Left => Some(37),
        KeyCode::Right => Some(39),
        KeyCode::Up => Some(38),
        KeyCode::Down => Some(40),
        KeyCode::Home => Some(36),
        KeyCode::End => Some(35),
        KeyCode::PageUp => Some(33),
        KeyCode::PageDown => Some(34),
        KeyCode::Tab => Some(9),
        KeyCode::Delete => Some(46),
        KeyCode::Insert => Some(45),
        KeyCode::F(n) => Some(111 + n as i32),
        KeyCode::Char(c) => Some(c as i32 + '0' as i32),
        KeyCode::Esc => Some(27),
        KeyCode::CapsLock => Some(20),
        KeyCode::ScrollLock => Some(145),
        KeyCode::NumLock => Some(144),
        KeyCode::PrintScreen => Some(44),
        KeyCode::Pause => Some(19),
        _ => None,
    }
}
