pub mod settings;
pub mod disassembly;
pub mod breakpoint;
pub mod controls;
pub mod info;
pub mod memory;
pub mod serial;

pub struct Views {
    controls_view: controls::ControlsView,
    settings_view: settings::SettingsView,
    disassembly_view: disassembly::DisassemblyView,
    breakpoint_view: breakpoint::BreakpointView,
    info_view: info::InfoView,
    memory_view: memory::MemoryView,
}