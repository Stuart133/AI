use std::{
    collections::{BinaryHeap, HashMap},
    path::Path,
};

#[derive(Debug)]
pub struct Bill {
    code: String,
    description: String,
}

pub fn parse_bills(data_file: &Path) -> Vec<Bill> {
    let data = match std::fs::read_to_string(data_file) {
        Ok(str) => str,
        Err(_) => return vec![],
    };

    data.lines()
        .skip(1)
        .map(|l| Bill {
            code: l.split(",").skip(3).take(1).collect(),
            description: l.split(",").skip(6).take(1).collect(),
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Legislator {
    name: String,
    party: Party,
    votes: Vec<Vote>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Party {
    Democrat,
    Republican,
    Independent,
}

impl Party {
    fn new(code: &str) -> Self {
        match code {
            "100" => Party::Democrat,
            "200" => Party::Republican,
            _ => Party::Independent,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

impl Into<f64> for Vote {
    fn into(self) -> f64 {
        match self {
            Vote::Yea => 1.0,
            Vote::Nay => -1.0,
            Vote::Absent => 0.0,
        }
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

pub fn evaluate(
    distance: fn(&Legislator, &Legislator) -> i64,
    k: usize,
    group1: &Vec<Legislator>,
    group2: &Vec<Legislator>,
) -> usize {
    let mut score = 0;

    for (test, train) in vec![(group1, group2), (group2, group1)] {
        let classifier = NearestNeighboursClassifier::new(distance, k, train);
        for leg in test {
            let actual = leg.party;
            let predicted = classifier.classify(leg);
            if actual == predicted {
                score += 1;
                println!("{}: {:?} CORRECT", leg.name, predicted);
            } else {
                println!("{}: {:?} WRONG. ACTUAL {:?}", leg.name, predicted, actual);
            }
        }
    }

    score
}

pub fn crosscheck(group: Vec<Legislator>) -> (Vec<Legislator>, Vec<Legislator>) {
    let mut g1 = vec![];
    let mut g2 = vec![];

    for (i, leg) in group.into_iter().enumerate() {
        if i % 4 == 0 || i % 4 == 3 {
            g1.push(leg);
        } else {
            g2.push(leg);
        }
    }

    (g1, g2)
}

pub fn hamming_distance(left: &Legislator, right: &Legislator) -> i64 {
    let mut distance = 0;

    for i in 0..left.votes.len() {
        if left.votes[i] != right.votes[i] {
            distance += 1;
        }
    }

    distance
}

pub fn euclidean_distance(left: &Legislator, right: &Legislator) -> i64 {
    let mut total = 0.0;

    for i in 0..left.votes.len() {
        let l: f64 = left.votes[i].into();
        let r: f64 = right.votes[i].into();

        total += (l - r).powi(2);
    }

    total.sqrt() as i64
}

pub struct NearestNeighboursClassifier<'a> {
    distance: fn(&Legislator, &Legislator) -> i64,
    k: usize,
    data: &'a Vec<Legislator>,
}

#[derive(PartialEq, Eq, Hash)]
struct Distance<'a> {
    distance: i64,
    data: &'a Legislator,
}

impl<'a> Ord for Distance<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the order as we are using this in a min heap
        other.distance.cmp(&self.distance)
    }
}

impl<'a> PartialOrd for Distance<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> NearestNeighboursClassifier<'a> {
    pub fn new(
        distance: fn(&Legislator, &Legislator) -> i64,
        k: usize,
        training_data: &'a Vec<Legislator>,
    ) -> Self {
        NearestNeighboursClassifier {
            distance,
            k,
            data: training_data,
        }
    }

    pub fn classify(&self, query: &Legislator) -> Party {
        let distances: Vec<Distance> = self
            .data
            .iter()
            .map(|l| Distance {
                distance: (self.distance)(l, query),
                data: l,
            })
            .collect::<BinaryHeap<Distance>>()
            .into_iter()
            .take(self.k)
            .collect();

        let mut map = HashMap::new();
        for i in distances.iter() {
            match map.insert(i, 1_i64) {
                Some(v) => {
                    *map.get_mut(i).unwrap() += v;
                }
                None => {}
            }
        }

        let mut best_count = 0;
        let mut best_party = Party::Democrat;
        for (k, v) in map {
            if v > best_count {
                best_count = 0;
                best_party = k.data.party;
            }
        }

        return best_party;
    }
}

pub struct DisorderTree<'a> {
    yes: &'a DisorderTree<'a>,
    no: &'a DisorderTree<'a>,
    vote_index: usize,
}

#[cfg(test)]
mod tests {
    use super::{euclidean_distance, Legislator, Party, Vote};

    #[test]
    pub fn euclidean() {
        let l1 = Legislator {
            name: "".to_string(),
            party: Party::Democrat,
            votes: vec![Vote::Yea, Vote::Yea],
        };
        let l2 = Legislator {
            name: "".to_string(),
            party: Party::Democrat,
            votes: vec![Vote::Yea, Vote::Nay],
        };

        let distance = euclidean_distance(&l1, &l2);

        assert_eq!(2, distance);
    }
}
