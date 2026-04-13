# Changelog

## dz6 v0.7.0

- Hex view:
  - Replace mode:
    - `t` now truncates the file (it was `T` before).
    - `T` reverse truncates the file (deletes from offset 0 to current offset).
    - Both commands above need confirmation from the user as they can't be undone.

## dz6 v0.6.0

- Hex view:
  -  `Ctrl+f` goes down one page (same as `PgDown`).
  - `Ctrl+b` goes up one page (same as `PgUp`).
  - `~` changes case when applicable (available in Normal, Replace, and Select modes).
  - `Ctrl+a`, `Ctrl+x`, `n`, and `z` work without putting the editor in Replace mode.
  - In Select mode, the number of selected bytes is now shown at the status bar.

## dz6 v0.5.1

- Improved search. Thanks to @yeggor, search is much faster now.
- Bug fixes:
  - Panic due to an out-of-bounds read if you open a large file, go to the last offset, and switch to text view (@yeggor).

## dz6 v0.5.0

- If you make a selection (`v` key) and change something, undo (`u` key) is now aware of it.
- Backward search is here! Press `?` key, just like in Vim. Also, `N` (`Shift+n`) to search for the previous result. Calculator key changed to `=` due to this addition.
- Search wrapping has arrived! It's enabled by default. Disable with `:set nowrapscan`. Re-enable with `:set wrapscan`.
- In select mode, you can now select up and down.
- New `:sel <offset> <length>` command to make a selection from the command bar.
- In hex view, the number of bytes per line can now be set automatically according to the screen size. Enable with `:set byteline auto` (it doesn't work with `~/.dz6init` yet).
- More descriptive error messages.
- Bug fixes:
  - Editing keeps only the last change most of the time (#25). Thanks to @sergiogarciadev for reporting it and suggesting a fix!
  - When editing the Hex and Text are showing different data (#26). Thanks to @sergiogarciadev again. :)
  - Multiple bugs related to zero-byte files.
  - If a target file has a related database (.dz6 file), dz6 would parse `editing_hex` as false, causing Replace mode to default to ASCII editing.
  - High CPU usage problem due to frequent event polling.
  - Selection is now reflected at the ASCII dump.
  - If an offset has a comment, it was being shown when the user starts a search with `/` or `?`.

## dz6 v0.4.2

- Thanks to @sergiogarciadev, dz6 now uses mmap-io (https://github.com/mentebinaria/dz6/pull/24) for file access.
- Due to the use of mmap-io, this version fixes #4 and #5.
- Hex view: scrolling is now smoother as expected from an editor.
- Hex view: extra offset added to quickly show the file size.
- Fix errors and inconsistencies when opening zero-byte files.

## dz6 v0.4.1

- Save a database file (a TOML file) at the target file directory containing bookmarks and comments. This data is restored when the same file is open. (https://github.com/mentebinaria/dz6/issues/14)
- Basic support for multibyte selection. Press `v` (like visual mode in vim), then select a byte range. Then, `y` to copy bytes to clipboard, `z` to fill them with zeroes, or `n` to fill them with x86 NOPs.
- Support for initialization file at `$HOME/.gdbinit`. Commands added to this file will be executed at startup (one per line). (https://github.com/mentebinaria/dz6/issues/12)
- New `:` commands:
    - `set db` turn on database loading/saving (default)
    - `set nodb` turn off the above

## dz6 v0.4.0

- Status bar now shows "COMMAND" when you press `:`.
- New `:` commands:
    - `cmt <offset> <comment>` (programatic alternative to `;`)
    - `set byteline <number>` sets the number of bytes per line in the hex dump
    - `set ctrlchar <char>`sets the character shown for ASCII non-graphical byte values
    - `set dimzero` dim nullbytes
    - `set dimctrl` dim all control characters
    - `set nodim` turn off dimming
    - `set theme` changes the theme
    - `w` write changes to file
    - `wq` or `x` write changes to file and quit
- In-memory buffer when patching bytes. Nothing is written to the file until you use some of the writing commands (`w`, `wq` or `x`), but truncating is an exception (`T` in replace mode).
 - Light theme.
- dz6 beeps if you try  to enter replace mode when editing a read-only file.

## dz6 v0.3.1

- Text view: use the area designated for the ruler in hex view.
- Text view: Navigate with `j` and `k` in encoding selection dialog.
- Search: rename ASCII search to UTF-8 search.
- `:` returns to NORMAL mode if no command is typed before pressing `Enter`.

## dz6 v0.3.0

- Quit command is now `:q` instead of `ESC`
- `.` is the default character shown in ASCII dump for non-ASCII characters
- `-r` command line switch to force opening a file in read only mode

## dz6 v0.2.0

- First public version.
