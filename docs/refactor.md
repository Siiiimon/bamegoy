#### GUI <-> Emulator Communication

_Problem_:
the command based channel communication does not work well for transmitting things
like frame buffers or registers effectively.

_Fix_:
we should have other means of communication, even if they're less 'safe', but
an atomic buffer, which the emulator writes into and the gui can read from, would
be a better fit for audio / graphics data at least, where performance is necessary.

_Implementation_:
the Driver/Emulator Messages responsibility is to dictate the emulators state
from the outside on a relatively surface level. they shouldn't be integrated too
tightly in the cpu's run cycle where it's architecture has be deformed for the sake
of them. how things like watchers and breakpoints and so on will work remains to be
seen.
