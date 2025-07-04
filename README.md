# HSH: A Simple Shell in Rust


HSH (Hobby SHell) is a basic command-line shell implemented in **Rust**. This project goes beyond just a simple command executor; it serves as a practical, hands-on example for understanding the core mechanics of a shell. It demonstrates fundamental concepts of system programming, including command parsing, interacting with the file system, and basic process control in a safe and efficient language like Rust. HSH is designed to be extensible, providing a solid foundation for exploring more advanced shell features.

## ✨ Features

HSH provides the following built-in commands:

* **`cat <file>`**: Reads and displays the content of a specified file to the console.
* **`ls`**: Lists the contents of the current working directory. By default, it hides files and directories that start with a dot (`.`) on Unix-like systems, offering a cleaner view.
* **`clear`**: Clears the terminal screen, providing a fresh workspace.
* **`cd <directory>`**: Changes the current working directory. Supports relative and absolute paths.
* **`exit`**: Gracefully terminates the shell session.
* **`touch <file>`**: Creates a new, empty file if it doesn't exist. If the file already exists, it updates its last modified timestamp to the current time without altering the file's content.

## 🚀 Getting Started

These instructions will help you get HSH up and running on your local machine for development and testing.

### Prerequisites

You'll need **Rust and Cargo** installed on your system. If you don't have them, the easiest way to install them is via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
````

### Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/itshsvm/hsh.git
    cd hsh
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

    This command compiles the project in **release mode**, which generates an optimized executable for better performance.

3.  **Run the shell:**

    ```bash
    ./target/release/hsh
    ```

    You should now see the `hsh>` prompt, ready for your commands\!

## 💡 Usage

Once the shell is running, you can type any of the supported commands at the `hsh>` prompt and press `Enter`.

```bash
hsh > ls
src   target   README.md   Cargo.toml
hsh > touch new_document.txt
hsh > ls
src   target   README.md   Cargo.toml   new_document.txt
hsh > cat README.md
# HSH: A Simple Shell in Rust
---
# ... (contents of README.md) ...

hsh > cd src
hsh > ls
main.rs
hsh > clear
# (the terminal screen will clear)
hsh > exit
```

-----

## 📋 TODO

HSH is an ongoing project, and there are many features planned for future development:

  * **External Command Execution**: Allow running commands that are not built-in (e.g., `git`, `python`, `code`). This involves searching the `PATH` environment variable and spawning child processes.
  * **Command History**: Implement an in-memory history of executed commands that can be navigated with arrow keys.
  * **Tab Completion**: Enable auto-completion for commands and file paths.
  * **Piping (`|`)**: Support redirecting the output of one command as the input to another.
  * **Input/Output Redirection (`>`, `>>`, `<`)**: Allow directing command output to files or reading input from files.
  * **Background Processes (`&`)**: Run commands in the background without blocking the shell.
  * **More Built-in Commands**:
      * `pwd`: Print current working directory.
      * `mkdir`: Create new directories.
      * `rm`: Remove files or directories.
      * `cp`: Copy files or directories.
      * `mv`: Move/rename files or directories.
      * `echo`: Print arguments to stdout.
  * **Environment Variables**: Support setting, getting, and listing environment variables.
  * **Error Handling Improvements**: More robust and user-friendly error messages for various scenarios.
  * **Prompt Customization**: Allow users to customize the shell prompt.

-----

## 🤝 Contributing

Contributions are always welcome\! If you have suggestions for new features, improvements, or bug fixes, please feel free to:

1.  **Open an issue**: Describe the bug or suggest an enhancement.
2.  **Submit a pull request**: Fork the repository, make your changes, and then submit a pull request.
