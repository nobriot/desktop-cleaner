# Don't put stuff on the Desktop!

When we were visiting [Dropzone Denmark](https://www.dropzonedenmark.dk/en),
we noticed on the shared computers that they have a problem to have people
organize their content, and their desktop get polluted.

The best way to prevent people from adding things on your Desktop...
Is to just delete everything automatically 😀

## Introducing to you .... the Desktop cleaner

You'll need [cargo](https://www.rust-lang.org/tools/install) to build.

Then you can run 

```console
cargo build --release && ./target/release/desktop-cleaner
```

### Install

To build and install, run 

```console
make install
```

It is possible to un-install with 

```console
make uninstall
```

### Options

You can specify the interval at which the desktop cleaning runs, in seconds,
using the `--interval` argument. For example, run every 15 seconds:

```console
desktop-cleaner --interval 15
```

If you want to specify which directory is the home directory
(e.g. clean Desktops of other users), you can use `--home-dir`

```console
desktop-cleaner --home-dir $(pwd)
```

Files can be recovered from the trash-bin, but if you want to test it you can 
run with the `--dry-run` option:

```console
desktop-cleaner --dry-run
```

