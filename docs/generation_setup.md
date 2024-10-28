# Terrain Generation Setup

Terrain generation is driven through Rust. All of the code for the terrain generator is in the
`rgen` directory at the root of this repository.

## Getting started

First, you'll need to install [Rust](https://www.rust-lang.org/tools/install). Any relatively recent
version should work.

Then, once you have rust installed, open up a terminal in the nested `rgen` directory in this
project. Build the project by running `cargo build`.

If this works, you can move onto the next step. If it failed, then take a look at the
"troubleshooting" section below. to hit errors:

## Hooking up the terrain generator to the mod

We use MultiMC when developing the terrain generator. So, create a new MultiMC instance for 1.12.2,
and install forge and the RGen mod. See the `basic_setup.md` file for compiling and installing the
mod.

Once you have a MultiMC instance, open up the "minecraft folder" for it, and copy the path to that 
folder. This path on my linux machine looks like this (I called my instance `1.12.2-rgen`):
```
~/.local/share/multimc/instances/1.12.2-rgen
```

On MacOS, it'll look like this (I have the MultiMC app on my desktop):
```
/Users/macmv/Desktop/MultiMC.app/Data/instances/1.12.2-rgen
```

Now, take that path (make sure it ends in the instance name, not in `.minecraft`), and go back to
the command line with the compiled project (where `cargo build` was run). Run the following command:
```
./multimc.sh ~/.local/share/multimc/instances/1.12.2-rgen
```

And this, finally, will hook up the rust library to the multimc instance. The `multimc.sh` command
may spit out some errors, but they can be ignored.

Now that you have the rust library linked up, start the multimc instance. Make a new creative world
called "1", give it the seed "1", and set the world type to "RGen". Open up the world, and assuming
everything worked, you should be looking at the RGen terrain.

Note that multimc will clear the attached library every time you start the instance. So, before
starting the instance, the `./multimc.sh` script must be run again.

## Troubleshooting

```
$ cargo build
command not found: cargo
```
You didn't install rust, or rust isn't in your PATH.

```
$ cargo build
error: could not find `Cargo.toml`
```
You didn't `cd` into the `rgen` directory before running `cargo build`.

**Game crashes**
```
no rgen_jni found in java.library.path
```
MultiMC deleted the rust library, run the `multimc.sh` script again.
