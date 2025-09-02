use std::sync::mpsc::TryRecvError;
use iced::Element;
use iced::widget::{button, row, text};
use bamegoy_core::{Emulator, Handle};
use bamegoy_core::protocol::command::Command;
use bamegoy_core::protocol::event::Event;
use bamegoy_core::protocol::policy;

pub struct App {
    handle: Handle,

    is_running: bool,
}

impl App {
    pub fn new(cartridge: Vec<u8>) -> Self {
        let handle = Emulator::init(cartridge, false);
        Self {
            handle,

            is_running: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PausePressed,
    RunPressed,
}

fn handle_event(app: &mut App) {
    let maybe_event = app.handle.rx.try_recv();
    if let Err(e) = maybe_event {
        if e == TryRecvError::Empty {
            return;
        } else {
            panic!("{}", e);
        }
    }

    match maybe_event.unwrap() {
        Event::Paused => app.is_running = false,
        Event::Running => app.is_running = true,
        _ => {},
    }
}

pub fn update(app: &mut App, message: Message) {
    match message {
        Message::PausePressed => {
            app.handle.tx.send(Command::PauseRequest).unwrap()
        },
        Message::RunPressed => {
            app.handle.tx.send(Command::Run(Some(policy::run_forever()))).unwrap()
        }
    }

    // FIX: update only runs if there's a Message
    // handle_event should run whenever a new event is in the pipe
    handle_event(app);
}

pub fn view(app: &App) -> Element<Message> {
    // TODO: do some composition: https://docs.rs/iced/0.13.1/iced/#scaling-applications
    let run_button_label = if app.is_running {"||"} else {">"};
    let run_button_message = if app.is_running {Message::PausePressed} else {Message::RunPressed};
    let registers = app.handle.register_snapshot.load();

    row![
        button(run_button_label).on_press(run_button_message),
        text(registers.pc)
    ].into()
}