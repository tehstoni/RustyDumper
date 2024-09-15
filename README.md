RustyDumper

RustyDumper is a simple command-line tool written in Rust for creating memory dumps of running processes on Windows systems. It utilizes Windows APIs to generate a full memory dump, which can be useful for debugging, analysis, or forensic purposes.
Features

    Process Memory Dumping: Dump the full memory of any running process using its PID.
    Custom Output Path: Specify the output file path for the memory dump.
    Easy to Use: Simple command-line interface with straightforward arguments.

Prerequisites

    Operating System: Windows 7 or later.
    Rust Toolchain: Rust 1.56.0 or later.
    Cargo: Package manager for Rust.

Installation

Clone the repository and build the project using Cargo:

```bash

git clone https://github.com/tehstoni/RustyDumper.git
cd RustyDumper
cargo build --release
```

The compiled executable will be located in the target/release directory.
Usage

```bash

rustydumper.exe <PID> <OUTPUT_PATH>

    <PID>: Process ID of the target process you want to dump.
    <OUTPUT_PATH>: File path where the memory dump will be saved.
```
Example

```bash
rustydumper.exe 1234 C:\dumps\process_dump.dmp
```
This command will create a full memory dump of the process with PID 1234 and save it to C:\dumps\process_dump.dmp.
Command-Line Arguments

    pid (u32): The Process ID of the target application.
    output (PathBuf): The output file path for the memory dump.

Building from Source

Ensure you have the Rust toolchain installed. Then run:

```bash

cargo build --release
```

Dependencies

    clap: Command-line argument parsing.
    windows: Windows API bindings for Rust.

How It Works

RustyDumper performs the following steps:

    Open Process: Uses OpenProcess to obtain a handle to the target process.
    Create Output File: Utilizes CreateFileW to create the dump file.
    Write Dump: Calls MiniDumpWriteDump to write the memory dump to the file.
    Cleanup: Closes all handles to free resources.

Disclaimer

    Permissions: Ensure you have the necessary permissions to access and dump the target process.
    Legal Use: Use this tool responsibly and legally. Unauthorized memory dumping may violate software licenses or laws.
