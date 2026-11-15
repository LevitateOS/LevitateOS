# Release public keys

The `*.pub` files here are the release signing keys' public halves (the 32-byte
Ed25519 public key in base64 and a newline), made by `cargo xtask release keygen`.
Every image carries them at `/usr/share/levitate/release-keys/`, where `recupdate`
checks a release's signature against them. An image built without a key refuses
every release. The private key never comes here (docs/release-engineering.md,
sections 2 and 10).
