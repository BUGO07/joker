# Got bored again, decided to make Joker (the card game)

## Features
- Card logic
- Calling / Taking
- Rounds system
- Nines mode
- Classic mode
- H Penalties

## TODO
- Bonuses
- Nicer UI / Animations
- Networking

## Building and running
### Requirements
- [`rust`](https://rustup.rs/) - obviously
- `rustup target install wasm32-unknown-unknown` rust target (optional, needed for web builds)
- `cargo install wasm-server-runner` (optional, needed for web builds)

### Building
1. Clone the repo:\
    `git clone --depth=1 https://github.com/BUGO07/joker`
2. `cd joker`
3. `cargo run` - builds and runs the project.
4. (Optional) `cargo run --target wasm32-unknown-unknown` builds and serves wasm.

## Card Assets
- [Neon Orbis' Playing Cards](https://neonorbis.itch.io/playing-cards)