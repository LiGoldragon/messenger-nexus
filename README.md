# Messenger Nexus research prototype

An isolated implementation of MessengerNexus.1: a Matrix room, one thread
per subject, with each thread owned by the flow that opened it. Room-level
messages go to the configured speaker. Matrix SDK event handlers receive
messages; a server-held sync connection supplies events.

## Contract checkpoint

`ethos/messenger.ethos`, `ethos/signal.ethos`, and `ethos/emission.ethos`
transcribe the library, Signal, and Emission of the supplied design at
`/home/li/primary/flows/1e2133/proposals/messenger-nexus.md`.
All three pass `Check` and `Generate` with published Ethos Zero 30.1.0.
Generated Rust was inspected in temporary scratch space, not committed.
Their imports remain conceptual names from the design rather than resolved
crate dependencies; generation alone does not prove Rust compilation.

The supplied design leaves Operation, Memory, meta configuration, and
caller-to-flow identity unspecified. The design owner, Psyche core Opus,
has the combined contract question before implementation bodies.

## Research boundaries

The intended transport is matrix-sdk 0.19.1 with E2EE and a persistent
SQLite crypto store. Its published crate declares Rust 1.96 as its minimum.
The prototype uses throwaway accounts and an isolated room. No living
account, room, credentials, or device verification is used.

At this checkpoint there are no executable bodies, Cargo workspace,
compiled artifacts, account registrations, or live messages.
