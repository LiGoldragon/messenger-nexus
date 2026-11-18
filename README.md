# Messenger Nexus research prototype

An isolated implementation of MessengerNexus.1: a Matrix room, one thread
per subject, with each thread owned by the flow that opened it. Room-level
messages go to the configured speaker. Matrix SDK event handlers receive
messages; a server-held sync connection supplies events.

## Contract checkpoint

The six schemas under `crates/core/ethos/` describe Library, ordinary and
meta Signal, Operation, Memory, and Emission. They build on the design at
`/home/li/primary/flows/1e2133/proposals/messenger-nexus.md`.
All three pass `Check` and `Generate` with published Ethos Zero 30.1.0.
Generated Rust is committed under `crates/core/src/generated/`.
Generation alone does not prove Rust compilation. `operations.rs` declares
the runtime capability contracts before their implementations.

Psyche core Opus filled the research contract in request `zs5u`: Open takes
an explicit opener Address as a tentative prototype stand-in. The
translator filters the configured room/user and deduplicates Matrix event
identities before forming Heard. Memory keeps thread records and meta
configuration. A separate typed Deliver emission calls the Flow CLI until
a typed Flow edge exists. Text works first; images follow later.

Request `4gtr` additionally rules SDK failures as a tentative typed enum;
only the Flow subprocess has a native exit status. Configuration is set one
value at a time and observed as a snapshot followed by changes.

## Research boundaries

The intended transport is matrix-sdk 0.19.1 with E2EE and a persistent
SQLite crypto store. Its published crate declares Rust 1.96 as its minimum.
The prototype uses throwaway accounts and an isolated room. No living
account, room, credentials, or device verification is used.

The first Cargo member is `messenger-core`, a transport-independent library
with generated vocabulary, trait-first contracts, and hand-written operation
bodies. The operation bodies have been parsed/formatted but not compiled. A lock,
Cargo.nix, build-base feature/version registration, Rust compilation,
runtime adapters, clients and tests remain to be completed.

An isolated, ready-built Synapse 1.164.2 fixture was set up in private
scratch space: two throwaway users joined one encrypted room, encryption
state was confirmed, and the server shut down cleanly. Zero messages were
sent. This proves fixture setup, not SDK E2EE or thread delivery. Live
fixture state and credentials are not repository content.
