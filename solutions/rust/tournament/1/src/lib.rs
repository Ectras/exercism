use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn opposite(&self) -> Outcome {
        match self {
            Outcome::Win => Outcome::Loss,
            Outcome::Loss => Outcome::Win,
            Outcome::Draw => Outcome::Draw,
        }
    }
}

impl TryFrom<&str> for Outcome {
    type Error = ();

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "win" => Ok(Outcome::Win),
            "loss" => Ok(Outcome::Loss),
            "draw" => Ok(Outcome::Draw),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Row {
    pub played: u16,
    pub wins: u16,
    pub draws: u16,
    pub loses: u16,
}

impl Row {
    pub fn points(&self) -> u16 {
        self.wins * 3 + self.draws
    }

    pub fn add_result(&mut self, result: Outcome) {
        match result {
            Outcome::Win => self.wins += 1,
            Outcome::Loss => self.loses += 1,
            Outcome::Draw => self.draws += 1,
        };
        self.played += 1;
    }
}

fn parse_games(match_results: &str) -> HashMap<&str, Row> {
    let mut table: HashMap<&str, Row> = HashMap::new();
    for line in match_results.split_terminator('\n') {
        // Parse a line
        let tokens = line.split_terminator(';').collect::<Vec<_>>();
        let [team_a, team_b, result] = tokens[..] else {
            panic!()
        };
        let result = Outcome::try_from(result).unwrap();

        // Add the result to the table
        table.entry(team_a).or_default().add_result(result);
        table
            .entry(team_b)
            .or_default()
            .add_result(result.opposite());
    }
    table
}

pub fn tally(match_results: &str) -> String {
    let table = parse_games(match_results);

    // Sort by points and then team names
    let mut sorted = table.into_iter().collect::<Vec<_>>();
    sorted.sort_unstable_by(|(team_a, row_a), (team_b, row_b)| {
        row_b.points().cmp(&row_a.points()).then(team_a.cmp(team_b))
    });

    // Format table header
    let mut out = Vec::with_capacity(sorted.len());
    out.push(format!(
        "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
        "Team", "MP", "W", "D", "L", "P"
    ));

    // Format rest of table
    for (team, row) in sorted {
        out.push(format!(
            "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
            team,
            row.played,
            row.wins,
            row.draws,
            row.loses,
            row.points()
        ));
    }
    out.join("\n")
}
