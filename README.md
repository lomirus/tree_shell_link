## Introduction

> [!WARNING]  
> This program is only specified for Windows platform.

Generate shell links (Windows shortcut, i.e. `.lnk` file) from the specified directory recursively.

For example, this is `C:/tree_shell_link/foo`:

```
foo
├─a
└─b
    └─c
```

Run `cargo run -- foo bar`, then `C:/tree_shell_link/bar` will be:

```
bar
├─a.lnk
└─b
    └─c.lnk
```