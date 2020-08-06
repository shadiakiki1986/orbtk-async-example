Setting up the dev environment on a fresh Ubuntu server 20.04 machine

```
sudo apt-get update

# install cargo deps
sudo apt install build-essential

# install a desktop GUI
sudo apt install lxde # fetches 300 MB, 1.6 GB additional disk space, will prompt to choose a display manager (lightdm, ggdm3?)
sudo apt install libsdl2-dev

# install cargo
curl https://sh.rustup.rs -sSf | sh

# install orbtk deps
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev cmake libfreetype6-dev libexpat-dev

# how to create this project
#cargo new --bin orbtk-async-example
git clone git@gitlab.com:shadiakiki1986/orbtk-async-example.git

# enable remote desktop
sudo apt install xrdp
sudo passwd ubuntu
```


cargo-fu

```
cargo build
cargo run
cargo run --example futures_threadpool
cargo run --example crossbeam_channel
cargo run --example crossbeam_thread
cargo run --example orbtk_minimal
cargo run --example orbtk_oneButton_sync
cargo run --example orbtk_oneButton_async
```

Get gui on ec2:
- https://www.australtech.net/how-to-enable-gui-on-aws-ec2-ubuntu-server/
- https://phoenixnap.com/kb/how-to-install-a-gui-on-ubuntu
