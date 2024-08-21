use crossterm::event;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Terminal;
use rspew::renderer;
use rspew::widgets::cave::{Cave, CaveConfig, CaveWidget};
use rspew::widgets::noise_background::NoiseBackground;
use rspew::widgets::spaceship::{self, SpaceshipWidget};
use std::io::{Result, Stdout};

fn main() -> Result<()> {
    // init
    let renderer = renderer::Renderer::new();
    let mut terminal = renderer.start().unwrap();
    let seed: f64 = 102.0;

    let mut background = NoiseBackground::new(seed);

    let mut spaceship = spaceship::SpaceshipModel::new(10, 10);

    let mut cave = Cave::new(
        100.0,
        CaveConfig {
            opening_max: 0.8,
            opening_min: 0.7,
            frequency: 100.0,
            smooth: 50.0,
            color: Color::Gray,
        },
    );
    let mut cave_foreground = Cave::new(
        200.0,
        CaveConfig {
            opening_max: 1.0,
            opening_min: 0.9,
            frequency: 10.0,
            smooth: 20.0,
            color: Color::DarkGray,
        },
    );

    // game loop
    gameloop(
        &mut terminal,
        &mut spaceship,
        &mut background,
        &mut cave,
        &mut cave_foreground,
    )?;

    // post loop cleanup
    renderer.cleanup()
}

fn gameloop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    spaceship: &mut spaceship::SpaceshipModel,
    background: &mut NoiseBackground,
    cave: &mut Cave,
    cave_foreground: &mut Cave,
) -> Result<()> {
    loop {
        // draw everything
        terminal.draw(|frame| {
            let area = frame.size();

            // update animations
            background.scroll();
            frame.render_widget(*background, area);

            cave.scroll(area);
            frame.render_widget(CaveWidget::new(cave.color, &cave.pixels), area);

            cave_foreground.scroll(area);
            frame.render_widget(
                CaveWidget::new(cave_foreground.color, &cave_foreground.pixels),
                area,
            );

            let color = if cave.has_pixel(spaceship.position.x, spaceship.position.y) {
                Color::Red
            } else {
                Color::White
            };
            frame.render_widget(SpaceshipWidget::new(&spaceship, color), area);

            frame.render_widget(
                Block::default()
                    .title("··· Scroll Speed: ← → ··· Quit: Esc / q ···")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black)),
                area,
            )
        })?;

        // capture events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc {
                    break;
                }
                if key.code == event::KeyCode::Char(' ') {
                    background.set_speed_x(0);
                    cave.set_speed_x(0);
                    cave_foreground.set_speed_x(0);
                }
                if key.code == event::KeyCode::Left && background.speed_x > -10 {
                    background.set_speed_x(background.speed_x - 1);
                    cave.set_speed_x(cave.speed_x - 2);
                    cave_foreground.set_speed_x(cave_foreground.speed_x - 3);
                }
                if key.code == event::KeyCode::Right && background.speed_x < 10 {
                    background.set_speed_x(background.speed_x + 1);
                    cave.set_speed_x(cave.speed_x + 2);
                    cave_foreground.set_speed_x(cave_foreground.speed_x + 3);
                }

                if key.code == event::KeyCode::Up {
                    spaceship.position.y -= 1;
                }
                if key.code == event::KeyCode::Down {
                    spaceship.position.y += 1;
                }
            }
        }
    }

    return Result::Ok(());
}
