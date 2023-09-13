# rusty-ifinfo

A very simple tool that shows various information of network interfaces in a
Linux system. This is **not** meant for production use, it is merely created
to test the `clatox-netlink` crate.

## Building
Just clone the repo and run `cargo build` in the directory this README is located
in. The final binary will be placed in `./target/debug/` or `./target/release/`.

## Usage
Run the binary. If everything goes well, you should see a long list of attributes
printed for each network interface.

## License
This tool and its source code are GPLv2-licensed.
