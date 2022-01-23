use progressive::progress;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

type Word = [u8; 5];
type Tiles = [Tile; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
  No,
  Contains,
  Yes,
}

fn get_tiles(guess: &Word, answer: &Word) -> Tiles {
  let mut ret = [Tile::No; 5];
  for (idx, tile) in ret.iter_mut().enumerate() {
    let g = guess[idx];
    if g == answer[idx] {
      *tile = Tile::Yes;
    } else if answer.contains(&g) {
      *tile = Tile::Contains;
    }
  }
  ret
}

fn might_be(guess: &Word, tiles: &Tiles, word: &Word) -> bool {
  tiles.iter().enumerate().all(|(idx, tile)| {
    let g = guess[idx];
    match tile {
      Tile::No => !word.contains(&g),
      Tile::Contains => word.contains(&g),
      Tile::Yes => word[idx] == g,
    }
  })
}

fn get_words() -> Vec<Word> {
  include_str!("words_alpha.txt")
    .lines()
    .filter_map(|line| line.as_bytes().try_into().ok())
    .collect()
}

fn main() {
  let words = get_words();
  let iter = words.iter().map(|guess| {
    let total: usize = words
      .par_iter()
      .map(|answer| {
        let tiles = get_tiles(guess, answer);
        words
          .iter()
          .filter(|&word| might_be(guess, &tiles, word))
          .count()
      })
      .sum();
    (total, guess)
  });
  let (_, best) = progress(iter).min().unwrap();
  let best = std::str::from_utf8(best).unwrap();
  println!("best: {}", best);
}
