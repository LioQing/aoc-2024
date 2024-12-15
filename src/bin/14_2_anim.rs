use aoc_2024::input;
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use glam::*;
use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Block, Padding, Paragraph, Wrap,
    },
};

#[path = "14.rs"]
#[allow(dead_code)]
mod day;

struct State {
    robots: Vec<day::Robot>,
    duration: std::time::Duration,
    time: std::time::Instant,
    elapsed: std::time::Duration,
}

impl State {
    fn new(robots: Vec<day::Robot>, steps: i32) -> Self {
        Self {
            robots,
            duration: std::time::Duration::from_secs_f32(steps as f32),
            time: std::time::Instant::now(),
            elapsed: std::time::Duration::default(),
        }
    }
}

fn main() {
    env_logger::init();

    log::info!("Running day 14 solution");

    let (input, size) = (input(14), ivec2(101, 103));
    let robots = day::parse_input(&input);
    let steps = day::solution_part_2(&robots, size);

    log::info!("Starting terminal");

    std::io::stdout()
        .execute(crossterm::event::EnableMouseCapture)
        .expect("mouse should be captured");
    enable_raw_mode().expect("raw mode should be enabled");

    let mut terminal = ratatui::init();
    terminal.clear().expect("terminal should be cleared");

    let term_size = (size / ivec2(1, 2) + ivec2(2, 2)).as_u16vec2();

    let mut anim_state: Option<State> = None;
    let mut speed: i32 = 10; // per decisecond
    let mut is_running = true;

    while is_running {
        let curr_term_size = terminal.size().expect("terminal should have size");
        let speed_secs = speed as f32 / 10.0;

        match anim_state.as_mut() {
            None => terminal
                .draw(|frame| {
                    let [center] = Layout::horizontal([Constraint::Length(term_size.x)])
                        .flex(Flex::Center)
                        .areas(frame.area());
                    let [center] = Layout::vertical([Constraint::Length(term_size.y)])
                        .flex(Flex::Center)
                        .areas(center);

                    let paragraph = Paragraph::new("Space to start/restart the animation\nScroll to speed up/down\nQ to quit")
                        .block(
                            Block::bordered()
                                .title("Advent of Code 2024 - Day 14 Part 2 Animation")
                                .title_bottom(format!(
                                    "The terminal size must be at least {}x{} (currently {}x{}) | Speed: {:.1}x",
                                    term_size.x,
                                    term_size.y,
                                    curr_term_size.width,
                                    curr_term_size.height,
                                    speed_secs,
                                ))
                                .padding(Padding::new(
                                    0,
                                    0,
                                    center.height / 2 - 3,
                                    0,
                                )),
                        )
                        .alignment(Alignment::Center)
                        .wrap(Wrap { trim: false });

                    frame.render_widget(paragraph, center);
                })
                .expect("frame should be rendered"),
            Some(state) => {
                let elapsed = match state.elapsed < state.duration {
                    true => {
                        let dt = state.time.elapsed().mul_f32(speed_secs);
                        state.time = std::time::Instant::now();
            
                        state.elapsed += dt;
                        if state.elapsed > state.duration {
                            state.elapsed = state.duration;
                        }
            
                        state.elapsed
                    }
                    false => state.duration
                }.as_secs_f32();

                let (positions, velocities): (Vec<_>, Vec<_>) = state.robots.iter().map(|robot| (robot.p, robot.v)).unzip();
                let duration = state.duration.as_secs_f32();
                
                terminal
                    .draw(|frame| {
                        let [center] = Layout::horizontal([Constraint::Length(term_size.x)])
                            .flex(Flex::Center)
                            .areas(frame.area());
                        let [center] = Layout::vertical([Constraint::Length(term_size.y)])
                            .flex(Flex::Center)
                            .areas(center);

                        let canvas = Canvas::default()
                            .block(
                                Block::bordered()
                                    .title_top(
                                        "Advent of Code 2024 - Day 14 Part 2 Animation"
                                    )
                                    .title_bottom(
                                        format!(
                                            "Elapsed: {:4.2}s / {:4.2}s | Speed: {:.1}x",
                                            elapsed, duration, speed_secs,
                                        )
                                    )
                            )
                            .marker(Marker::HalfBlock)
                            .x_bounds([0.0, size.x as f64])
                            .y_bounds([0.0, size.y as f64])
                            .paint(|ctx| {
                                ctx.draw(&Points {
                                    coords: &positions
                                        .iter()
                                        .map(IVec2::as_vec2)
                                        .zip(velocities.iter().map(IVec2::as_vec2))
                                        .map(|(pos, vel)| (pos + vel * elapsed).rem_euclid(size.as_vec2()))
                                        .map(|pos| (pos.x as f64, size.y as f64 - pos.y as f64))
                                        .collect_vec(),
                                    color: Color::White,
                                });
                            });

                        frame.render_widget(canvas, center);
                    })
                    .expect("frame should be rendered")
            }
        };

        while event::poll(std::time::Duration::from_secs(0)).expect("event should be polled") {
            match event::read().expect("event should be read") {
                Event::Key(event) => match event.code {
                    KeyCode::Char(' ') => {
                        anim_state = Some(State::new(robots.clone(), steps));
                    }
                    KeyCode::Char('q') => {
                        is_running = false;
                        break;
                    }
                    _ => {}
                },
                Event::Mouse(event) => match event.kind {
                    MouseEventKind::ScrollUp => {
                        let speed_delta = match speed {
                            0..10 => 1,
                            10..100 => 10,
                            100..1000 => 100,
                            1000..10000 => 1000,
                            _ => 0,
                        };
                        speed = (speed + speed_delta).min(10000);
                    }
                    MouseEventKind::ScrollDown => {
                        let speed_delta = match speed {
                            0..=10 => 1,
                            11..=100 => 10,
                            101..=1000 => 100,
                            1001..=10000 => 1000,
                            _ => 0,
                        };
                        speed = (speed - speed_delta).max(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    ratatui::restore();

    disable_raw_mode().expect("raw mode should be disabled");
    std::io::stdout()
        .execute(crossterm::event::DisableMouseCapture)
        .expect("mouse should be released");
}
