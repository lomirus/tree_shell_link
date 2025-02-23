#![feature(path_add_extension)]

use std::{
    fs,
    io::ErrorKind,
    os::windows::ffi::OsStrExt,
    path::{Path, PathBuf},
};
use windows::{
    Win32::{
        System::Com::{
            CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
            IPersistFile,
        },
        UI::Shell::{IShellLinkW, ShellLink},
    },
    core::{Interface, PCWSTR},
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn path_buf_to_wide(path: PathBuf) -> Vec<u16> {
    if path.is_absolute() {
        path.to_str().unwrap()[4..]
            .encode_utf16()
            .chain(Some(0))
            .collect()
    } else {
        path.as_os_str().encode_wide().chain(Some(0)).collect()
    }
}

fn create_shortcut(target: &Path, shortcut_path: &Path) -> windows::core::Result<()> {
    let target = path_buf_to_wide(target.to_path_buf());
    let shortcut_path = path_buf_to_wide(shortcut_path.to_path_buf());

    unsafe {
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_ALL)?;

        shell_link.SetPath(PCWSTR(target.as_ptr())).unwrap();

        let persist_file: IPersistFile = shell_link.cast()?;
        persist_file.Save(PCWSTR(shortcut_path.as_ptr()), true)?;
    }

    Ok(())
}

fn main() {
    let Args { input, output } = Args::parse();

    let input_read_dir = match fs::read_dir(&input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("[INPUT] error: {err}");
            return;
        }
    };
    match fs::read_dir(&output) {
        Ok(value) => {
            if value.count() != 0 {
                eprintln!("[OUTPUT] error: directory is not empty");
                return;
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match fs::create_dir_all(&output) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("error when creating [OUTPUT]: {err}");
                    return;
                }
            },
            _ => {
                eprintln!("[OUTPUT] error: {err}");
                return;
            }
        },
    };
    let output = output.canonicalize().unwrap();

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok().unwrap();
    }
    let mut todo_read_dirs = vec![input_read_dir];
    while !todo_read_dirs.is_empty() {
        let last_read_dir = todo_read_dirs.pop().unwrap();
        for entry in last_read_dir {
            let entry = entry.unwrap();
            let entry_type = entry.file_type().unwrap();
            if entry_type.is_dir() {
                let read_dir = fs::read_dir(entry.path()).unwrap();
                todo_read_dirs.push(read_dir);
                let entry_path = entry.path();
                let affix = entry_path.strip_prefix(&input).unwrap();
                let symlink_path = output.join(affix);
                fs::create_dir_all(symlink_path).unwrap();
                continue;
            }
            let target_path = entry.path().canonicalize().unwrap();
            let shortcut_path = {
                let mut shortcut_path = output.join(entry.path().strip_prefix(&input).unwrap());
                shortcut_path.add_extension("lnk");
                shortcut_path
            };

            if let Err(err) = create_shortcut(&target_path, &shortcut_path) {
                eprintln!("{err}");
                eprintln!("  - target_path: {}", target_path.to_string_lossy());
                eprintln!("  - shortcut_path: {}", shortcut_path.to_string_lossy());
            };
        }
    }

    unsafe {
        CoUninitialize();
    }

    println!("Success");
}
