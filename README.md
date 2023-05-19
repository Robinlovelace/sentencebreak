# sentencebreak

Command line tool to break lines at sentences. It takes an input file, e.g. README.md, and outputs a new file that has the same contents but with line breaks added after each sentence:

```bash
sentencebreak README.md
```

The above command will overwrite the input file and output the following text to README.md:

Before:

```text
Command line tool to break lines at sentences. It takes an input file, e.g. README.md, and outputs a new file that has the same contents but with line breaks added after each sentence:
```

After:

```text
Command line tool to break lines at sentences.
It takes an input file, e.g. README.md, and outputs a new file that has the same contents but with line breaks added after each sentence:
```

You can set the output to be a different file with the `-o` flag:


```bash
sentencebreak README.md -o README_sentencebreaks.md
```

## Installation

To install the tool, run:

```bash
cargo install sentencebreak
```

To install the latest version from GitHub, run:

```bash
cargo install --git https://github.com/robinlovelace/sentencebreak
```