pub trait Media {
  fn play(&self) -> String;
}

pub struct Movie {
  pub title: String,
  pub length_seconds: usize
}
pub struct Series {
  pub title: String,
  pub episodes: usize
}

impl Media for Movie {
  fn play(&self) -> String {
    format!("Movie {} has {} seconds", self.title, self.length_seconds)
  }
}

impl Media for Series {
  fn play(&self) -> String {
    format!("Series {} has {} episodes", self.title, self.episodes)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_movie() {
    let movie: Box<dyn Media> = Box::new(Movie {
      title: String::from("Star Wars"),
      length_seconds: 121
    });
    assert_eq!(movie.play(), "Movie Star Wars has 121 seconds");
  }

  #[test]
  fn test_series() {
    let series: Box<dyn Media> = Box::new(Series {
      title: String::from("Game of Thrones"),
      episodes: 7
    });
    assert_eq!(series.play(), "Series Game of Thrones has 7 episodes");
  }
}