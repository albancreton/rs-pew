use std::io::Result;

use crossterm::event;
use rspew::background::Background;
use rspew::renderer;

fn main() -> Result<()> {
    // init
    let renderer = renderer::Renderer::new();
    let mut terminal = renderer.start().unwrap();
    let seed: f64 = 100.0;
    let mut background = Background::new(seed);

    // game loop
    loop {
        // draw everything
        terminal.draw(|frame| {
            let area = frame.size();
            let canvas = background.render(area);

            frame.render_widget(canvas, area);
        })?;

        // update animations
        background.scroll();

        // capture events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') {
                    break;
                }
                if key.code == event::KeyCode::Left && background.speed_x > -10 {
                    background.set_speed_x(background.speed_x - 1);
                }
                if key.code == event::KeyCode::Right && background.speed_x < 10 {
                    background.set_speed_x(background.speed_x + 1);
                }
            }
        }
    }

    // post loop cleanup
    renderer.cleanup()
}
