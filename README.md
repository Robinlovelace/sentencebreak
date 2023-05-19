# sentencebreak

Command line tool to break lines at sentences.
It takes an input file, e.g. README.md, and outputs a new file that has the same contents but with line breaks added after each sentence:

```bash
cargo run README.md -o README_sentencebreaks.md
```

You can set the output to be the same file as the input by omitting the `-o` flag:

```bash
cargo run README.md
```