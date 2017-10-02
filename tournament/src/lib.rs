use std::collections::HashMap;

const HOME: usize = 0;
const AWAY: usize = 1;
const RESULT: usize = 2;

const WIN: usize = 0;
const LOSS: usize = 1;
const TIE: usize = 2;

pub fn tally(input: &str) -> String {
    let mut total: HashMap<String, (usize, usize, usize)> = HashMap::new();
    for line in input.split("\n") {
        if line == "" { continue }
        let fields = line.split(';').collect::<Vec<&str>>();
        match fields[RESULT] {
            "win" => {
                add_result(WIN, fields[HOME], &mut total);
                add_result(LOSS, fields[AWAY], &mut total);
            },
            "loss" => {
                add_result(WIN, fields[AWAY], &mut total);
                add_result(LOSS, fields[HOME], &mut total);
            },
            "draw" => {
                add_result(TIE, fields[AWAY], &mut total);
                add_result(TIE, fields[HOME], &mut total);
            },
            _ => (),
        };
    }
    let mut output =  vec!(
        "Team                           | MP |  W |  D |  L |  P".to_string());
    let mut teams = total.iter().collect::<Vec<_>>();
    teams.sort_by_key(|&(name, &(w, _, t))| (std::usize::MAX - w * 3 - t, name));
    for &(name, &(win,loss, tie)) in teams.iter() {
        output.push(format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                            name, win+loss+tie, win, tie, loss, 3 * win + tie));
    }
    output.join("\n")
}

fn add_result(result: usize, team: &str, 
              scores: &mut HashMap<String, (usize, usize, usize)>) {
    if !scores.contains_key(team) { scores.insert(team.to_string(), (0,0,0)); } 
    let mut record = scores.get_mut(team).unwrap();
    match result {
        WIN => record.0 += 1,
        LOSS => record.1 += 1,
        TIE => record.2 += 1,
        _ => panic!("Nope"),
    }
}

