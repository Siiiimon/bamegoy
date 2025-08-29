use bamegoy_core::{protocol::{command::Command, event::Event, policy}, Emulator};




#[test]
pub fn init_kill() {
    let emulator = Emulator::init(vec![0; 0x8000], false);

    let r = emulator.rx.recv();
    assert_eq!(Ok(Event::Paused), r);

    let s = emulator.tx.send(
        Command::Kill
    );
    assert!(s.is_ok());
    assert!(emulator.thread.join().is_ok());
}

#[test]
pub fn step() {
    let emulator = Emulator::init(vec![0; 0x8000], false);

    let paused0 = emulator.rx.recv();
    assert_eq!(Ok(Event::Paused), paused0);

    let _ = emulator.tx.send(
        Command::GetRegisters
    );
    let initial_registers = if let Event::Registers(r) = emulator.rx.recv().unwrap() {
        r
    } else {
        panic!("expected registers")
    };

    let _ = emulator.tx.send(
        Command::Run(Some(policy::single_step()))
    );
    let running0: Result<Event, std::sync::mpsc::RecvError> = emulator.rx.recv();
    assert_eq!(Ok(Event::Running), running0);
    let paused1 = emulator.rx.recv();
    assert_eq!(Ok(Event::Paused), paused1);

    let _ = emulator.tx.send(
        Command::GetRegisters
    );
    let stepped_registers = if let Event::Registers(r) = emulator.rx.recv().unwrap() {
        r
    } else {
        panic!("expected registers")
    };

    assert_eq!(initial_registers.pc + 1, stepped_registers.pc);
    
    let kill = emulator.tx.send(
        Command::Kill
    );
    assert!(kill.is_ok());
    assert!(emulator.thread.join().is_ok());
}