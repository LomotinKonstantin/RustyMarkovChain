# RustyMarkovChain
A simple Markov chain for word generation.

Fitting:
```bash
$ main.exe fit one_word_per_line_list.txt
```

After fitting, the app creates a `weigths.bin` file with the trained chain.

Generating a word with minimal length 10 (with a `weigths.bin` file in the same directory):
```bash
$ main.exe gen 10
```
