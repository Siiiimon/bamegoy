┌── main
│   entry point and combines emulator core with gui frontend.
├── emulator
│   the emulator module only deals with how to instantiate itself and orchestrates
│   it's lifecycle loop by delegating remaining tasks to submodules.
├──── runtime
│     the runtime handles overall emulator state mutation, like executing policies.
├──── host
│     the host module is the interface to any frontend driving the emulator. it contains
│     run policies and channel message protocols.
