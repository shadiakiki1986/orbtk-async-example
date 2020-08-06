This repo holds an example implementation for thread listeners in orbtk:
https://github.com/redox-os/orbtk/issues/243


## Usage

There are multiple implementations in the `examples` folder that I used as steps
to develop the main example `examples/orbtk_oneButton_async.rs`.

To build and run the GUI example running a separate thread:

```
cargo run --example orbtk_oneButton_async
```

This example builds an orbtk GUI with 2 buttons:
- Button 1 (named 'S'): Runs some syncronous code (defined in `do_sync` below and contains a sleep) and shows how the GUI hangs
- Button 2 (named 'A'): Runs the same sleep code in a separate thread (defined in `do_async` and main) and shows how the GUI doesn't hang
Clicking this button shows the async code display to stdout in the terminal from
which the GUI was built/launched. Clicking it multiple times will repeat the async
code.

The GUI is ugly and could be improved, but this is just a proof-of-concept.



## Useful commands

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
