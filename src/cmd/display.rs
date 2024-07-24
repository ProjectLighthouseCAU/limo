use crate::{context::Context, path::VirtualPathBuf};
use anyhow::Result;
use clap::{command, Parser};
use crossterm::event::{Event, EventStream};
use futures::{select, StreamExt};
use lighthouse_client::protocol::{Frame, Model};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, BorderType, Padding, Widget,
    },
    Terminal,
};
use std::io::stdout;

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
                None | Some(Err(_)) | Some(Ok(Event::Key(_)))=> break,
                _ => {},
            },
            msg = stream.next() => match msg {
                None | Some(Err(_)) => break,
                Some(Ok(msg)) => if let Model::Frame(lh_frame) = msg.payload {
                    terminal.draw(|frame| {
                        let canvas = draw_to_canvas(
                            lh_frame,
                            frame.size().width.into(),
                            frame.size().height.into(),
                            path.to_string()
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

fn draw_to_canvas(frame: Frame, max_width: f64, max_height: f64, title: String) -> impl Widget {
    Canvas::default()
        .block(
            Block::bordered()
                .title(title)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
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
