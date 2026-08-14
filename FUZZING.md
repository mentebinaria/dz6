# Fuzzing

There are currently two fuzzing targets for the `dz6` crate: `app_load_file`, which fuzz test the hex editor's ability to open and load files from the filesystem, and `commands_parse_command`, which tests the ability to correctly parse commands in the TUI.

## Dictionaries

A [dictionary](https://appsec.guide/docs/fuzzing/techniques/dictionary/) for valid `dz6` commands can be found in `fuzz/dict/commands.dict`. Passing one dictionary to libFuzzer lets it splice whole keywords and punctuation into inputs instead of rediscovering them one byte at a time, significantly improving coverage and efficiency.

# Fuzzing a target

In order to run a target, you will need to install [cargo-fuzz](https://rust-fuzz.github.io/book/introduction.html) via:

```
$ rustup install nightly
$ cargo install cargo-fuzz
```

Next, go to the `fuzz/` directory, and run:

```
$ cargo fuzz run [target]
```

Additionally, you can fuzz the command parser target together with its dictionary by invoking:

```
$ cargo fuzz run commands_parse_command -- -dict=dict/commands.dict
```

The path is relative to the directory you run the command from. libFuzzer reports how many entries it loaded at startup.

> Fuzzing speed can be improved by disabling [ASan](https://github.com/google/sanitizers/wiki/addresssanitizer), which can be done by running `cargo fuzz run -s none [target]`.
