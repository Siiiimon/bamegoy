use bamegoy::emulator::{policy, Emulator, EmulatorMessage};

#[test]
pub fn init_kill() {
    let emulator = Emulator::init(vec![0; 0x8000], false);

    let r = emulator.rx.recv();
    assert_eq!(Ok(EmulatorMessage::Paused), r);

    let s = emulator.tx.send(
        bamegoy::emulator::DriverMessage::Kill
    );
    assert!(s.is_ok());
    assert!(emulator.thread.join().is_ok());
}

#[test]
pub fn step() {
    let emulator = Emulator::init(vec![0; 0x8000], false);

    let paused0 = emulator.rx.recv();
    assert_eq!(Ok(EmulatorMessage::Paused), paused0);

    let _ = emulator.tx.send(
        bamegoy::emulator::DriverMessage::GetRegisters
    );
    let initial_registers = if let EmulatorMessage::Registers(r) = emulator.rx.recv().unwrap() {
        r
    } else {
        panic!("expected registers")
    };

    let _ = emulator.tx.send(
        bamegoy::emulator::DriverMessage::Run(Some(policy::single_step()))
    );
    let running0: Result<EmulatorMessage, std::sync::mpsc::RecvError> = emulator.rx.recv();
    assert_eq!(Ok(EmulatorMessage::Running), running0);
    let paused1 = emulator.rx.recv();
    assert_eq!(Ok(EmulatorMessage::Paused), paused1);

    let _ = emulator.tx.send(
        bamegoy::emulator::DriverMessage::GetRegisters
    );
    let stepped_registers = if let EmulatorMessage::Registers(r) = emulator.rx.recv().unwrap() {
        r
    } else {
        panic!("expected registers")
    };

    assert_eq!(initial_registers.pc + 1, stepped_registers.pc);
    
    let kill = emulator.tx.send(
        bamegoy::emulator::DriverMessage::Kill
    );
    assert!(kill.is_ok());
    assert!(emulator.thread.join().is_ok());
}