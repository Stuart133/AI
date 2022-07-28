use std::path::Path;

#[derive(Debug)]
pub struct Legislator {
    name: String,
    party: Party,
    votes: Vec<Vote>,
}

#[derive(Debug)]
pub enum Party {
    Democrat,
    Republican,
}

impl Party {
    fn new(code: &str) -> Self {
        println!("{}", code);
        match code {
            "100" => Party::Democrat,
            "200" => Party::Republican,
            _ => Party::Democrat,
        }
    }
}

#[derive(Debug)]
pub enum Vote {
    Yea,
    Nay,
    Absent,
}

impl Vote {
    fn new(code: char) -> Self {
        match code {
            '1' | '2' | '3' => Vote::Yea,
            '4' | '5' | '6' => Vote::Nay,
            _ => Vote::Absent,
        }
    }

    fn new_votes(votes: &str) -> Vec<Self> {
        votes.chars().map(|c| Self::new(c)).collect()
    }
}

pub fn parse(data_file: &Path) -> Vec<Legislator> {
    let data = match std::fs::read_to_string(data_file) {
        Ok(str) => str,
        Err(_) => return vec![],
    };

    data.lines()
        .map(|line| Legislator {
            name: line[25..36].to_string(),
            party: Party::new(&line[20..23]),
            votes: Vote::new_votes(&line[36..]),
        })
        .collect()
}
