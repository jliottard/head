# head
My implementation of the Unix utility head, that prints the contents of a file. 

# Features
Support for:
- standard input to print
- n argument to display an limited number of lines
- c argument to display an limited number of bytes
- multiple files to print

# Usages
Print the standard input:
```bash
cargo r --release
```
Print the multiple files:
```bash
cargo r --release -- text.txt text2.text
```

Print the N first lines of a file:
```bash
cargo r --release -- text.txt -n <N>
```

Print the N first bytes of a file:
```bash
cargo r --release -- text.txt -c <N>
```

