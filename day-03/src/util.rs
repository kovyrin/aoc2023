#[derive(Debug)]
pub struct CharRow {
    row: Vec<char>,
    width: usize,
    default: char,
}

impl CharRow {
    pub fn from_str(input: &str, default: char) -> Self {
        let row = input.trim().chars().collect::<Vec<char>>();
        let width = row.len();

        Self {
            row,
            width,
            default,
        }
    }

    pub fn len(&self) -> usize {
        self.width
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.row.iter()
    }

    pub fn cell(&self, idx: i32) -> &char {
        if idx < 0 || idx as usize >= self.width {
            return &self.default;
        }
        &self.row[idx as usize]
    }
}

// A data structure representing a rectangular map where each cell is a char
// It behaves like a 2D array, but allows out of bounds access (returns the default char)
#[derive(Debug)]
pub struct CharMap {
    map: Vec<CharRow>,
    height: usize,
    default_row: CharRow,
}

impl CharMap {
    pub fn from_str(input: &str, default: char) -> Self {
        let mut map = Vec::new();
        for line in input.lines() {
            map.push(CharRow::from_str(&line, default));
        }

        let width = map[0].len();
        let height = map.len();
        let default_row = CharRow::from_str(&default.to_string().repeat(width), default);

        Self {
            map,
            height,
            default_row,
        }
    }

    pub fn lines(&self) -> impl Iterator<Item = &CharRow> {
        self.map.iter()
    }

    pub fn line(&self, idx: i32) -> &CharRow {
        if idx < 0 || idx as usize >= self.height {
            return &self.default_row;
        }
        &self.map[idx as usize]
    }

    pub fn cell(&self, x: i32, y: i32) -> &char {
        self.line(y).cell(x)
    }
}
