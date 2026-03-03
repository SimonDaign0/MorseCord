mod board;
mod morse;
mod webhook;

use board::Board;
use rppal::gpio::Trigger;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use webhook::send_discord_msg;

use crate::morse::mrs_code::decode;

#[derive(PartialEq, Eq)]
enum State {
    Idle,
    Listening,
}

struct Context {
    state: Mutex<State>,
    seq: Mutex<String>,
    msg: Mutex<String>,
    last_pressed: Mutex<Option<Instant>>,
    is_pressed: Mutex<bool>,
}

impl Context {
    fn new() -> Self {
        Self {
            state: Mutex::new(State::Idle),
            seq: Mutex::new(String::new()),
            msg: Mutex::new(String::new()),
            last_pressed: Mutex::new(None),
            is_pressed: Mutex::new(false),
        }
    }
}

fn main() {
    const BUILD_TIME: Duration = Duration::from_millis(700);
    const PUSH_MSG: Duration = Duration::from_secs(2);

    let board = Arc::new(Mutex::new(Board::new(21, 19)));
    let ctx = Arc::new(Context::new());

    init_btn(Arc::clone(&board), Arc::clone(&ctx));

    // startup blink
    board.lock().unwrap().blink(2);

    println!("Press to start Morse input...");

    loop {
        let last_press = {
            let guard = ctx.last_pressed.lock().unwrap();
            *guard
        };
        if let Some(last) = last_press {
            let elapsed = Instant::now() - last;
            if !ctx.msg.lock().unwrap().is_empty()
                && *ctx.is_pressed.lock().unwrap()
                && elapsed > PUSH_MSG
            {
                // Send message to Discord
                let webhook = "YOUR_DISCORD_WEBHOOK_URL";
                let username = "Morse Bot";
                send_discord_msg(webhook, &ctx.msg.lock().unwrap(), Some(username), None);
                push_msg(&ctx);
                *ctx.state.lock().unwrap() = State::Idle;
                board.lock().unwrap().blink(2);
                println!("Listening...");
                *ctx.state.lock().unwrap() = State::Listening;
            } else if elapsed > BUILD_TIME {
                let seq_string = {
                    let mut seq = ctx.seq.lock().unwrap();
                    if seq.is_empty() {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }

                    let s = seq.clone();
                    seq.clear();
                    s
                };

                let decoded = decode(&seq_string);
                finalise_char(&ctx, decoded);
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn init_btn(board: Arc<Mutex<Board>>, ctx: Arc<Context>) {
    // clone LED once
    let led = {
        let board_guard = board.lock().unwrap();
        Arc::clone(&board_guard.led)
    };

    let mut board_guard = board.lock().unwrap();

    board_guard
        .btn
        .set_async_interrupt(Trigger::Both, millis(35), move |event| {
            let mut state = ctx.state.lock().unwrap();

            if *state == State::Idle {
                *state = State::Listening;
                return;
            }

            match event.trigger {
                // Button pressed
                Trigger::FallingEdge => {
                    led.lock().unwrap().set_high();
                    *ctx.is_pressed.lock().unwrap() = true;
                    let mut last = ctx.last_pressed.lock().unwrap();
                    *last = Some(Instant::now());
                }

                // Button released
                Trigger::RisingEdge => {
                    led.lock().unwrap().set_low();
                    *ctx.is_pressed.lock().unwrap() = false;
                    let mut last = ctx.last_pressed.lock().unwrap();

                    if let Some(previous) = *last {
                        let elapsed = Instant::now() - previous;
                        build_char(&ctx, elapsed);
                    } else {
                        println!("Listening...");
                    }

                    *last = Some(Instant::now());
                }

                _ => {}
            }
        })
        .unwrap();
}

fn build_char(ctx: &Context, duration: Duration) {
    const DOT_MAX: Duration = Duration::from_millis(150);
    const DASH_MAX: Duration = Duration::from_millis(700);

    let mut seq = ctx.seq.lock().unwrap();

    if duration < DOT_MAX {
        seq.push('.');
        println!(".");
    } else if duration < DASH_MAX {
        seq.push('-');
        println!("-");
    }
}

fn finalise_char(ctx: &Context, ch: Option<char>) {
    println!("Finalising character...");

    let mut msg = ctx.msg.lock().unwrap();

    match ch {
        Some('<') => {
            if !msg.is_empty() {
                msg.pop();
            }
        }
        Some('>') => {
            msg.push(' ');
        }
        Some(c) => {
            msg.push(c);
        }
        None => {
            println!("Invalid Morse sequence");
        }
    }

    println!("Current message: {}", &*msg.replace(" ", "_"));
}

fn millis(ms: u64) -> Option<Duration> {
    Some(Duration::from_millis(ms))
}

fn push_msg(ctx: &Context) {
    let mut msg = ctx.msg.lock().unwrap();
    println!("Final msg: {}", msg);
    msg.clear();
}
