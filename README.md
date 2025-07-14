**Robust Filesystem Utility CLI** 🛠️

This project presents a command-line interface (CLI) tool crafted in Rust, designed to streamline common filesystem operations. It empowers developers and users alike to effortlessly create files and folders, and append content to existing files, directly from their terminal. This utility is built with robustness and user-friendliness in mind, aiming to simplify daily development workflows.

### Installation

Getting this powerful utility up and running on your local machine is straightforward!

1.  **Clone the Repository**:
    Begin by cloning the project to your local system using Git:
    ```bash
    git clone https://github.com/your-username/filesystem-utility.git
    ```
    (Replace `https://github.com/your-username/filesystem-utility.git` with the actual repository URL.)

2.  **Navigate to the Project Directory**:
    Change into the newly cloned directory:
    ```bash
    cd filesystem-utility
    ```

3.  **Build the Project**:
    Ensure you have Rust and Cargo installed. Then, compile the project in release mode for optimized performance:
    ```bash
    cargo build --release
    ```

4.  **Run the Utility (Optional: Add to PATH)**:
    You can run the utility directly using Cargo:
    ```bash
    cargo run -- <command> [options]
    ```
   

### Usage

The `filesystem-utility` offers two primary subcommands: `create` and `write`.

#### 📁 Create Files and Folders

The `create` command allows you to generate new folders or files with various options.

*   **Creating a single folder in the current directory**:
    ```bash
    cargo run -- create --folder my_new_folder
    ```
    This will create `my_new_folder` in your current working directory.

*   **Creating a file in the current directory**:
    ```bash
    cargo run -- create --file my_document.txt
    ```
    This will create `my_document.txt` in your current working directory. Remember to include a file extension!

*   **Creating a new folder inside an existing folder**:
    ```bash
    cargo run -- create --folder nested_folder --in parent_folder
    ```
    This command will search for `parent_folder` in the filesystem and, if found, create `nested_folder` inside it.

*   **Creating a new file inside an existing folder**:
    ```bash
    cargo run -- create --file README.md --in docs
    ```
    This will locate `docs` and create `README.md` within it.

*   **Handling multiple folder creation (note on current implementation)**:
    The tool provides a `--folders` argument for creating multiple folders. While the command structure is in place, the core logic for bulk creation using this specific argument is currently under development in the provided code, and might not behave as expected for more than one folder.
    ```bash
    # Example command (behavior might vary based on ongoing development):
    cargo run -- create --folders project_alpha project_beta
    ```

#### 📝 Write Content to Files

The `write` command enables you to append content to an existing file.

*   **Writing content to a file in the current directory**:
    ```bash
    cargo run -- write --file log.txt --content "New log entry" "Another line"
    ```
    This appends "New log entry Another line" followed by a newline to `log.txt`.

*   **Writing content to a file located within a specific folder**:
    ```bash
    cargo run -- write --file config.json --in settings_dir --content "{ \"key\": \"value\" }"
    ```
    This command will search for `settings_dir`, then for `config.json` inside it, and append the provided JSON string. Note that the `--in .` option can be used to explicitly search from the current working directory's root if needed.

### Features

Here are some of the key capabilities this utility brings to the table:

*   📂 **Intuitive CLI Interface**: A well-structured command-line interface powered by `clap` for easy interaction.
*   ➕ **Folder Creation**: Quickly generate new directories, either in the current working directory or nested within specified parent folders.
*   📄 **File Creation**: Create new files, ensuring proper extensions are used, within the current directory or a designated parent folder.
*   ✍️ **Content Appending**: Seamlessly add new lines or content snippets to existing files.
*   🔍 **Recursive Search**: Ability to locate target folders and files within nested directory structures for operations like creating or writing inside them.
*   ⚠️ **Robust Error Handling**: Provides clear feedback for common issues like existing files/folders, invalid names, or missing targets, enhancing reliability.

### Technologies Used

This project is built using modern Rust ecosystem tools to ensure performance and reliability.

| Category              | Technology   | Description                                                    |
| :-------------------- | :----------- | :------------------------------------------------------------- |
| **Language**          | Rust         | A systems programming language focused on safety, performance, and concurrency. |
| **CLI Framework**     | `clap`       | A powerful, fast, and easy-to-use argument parser for Rust CLIs. |
| **Filesystem Traversal** | `walkdir`    | A crate for recursively walking a directory tree.               |
| **File Operations**   | `std::fs`    | Rust's standard library for filesystem operations.             |
| **I/O Operations**    | `std::io`    | Rust's standard library for input and output.                  |

### Contributing

Your contributions are warmly welcomed! If you're passionate about improving this utility, here's how you can help:

*   **Reporting Bugs**: 🐛 If you find any issues, please open a new issue on the GitHub repository and provide detailed steps to reproduce the problem.
*   **Suggesting Features**: ✨ Have an idea for a new feature or an enhancement? Feel free to open an issue to discuss it.
*   **Submitting Pull Requests**: 🚀 Fork the repository, make your changes, and submit a pull request. Please ensure your code adheres to the existing style and includes relevant tests if applicable.

### License

This project is open source and available under the MIT License.

### Author Info

Connect with the developer behind this project:

**[chukwu obed]**
*   GitHub: [@obed-smart](https://github.com/obed-smart)
*   LinkedIn: [@obed-smartgit](https://www.linkedin.com/in/obed-smart/)
*   Twitter: [@eberechukwuobed](https://x.com/eberechukwuobed?s=21)


### Badges

[![Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/Type-CLI-blue)](https://crates.io/crates/clap)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)