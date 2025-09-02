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

impl Default for App {
    fn default() -> Self {
        let handle = Emulator::init(vec![0; 0x8000], false);
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
            app.handle.tx.send(Command::Run(Some(policy::single_step()))).unwrap()
        }
    }

    handle_event(app);
}

pub fn view(app: &App) -> Element<Message> {
    let run_button_label = if app.is_running {"||"} else {">"};
    let run_button_message = if app.is_running {Message::PausePressed} else {Message::RunPressed};

    row![
        button(run_button_label).on_press(run_button_message),
        text("Hello World")
    ].into()
}