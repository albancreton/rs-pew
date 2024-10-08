use crossterm::event;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Terminal;
use rspew::renderer;
use rspew::widgets::cave::{CaveConfig, CaveModel, CaveWidget};
use rspew::widgets::noise_background::NoiseBackground;
use rspew::widgets::spaceship::{self, SpaceshipWidget};
use std::env;
use std::io::{Result, Stdout};

fn detect_true_color() -> bool {
    env::var("COLORTERM")
        .map(|val| val == "truecolor" || val == "24bit")
        .unwrap_or(false)
}

fn main() -> Result<()> {
    if !detect_true_color() {
        println!("You need a terminal that can render true colors to run this app.");
        return Ok(());
    }

    // init
    let renderer = renderer::Renderer::new();
    let mut terminal = renderer.start().unwrap();
    let seed: f64 = 102.0;

    let mut background = NoiseBackground::new(seed);

    let mut cave = CaveModel::new(
        100.0,
        CaveConfig {
            opening_max: 0.6,
            opening_min: 0.2,
            frequency: 30.0,
            smooth: 200.0,
            color: Color::Rgb(220, 240, 244),
        },
    );
    let mut cave_foreground = CaveModel::new(
        200.0,
        CaveConfig {
            opening_max: 1.0,
            opening_min: 0.5,
            frequency: 10.0,
            smooth: 10.0,
            color: Color::Rgb(180, 180, 190),
        },
    );

    // let's get the area of the terminal and generate the cave
    let area = terminal.get_frame().size();
    cave.scroll(area);

    // place the spaceship in the middle of the cave
    let mut spaceship =
        spaceship::SpaceshipModel::new(10, cave.openings[10].1 - cave.openings[10].0);

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
    cave: &mut CaveModel,
    cave_foreground: &mut CaveModel,
) -> Result<()> {
    let energy: &mut i32 = &mut 100;
    let is_coliding_cave: &mut bool = &mut false;

    loop {
        // draw everything
        terminal.draw(|frame| {
            let area = frame.size();

            background.scroll();
            frame.render_widget(*background, area);

            cave.scroll(area);
            frame.render_widget(CaveWidget::new(cave.color, &cave.openings), area);

            cave_foreground.scroll(area);
            frame.render_widget(
                CaveWidget::new(cave_foreground.color, &cave_foreground.openings),
                area,
            );

            let mut color = Color::White;
            *is_coliding_cave = spaceship.check_collision(cave);
            if *is_coliding_cave == true {
                *energy = *energy - 1 as i32;
                color = Color::Red;
            }
            frame.render_widget(SpaceshipWidget::new(&spaceship, color), area);

            if *is_coliding_cave == true {
                let opening = cave.openings[spaceship.position.x as usize];
                if spaceship.position.y < opening.0 {
                    spaceship.position.y += 1;
                } else {
                    spaceship.position.y -= 1;
                }
            }

            let divider = area.right() as f64 - 11.0;
            let mut s = String::from("");
            let r: usize = (*energy as f64 / (100.0 / divider)).floor() as usize;
            let d = divider as usize - r;

            for _ in 0..r {
                s.push_str("▣");
            }
            for _ in 0..d {
                s.push_str("□");
            }

            frame.render_widget(
                Block::default()
                    .title(format!("   {}   ", s))
                    .title_bottom("··· Scroll Speed: ← → ··· Quit: Esc / q ···")
                    .title_style(Style::default().fg(color).bold())
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black)),
                area,
            )
        })?;

        // capture events
        if event::poll(std::time::Duration::from_millis(8))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc {
                    break;
                }
                if key.code == event::KeyCode::Char(' ') {
                    background.set_speed_x(0.0);
                    cave.set_speed_x(0);
                    cave_foreground.set_speed_x(0);
                }
                if key.code == event::KeyCode::Left && background.speed_x > -10.0 {
                    background.set_speed_x(background.speed_x - 0.5);
                    cave.set_speed_x(cave.speed_x - 1);
                    cave_foreground.set_speed_x(cave_foreground.speed_x - 2);
                }
                if key.code == event::KeyCode::Right && background.speed_x < 10.0 {
                    background.set_speed_x(background.speed_x + 0.5);
                    cave.set_speed_x(cave.speed_x + 1);
                    cave_foreground.set_speed_x(cave_foreground.speed_x + 2);
                }

                if key.code == event::KeyCode::Up && *is_coliding_cave == false {
                    spaceship.position.y -= 1;
                }
                if key.code == event::KeyCode::Down && *is_coliding_cave == false {
                    spaceship.position.y += 1;
                }
            }
        }
    }

    return Result::Ok(());
}
