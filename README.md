# saver-cosmos

Official **cosmos** visualizer plugin for [IdleScreen](https://github.com/idlescreen/idle-core).

## Build

Requires a sibling checkout of the core daemon for `trance-api`:

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/saver-cosmos.git
cd saver-cosmos
cargo build --release
```

## Install

After adding the IdleScreen package repository:

```bash
sudo apt install trance-saver-cosmos
# or: sudo dnf install trance-saver-cosmos
```

See [idlescreen.github.io/idle-packages](https://idlescreen.github.io/idle-packages/).

## License

Apache-2.0. See [LICENSE](LICENSE).
