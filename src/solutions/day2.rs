use std::collections::HashMap;

pub fn run() {
    let strategy = "AY
BX
CZ";

    let mut score = 0;
    let mut rounds_played = 0;

    let mut outcomes = HashMap::new();
    outcomes.insert('R', 'P');
    outcomes.insert('P', 'S');
    outcomes.insert('S', 'R');

    for line in strategy.lines() {
        let opponent = line.chars().next().unwrap();
        let selected = line.chars().nth(1).unwrap();

        // Calculate the score for this round.
        let round_score = match (opponent, selected) {
            (o, s) if o == s => 3, // Draw
            (o, s) if outcomes[&o] == s => 6, // Win
            _ => 0, // Loss
        };

        // Add the round score to the total score.
        score += round_score;

        // Increment the number of rounds played.
        rounds_played += 1;
    }

    println!("Total score: {}", score);
    println!("Rounds played: {}", rounds_played);
}
