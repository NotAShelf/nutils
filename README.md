# nutils

> ~~Utils so good, they make you nut~~ A collection of utilities for the [Nix package manager](https://github.com/NixOS/nix)

## Available Utilities

### nixfetch [0.1.0]

nixfetch fetches a file, adds it to the nix store and returns JSON data for the newly
added path: mainly the file path and the hash. It is ultimately a wrapper around
`nix store prefetch-file`. The nix store is used to avoid storing duplicates
of the same file.

#### Usage

```bash
$ nixfetch https://raw.githubusercontent.com/NotAShelf/nutils/main/LICENSE # e.g.: try to fetch a path from Github
{"hash":"sha256-OXLcl0T2SZ8Pmy2/dmlvKuetivmyPd5m1q+Gyd+zaYY=","storePath":"/nix/store/zgw1a0w4j9anca8kjmgizy1m5zc9c0k2-LICENSE"}
```

### flakecheck [0.1.0] (WIP)

flakecheck runs `nix flake check` with the `--no-build` argument and monitors the
output for file errors containing the message `is not valid`. If a broken path is
found, then attempts to repair the given path with `nix-store --repair-path`.

This is not a magic bullet. You should be concerned if your store produces broken
paths, and investigate carefully.

## Building

You know how to use Rust, yes?

Build the workspace you'd like to use. Nix package is a TODO because I don't really
know how to build a multi-workspace project with Nix.

## Contributing

Do you know how to use Rust? I don't. Make a pull request if you find one of my many mistakes. You may unlock the
golden shelf skin at 99 mistakes found.

The repository follows a very simple structure: each utility must be its own workspace. If you find yourself
repeating code, start abstracting or add a `commons` workspace.

### Adding new utilities

This is a monorepo for utility wrappers around the nix package manager, focusing on the package manager itself
rather than `nixos-rebuild` unlike the awesome [nh](https://github.com/viperml/nh) project. If you wish to add
an utility, feel free to. Format your code with `rustfmt`, leave proper comments.

## License

All workspaces in this repository are licensed under GPLv3 or above. See [LICENSE](LICENSE) for
details and restrictions.
