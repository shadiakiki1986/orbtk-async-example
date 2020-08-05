Useful commands

```
# install cargo deps
sudo apt install build-essential

# install cargo
curl https://sh.rustup.rs -sSf | sh

# install orbtk deps
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev cmake libfreetype6-dev libexpat-dev

# how to create this project
cargo new --bin orbtk-async-example

# cargo-fu
cargo build
cargo run
cargo run --example crossbeam_channel
cargo run --example crossbeam_thread
cargo run --example orbtk_simple
```

Get gui on ec2:
- https://www.australtech.net/how-to-enable-gui-on-aws-ec2-ubuntu-server/
- https://phoenixnap.com/kb/how-to-install-a-gui-on-ubuntu
