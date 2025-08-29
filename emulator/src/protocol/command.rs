use crate::protocol::policy::Policy;

pub enum Command {
    Run(Option<Policy>),
    PauseRequest,
    Kill,

    GetRegisters,
}