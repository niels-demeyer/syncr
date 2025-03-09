# Directory Sync

This project provides a command-line tool to synchronize two directories efficiently by comparing their metadata. It checks for differences in file existence, modification times, and sizes to determine which files need to be copied or updated.

## Features

- Compare two directories and identify differences in files.
- Sync files based on modification time and size.
- Efficiently update files in both directories.

## Getting Started

### Prerequisites

Make sure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

### Building the Project

To build the project, navigate to the project directory and run:

```
cargo build
```

### Running the Program

You can run the program by providing two directory paths as command-line arguments. For example:

```
cargo run -- <source_directory> <destination_directory>
```

Replace `<source_directory>` and `<destination_directory>` with the paths of the directories you want to sync.

### Usage Example

To sync the directories `dir1` and `dir2`, you would run:

```
cargo run -- dir1 dir2
```

This command will compare the contents of `dir1` and `dir2` and synchronize them based on the defined criteria.

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.