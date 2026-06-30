# stargem spectator-client

Live, read-only Bevy spectator for stargem matches. Subscribes to
`SpectatorService.SubscribeMatch` over gRPC and renders ships in 3D
with an event journal.

## Build

    nix develop -c cargo build --release

## Run against a scenario-runner

    # terminal 1, in server/:
    cargo run --release --bin scenario-runner -- --scenario ship_destruction_kinetic --loop_

    # terminal 2, in spectator-client/:
    cargo run --release -- --match-name gunship_cannon_destroys_recon
