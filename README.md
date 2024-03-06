# Yet Another Word Counter

Word counter built in multiple languages following the directives at: https://codingchallenges.fyi/challenges/challenge-git/

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
