// (time between susie locking in and ralsei locking in)
// 1: 5f 2: 14f 3: 26f 4: 38f
// time lost
// 2: 9f 3: 21f 4: 35f
// (from my own frame by frame check. if there's randomness involved in the amount of timeloss this doesn't account for it)

const fn get_answer_timeloss(answer: u8) -> usize {
    match answer {
        2 => 9,
        3 => 21,
        4 => 35,
        _ => 0,
    }
}

/// A sorted array of every amount of timeloss in a 3 turn board 3 to compare against
static BOARD3_TIMELOSS: std::sync::LazyLock<Vec<usize>> = std::sync::LazyLock::new(|| {
    let mut buf = vec![0;64];
    for answer1 in 1..=4 {
        for answer2 in 1..=4 {
            for answer3 in 1..=4 {
                // I hope I did this right
                let idx = ((answer3 - 1) + (answer2 - 1) * 4 + (answer1 - 1) * 16) as usize;
                let timeloss = get_answer_timeloss(answer1) + get_answer_timeloss(answer2) + get_answer_timeloss(answer3);
                buf[idx] = timeloss;
            }
        }
    }
    buf.sort();
    buf
});

/// A sorted array of every amount of timeloss 2 turns into a board 3 to compare against
static BOARD3_TIMELOSS2: std::sync::LazyLock<Vec<usize>> = std::sync::LazyLock::new(|| {
    let mut buf = vec![0;16];
    for answer1 in 1..=4 {
        for answer2 in 1..=4 {
            let idx = ((answer2 - 1) + (answer1 - 1) * 4) as usize;
            let timeloss = get_answer_timeloss(answer1) + get_answer_timeloss(answer2);
            buf[idx] = timeloss;
        }
    }
    buf.sort();
    buf
});

/// A recorded Board 3
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub struct Board3 {
    /// All of Ralsei's answers on a given board
    pub answers: [u8;3],
    /// The recorded time that the board was recorded
    timestamp: std::time::SystemTime,
}

