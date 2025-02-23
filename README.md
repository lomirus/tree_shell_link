## Introduction

> [!WARNING]  
> This program is only specified for Windows platform.

Generate shell links (Windows shortcut, i.e. `.lnk` file) from the specified directory recursively.

For example, this is `C:/foo`:

```
foo
├─a
└─b
    └─c
```

Run `tree_shell_link foo bar`, then `C:/bar` will be:

```
bar
├─a.lnk
└─b
    └─c.lnk
```