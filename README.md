# RustyMarkovChain
A simple Markov chain for word generation.

Fitting:
```bash
$ markov_chain.exe fit -t one_word_per_line_list.txt -w weights.bin
```

After fitting, a `weigths.bin` file with the trained chain will appear.

Generating 10 words (assuming the `weigths.bin` file is in the same directory):
```bash
$ markov_chain.exe generate -w weights.bin -n 10
```
