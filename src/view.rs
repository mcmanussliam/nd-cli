pub struct View {
    use_color: bool,
}

impl View {
    pub fn new(use_color: bool) -> Self {
        Self { use_color }
    }

    pub fn clear(&self) {
        eprintln!("\x1b[2J\x1b[H");
    }

    pub fn muted(&self, message: &str) {
        if self.use_color {
            eprintln!("\x1b[2m{}\x1b[0m", message);
        } else {
            eprintln!("{message}");
        }
    }

    pub fn prompt(&self, message: &str) {
        if self.use_color {
            eprint!("\x1b[36m{}\x1b[0m", message);
        } else {
            eprint!("{message}");
        }
    }

    pub fn row(&self, index: usize, label: &str) {
        if self.use_color {
            eprintln!("\x1b[38;5;245m┃\x1b[0m {}. {}", index, label);
        } else {
            eprintln!("| {}. {}", index, label);
        }
    }

    pub fn warn(&self, message: &str) {
        if self.use_color {
            eprintln!("\x1b[33m{}\x1b[0m", message);
        } else {
            eprintln!("{message}");
        }
    }
}
