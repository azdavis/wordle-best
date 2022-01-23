# wordle-best

An attempt to find the best starting [Wordle][1] guess.

## The answer

TODO (it's running right now LMAO)

## Methodology

A guess G is "better" than a guess G' if, given all possible answer words A,
there are fewer words that might be the answer given the information gained from
the guess G on the answer word A than G'.

So, we find the best guess by naively O(n^3) looping over all guess words and
all answer words, then, for each pair, finding the number of words that might
possibly be the answer given this guess-answer pair.

Then we sum over all the the answer words for each guess and return the answer
word with the minimum overall sum.

## Running the code

Clone the repo and download a dictionary of words (one word per line) to
`src/words_alpha.txt`. I got one from [here][2]. (I ignored it in the git repo
since the file size is large.) The program automatically filters for length-5
words.

On my machine it runs at about 1 iteration per second and there are 15918 words,
so it'll take about 4.5 hours.

Scratch that, this is embarrassingly parallel, so I added rayon to speed it up.
With some combination of `par_iter` and regular `iter`, I'm now able to get
about 6 iterations per second on my machine. (I think I need to make the
outermost iterator non-parallel to make `progress` work.)

[1]: https://www.powerlanguage.co.uk/wordle/
[2]:
  https://github.com/dwyl/english-words/blob/22d7c41119076750a96fca2acd664ed994cc0a75/words_alpha.txt
