# plugin-cosmos

Official **cosmos** visualizer plugin for [IdleScreen](https://github.com/idlescreen/idlescreen) (Trance daemon).

## Build

Requires a sibling checkout of the daemon for `trance-api`:

```bash
git clone https://github.com/idlescreen/idlescreen.git
git clone https://github.com/idlescreen/plugin-cosmos.git
cd plugin-cosmos
cargo build --release
```

## Install

After adding the IdleScreen package repository:

```bash
sudo apt install trance-plugin-cosmos
# or: sudo dnf install trance-plugin-cosmos
```

See [idlescreen.github.io/packages](https://idlescreen.github.io/packages/).

## License

Apache-2.0. See [LICENSE](LICENSE).
