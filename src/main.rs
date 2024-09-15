use clap::Parser;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use windows::{
    core::PCWSTR,
    Win32::{Foundation::{CloseHandle, GENERIC_WRITE, HANDLE}, Storage::FileSystem::{CreateFileW, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ}, System::{Diagnostics::Debug::{MiniDumpWithFullMemory, MiniDumpWriteDump}, Threading::{OpenProcess, PROCESS_ALL_ACCESS}}},
};

fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    pid: u32,

    output: PathBuf,
}



fn main() -> windows::core::Result<()> {
    let args = Args::parse();

    let process_handle = unsafe {
        OpenProcess(PROCESS_ALL_ACCESS, false, args.pid)
    }?;

    let output_wide: Vec<u16> = to_wide_chars(args.output.to_str().unwrap());
    let file_handle = unsafe {
        CreateFileW(
            PCWSTR(output_wide.as_ptr()),
            GENERIC_WRITE.0,
            FILE_SHARE_READ,
            None,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )
    }?;

    let result = unsafe {
        MiniDumpWriteDump(
            process_handle,
            args.pid,
            file_handle,
            MiniDumpWithFullMemory,
            None,
            None,
            None,
        )
    };

    if !result.is_ok() {
        eprintln!("Failed to write dump");
        return Err(windows::core::Error::from_win32());
    }

    unsafe {
        let _ = CloseHandle(file_handle);
        let _ = CloseHandle(process_handle);
    }

    Ok(())
}
