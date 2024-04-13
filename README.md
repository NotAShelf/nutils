# nutils

> ~~Utils so good, they make you nut~~ A collection of utilities for the [Nix package manager](https://github.com/NixOS/nix)

## Available Utilities

### nixfetch (WIP)

nixfetch fetches a file, adds it to the nix store and returns JSON data for the newly
added path: mainly the file path and the hash. It is ultimately a wrapper around
`nix store prefetch-file`. The nix store is used to avoid storing duplicates
of the same file.

## Building

You know how to use Rust, yes?

## Contributing

Do you know how to use Rust? I don't. Make a pull request if you find one of my many mistakes. You may unlock the
golden shelf skin at 99 mistakes found.

The repository follows a very simple structure: each utility must be its own workspace, and common functions
should go under the `commons` workspace.

### Adding new utilities

This is a monorepo for utility wrappers around the nix package manager, focusing on the package manager itself
rather than `nixos-rebuild` unlike the awesome [nh](https://github.com/viperml/nh) project. If you wish to add
an utility, feel free to.

## License

All workspaces in this repository are licensed under GPLv3 or above. See [LICENSE](LICENSE) for
details and restrictions.