impl Default for Board3 {
    fn default() -> Self {
        Self {
            answers: [0;3],
            timestamp: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}

impl Board3 {
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates [Self::timestamp] with the current system time
    pub fn update_time(&mut self) {
        self.timestamp = std::time::SystemTime::now();
    }

    /// Returns how many answers have been entered on the board
    pub fn get_answer_num(&self) -> usize {
        for (idx, answer) in self.answers.iter().enumerate() {
            if *answer == 0 {
                return idx;
            }
        }
        return 3;
    }

    /// Returns the total timeloss in frames on the current board
    pub fn get_timeloss(&self) -> usize {
        let mut timeloss = 0;
        for answer in self.answers {
            timeloss += get_answer_timeloss(answer);
        }
        timeloss
    }
    
    pub fn clear(&mut self) {
        self.answers.fill(0);
    }

    pub fn get_percent_chance(&self) -> f64 {
        let array;
        match self.get_answer_num() {
            1 => {
                return self.answers[0] as f64 / 0.04;  
            } ,
            2 => {
                array = &BOARD3_TIMELOSS2;
            },
            3 => {
                array = &BOARD3_TIMELOSS;
            },
            _ => return 0.0,
        }
        // The idea here is to go through every member of the arrays until we hit the last option that's equal to the board's timeloss
        // These arrays are fully comprehensive, so the last one should be able to give us an exact chance of this ralsei board
        let mut latest: usize = 0;
        for (i, val) in array.iter().enumerate() {
            if val == &self.get_timeloss() {
                latest = i + 1;
            }
        }
        return latest as f64 / array.len() as f64 * 100.0;
    }

    /// panics if idx is more than 2
    fn get_answer(&self, idx: usize) -> Option<u8> {
        let answer = self.answers[idx];
        if answer == 0 {
            return None;
        } else {
            return Some(answer);
        }
    }

    pub fn get_answer1(&self) -> Option<u8> {
        self.get_answer(0)
    }
    pub fn get_answer2(&self) -> Option<u8> {
        self.get_answer(1)
    }
    pub fn get_answer3(&self) -> Option<u8> {
        self.get_answer(2)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tracker {
    #[serde(skip)]
    current_board: Board3,
    saved_boards: Vec<Board3>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            current_board: Board3::new(),
            saved_boards: Vec::new(),
        }
    }

    pub fn load() -> Self {
        let x;
        if let Some(json) = shared::file::read_file(crate::CONF_FILE) {
            x = serde_json::from_str(&json).unwrap_or(Self::new());
        } else {
            x = Self::new();
        }
        x
    }

    pub fn update(&mut self, answer: u8) {
        // can safely act as an index if < 3
        let answer_number = self.current_board.get_answer_num();
        if answer_number < 3 {
            self.current_board.answers[answer_number] = answer;
            if answer_number == 0 {
                self.current_board.update_time();
            } else if answer_number == 2 {
                self.save_board();
            }
        } else {
            self.current_board.clear();
        }
    }

    pub fn get_current_timeloss(&self) -> usize {
        self.current_board.get_timeloss()
    }

    pub fn get_total_timeloss(&self) -> usize {
        let mut timeloss = self.saved_boards.iter().map(|board| board.get_timeloss()).sum();

        if self.current_board.get_answer_num() < 3 {
            timeloss += self.current_board.get_timeloss();
        }

        timeloss
    }

    pub fn save_board(&mut self) {
        self.saved_boards.push(self.current_board);
        
        // Using the unwrap so I know something terrible has gone wrong earlier rather than later here
        shared::file::save_file(crate::CONF_FILE, serde_json::to_string_pretty(self).unwrap());
    }

    /// Gets the overall likelyhood of getting the current board
    pub fn get_percent_chance(&self) -> f64 {
        self.current_board.get_percent_chance()
    }

    /// Gets the current board's first Ralsei answer
    pub fn get_current_answer1(&self) -> Option<u8> {
        self.current_board.get_answer1()
    }

    /// Gets the current board's second Ralsei answer
    pub fn get_current_answer2(&self) -> Option<u8> {
        self.current_board.get_answer2()
    }

    /// Gets the current board's third Ralsei answer
    pub fn get_current_answer3(&self) -> Option<u8> {
        self.current_board.get_answer3()
    }

    /// Saves temp files containing the visually required information for OBS
    pub fn save_obs_files(&self) {
        shared::file::write_temp_file("board_timeloss", format!("{}f", self.get_current_timeloss().to_string()));
        shared::file::write_temp_file("total_timeloss", format!("{}f", self.get_total_timeloss().to_string()));
        shared::file::write_temp_file("ralsei1", self.get_current_answer1().and_then(|x| Some( x.to_string() ) ).unwrap_or(String::from(" ")));
        shared::file::write_temp_file("ralsei2", self.get_current_answer2().and_then(|x| Some( x.to_string() ) ).unwrap_or(String::from(" ")));
        shared::file::write_temp_file("ralsei3", self.get_current_answer3().and_then(|x| Some( x.to_string() ) ).unwrap_or(String::from(" ")));
        shared::file::write_temp_file("board_chance", format!("{:.2}%", self.get_percent_chance()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn individual_board_timeloss() {
        let mut tracker = Tracker::new();
        
        assert_eq!(tracker.get_current_timeloss(), 0);

        // perfect board
        tracker.update(1);
        tracker.update(1);
        tracker.update(1);
        assert_eq!(tracker.get_current_timeloss(), 0);
        // don't want it doing file io in tests
        tracker = Tracker::new();

        tracker.update(1);
        tracker.update(4);
        tracker.update(1);

        assert_eq!(tracker.get_current_timeloss(), 35);

        tracker = Tracker::new();

        tracker.update(4);
        tracker.update(4);
        tracker.update(4);
        assert_eq!(tracker.get_current_timeloss(), 105);
    }

    #[test]
    fn get_board_percent() {
        let mut tracker = Tracker::new();

        assert_eq!(tracker.get_percent_chance(), 0.0);
        tracker.update(1);
        assert_eq!(tracker.get_percent_chance(), 25.0);
        tracker.update(1);
        assert_eq!(tracker.get_percent_chance(), 1.0 / 0.16);
        tracker.update(1);
        assert_eq!(tracker.get_percent_chance(), 1.0 / 0.64);

        tracker = Tracker::new();
        tracker.update(4);
        assert_eq!(tracker.get_percent_chance(), 100.0);
        tracker.update(4);
        assert_eq!(tracker.get_percent_chance(), 100.0);
        tracker.update(4);
        assert_eq!(tracker.get_percent_chance(), 100.0);
    }
}