use std::io::Result;

use crossterm::event;
use ratatui::style::Color;
use rspew::renderer;
use rspew::widgets::cave::{Cave, CaveConfig};
use rspew::widgets::noise_background::NoiseBackground;

fn main() -> Result<()> {
    // init
    let renderer = renderer::Renderer::new();
    let mut terminal = renderer.start().unwrap();
    let seed: f64 = 102.0;

    let mut background = NoiseBackground::new(seed);

    let mut cave = Cave::new(
        100.0,
        CaveConfig {
            opening_ratio: 1.0,
            opening_min: 0.1,
            frequency: 75.0,
            smooth: 750.0,
            color: Color::Gray,
        },
    );
    let mut cave2 = Cave::new(
        200.0,
        CaveConfig {
            opening_ratio: 1.0,
            opening_min: 0.8,
            frequency: 100.0,
            smooth: 20.0,
            color: Color::DarkGray,
        },
    );

    // game loop
    loop {
        // draw everything
        terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(background, area);
            frame.render_widget(cave, area);
            frame.render_widget(cave2, area);
        })?;

        // update animations
        background.scroll();
        cave.scroll();
        cave2.scroll();

        // capture events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') {
                    break;
                }
                if key.code == event::KeyCode::Left && background.speed_x > -10 {
                    background.set_speed_x(background.speed_x - 1);
                    cave.set_speed_x(cave.speed_x - 2);
                    cave2.set_speed_x(cave2.speed_x - 3);
                }
                if key.code == event::KeyCode::Right && background.speed_x < 10 {
                    background.set_speed_x(background.speed_x + 1);
                    cave.set_speed_x(cave.speed_x + 2);
                    cave2.set_speed_x(cave2.speed_x + 3);
                }
            }
        }
    }

    // post loop cleanup
    renderer.cleanup()
}
