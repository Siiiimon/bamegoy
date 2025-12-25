pub struct Runtime {
    state: State,
}

#[derive(PartialEq)]
pub enum State {
    PauseRequested,
    Paused,
    Running,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            state: State::Paused,
        }
    }

    fn handle_state(&mut self) {
        match self.state {
            State::PauseRequested => {
                self.state = State::Paused;
                self.tx.send(EmulatorMessage::Paused).unwrap();
            }
            State::Paused => {}
            State::Running => match (self.cpu.try_lock(), self.bus.try_lock()) {
                (Ok(mut cpu), Ok(mut bus)) => {
                    cpu.step(&mut *bus);
                    if let Some(p) = &mut self.runtime.policy {
                        if p(&*cpu, &*bus) {
                            self.runtime.policy = None;
                            self.runtime.state = State::PauseRequested;
                        }
                    }
                }
                (Err(TryLockError::WouldBlock), _) | (_, Err(TryLockError::WouldBlock)) => {}
                (Err(TryLockError::Poisoned(_)), _) | (_, Err(TryLockError::Poisoned(_))) => {
                    panic!("CPU or Bus lock poisoned!");
                }
            },
        }
    }
}
