# Yet Another Word Counter

Word counter built in multiple languages following the directives at: https://codingchallenges.fyi/challenges/challenge-wc

## Usage

```sh
./ccwc [-c|--bytes] <file_path>
```

- `-c` or `--bytes`: Optional flag to display only the number of bytes in the file. If not provided, the program displays all available information.
- `<file_path>`: Path to the file for which you want to get information.

## Example

```sh
./ccwc data/test.txt
```

This command will display the number of words, lines, and bytes in the file `data/test.txt`.

```sh
./ccwc -c data/test.txt
```

This command will display only the number of bytes in the file `data/test.txt`.

## Patterns/Concepts

### Rust

#### Pattern Matching

Ref: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#matching-literals

Rust uses this type of expression to match a given value to a specified pattern. In the snippet, different literals are used as arms to the `match` expression to define the program's output. 

- Snippet:

```rust
    fn process_file(mode: Option<&str>, file_path: &str) {
        match mode {
            "all" => print_all_info(file_path),
            "bytes" => print_bytes(file_path),
            "lines" => print_lines(file_path),
            None => print_all_info(file_path),
            _ => unreachable!(),
        }
    }
```

### C++

#### RAII (Resource Acquisition is Initialization)

Ref: https://en.cppreference.com/w/cpp/language/raii

Is a programming technique in C++ that wraps resources that need to be requested (due to their limitation) to an object that manages their life cycle.

- Snippet:

```cpp
// ifstream::close is automatically called when it's associated object
// is destroyed:. 
// Ref: https://cplusplus.com/reference/fstream/ifstream/close/
int processFile(const string& option, const string& filePath) {
    ifstream inputFile(filePath);

    if (!inputFile.is_open()) {
        cerr << "Error opening the file!" << endl;
        return 1;
    }

    try {
        if (option == "-c" || option == "--bytes") {
            printBytes(inputFile, filePath);
        } else if (option == "-l" || option == "--lines") {
            printLines(inputFile, filePath);
        } else if (option == "-a") {
            printAll(inputFile, filePath);
        } else {
            cerr << "Unkown flag " << option << endl;
        }

        return 0;
    } catch (const std::ifstream::failure& e) {
        cerr << "Error reading file " << e.what() << endl;
        return 1;
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
