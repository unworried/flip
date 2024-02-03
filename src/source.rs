pub struct Source {
    text: String,
}

impl Source {
    pub fn new(text: String) -> Source {
        Source { text }
    }

    pub fn line_index(&self, index: usize) -> usize {
<<<<<<< HEAD
        self.text[..index].lines().count() - 1
=======
        if index == 0 {
            return 0;
        }

        (index + self.text[..index].lines().count() - 1) % self.text.lines().count()
        //self.text[..index].lines().count() - 1
>>>>>>> b9abefc (fixes)
    }

    pub fn line(&self, index: usize) -> &str {
        self.text.lines().nth(index).unwrap() // handle correctly
    }

    pub fn line_start(&self, index: usize) -> usize {
        self.text
            .lines()
            .take(index)
            .map(|line| line.len() + 1)
            .sum()
    }
}
