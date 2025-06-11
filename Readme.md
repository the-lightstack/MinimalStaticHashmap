# Tests using word2vec in Rust


Requires a dir called "English" with a word2vec file

## Goals
Smallest possible hash-map like lookup table, that can then directly be put into a file (as binary data) and be vmapped into memory  for **much** faster loading times than parsing.