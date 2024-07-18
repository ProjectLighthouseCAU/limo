use crate::{context::Context, path::VirtualPathBuf};
use anyhow::Result;
use clap::{command, Parser};
use futures::poll;
use futures_util::stream::StreamExt;
use lighthouse_client::protocol::{Frame, Model, Value, Verb};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Rectangle},
        Widget,
    },
    Terminal,
};
use std::{io::stdout, task::Poll, time::Duration};

// Timeout for keyboard event polling.
// This limits the execution speed of the loop
// and also the maximum possible framerate.
const POLL_TIMEOUT_MS: u64 = 10;

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

    let mut stream = ctx.lh.stream::<Value, Model>(&path.as_lh_vec(), Value::Nil).await?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    loop {
        if let Poll::Ready(Some(Ok(msg))) = poll!(stream.next()) {
            if let Model::Frame(lh_frame) = msg.payload {
                terminal.draw(|frame| {
                    let canvas = draw_to_canvas(
                        lh_frame,
                        frame.size().width.into(),
                        frame.size().height.into(),
                    );
                    frame.render_widget(canvas, frame.size());
                })?;
            }
        }

        // TODO: maybe we can put this in a future and await either the next frame
        // or a key event instead of polling both in a loop?
        if event::poll(Duration::from_millis(POLL_TIMEOUT_MS))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.intersects(KeyModifiers::CONTROL) => break,
                    _ => {}
                }
            }
        }
    }

    ctx.lh.perform::<Value, Value>(&Verb::Stop, &path.as_lh_vec(), Value::Nil).await?;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(String::new())
}

fn draw_to_canvas(frame: Frame, max_width: f64, max_height: f64) -> impl Widget {
    Canvas::default()
        // TODO decide whether to add a border and title
        // and figure out how to size it properly such that it fits the display
        // .block(Block::bordered().title(path.to_string()))
        .marker(Marker::Block)
        .paint(move |ctx| {
            for y in 0..14 {
                for x in 0..28 {
                    let c = frame.get(x, y);
                    ctx.draw(&Rectangle {
                        x: x as f64,
                        y: y as f64,
                        width: 1.0,
                        height: 1.0,
                        color: Color::from_u32(
                            c.blue as u32 | (c.green as u32) << 8 | (c.red as u32) << 16,
                        ),
                    })
                }
            }
        })
        .x_bounds([0.0, max_width])
        .y_bounds([0.0, max_height])
}
