use std::collections::HashMap;

#[derive(Debug)]
struct OverallResult {
    win: u32,
    lose: u32,
    draw: u32,
    match_played: u32,
    point: u32,
}

pub fn tally(match_results: &str) -> String {
    let lines = match_results.lines();
    let mut teams: HashMap<&str, OverallResult> = HashMap::new();

    for line in lines {
        let line_tokens: Vec<&str> = line.split(';').collect();
        let match_result = line_tokens[2];
        let first_team = line_tokens[0];
        let second_team = line_tokens[1];
        let mut first_team_this_line_result = OverallResult {
            win: 0,
            lose: 0,
            draw: 0,
            point: 0,
            match_played: 0,
        };

        let mut second_team_this_line_result = OverallResult {
            win: 0,
            lose: 0,
            draw: 0,
            point: 0,
            match_played: 0,
        };

        if match_result == "win" {
            first_team_this_line_result.win = 1;
            first_team_this_line_result.point = 3;
            second_team_this_line_result.lose = 1;
            second_team_this_line_result.point = 0;
        } else if match_result == "loss" {
            second_team_this_line_result.win = 1;
            second_team_this_line_result.point = 3;
            first_team_this_line_result.lose = 1;
            first_team_this_line_result.point = 0;
        } else {
            second_team_this_line_result.draw = 1;
            second_team_this_line_result.point = 1;
            first_team_this_line_result.draw = 1;
            first_team_this_line_result.point = 1;
        }
        teams
            .entry(first_team)
            .and_modify(|team| {
                *team = OverallResult {
                    match_played: team.match_played + 1,
                    draw: team.draw + first_team_this_line_result.draw,
                    lose: team.lose + first_team_this_line_result.lose,
                    win: team.win + first_team_this_line_result.win,
                    point: team.point + first_team_this_line_result.point,
                }
            })
            .or_insert(OverallResult {
                match_played: 1,
                draw: first_team_this_line_result.draw,
                lose: first_team_this_line_result.lose,
                win: first_team_this_line_result.win,
                point: first_team_this_line_result.point,
            });

        teams
            .entry(second_team)
            .and_modify(|team| {
                *team = OverallResult {
                    match_played: team.match_played + 1,
                    draw: team.draw + second_team_this_line_result.draw,
                    lose: team.lose + second_team_this_line_result.lose,
                    win: team.win + second_team_this_line_result.win,
                    point: team.point + second_team_this_line_result.point,
                }
            })
            .or_insert(OverallResult {
                match_played: 1,
                draw: second_team_this_line_result.draw,
                lose: second_team_this_line_result.lose,
                win: second_team_this_line_result.win,
                point: second_team_this_line_result.point,
            });
    }

    let mut teams: Vec<(&&str, &OverallResult)> = teams.iter().collect();
    teams.sort_by(|(team_1_name, _), (team_2_name, _)| {
        return team_1_name.to_lowercase().cmp(&team_2_name.to_lowercase());
    });
    teams.sort_by(|(_, team_1), (_, team_2)| return team_2.point.cmp(&team_1.point));

    let mut return_string =
        String::from("Team                           | MP |  W |  D |  L |  P\n");

    for (team_name, team_result) in teams {
        let forff = format!(
            "{:31}|{:3} |{:3} |{:3} |{:3} |{:3}\n",
            team_name,
            team_result.match_played,
            team_result.win,
            team_result.draw,
            team_result.lose,
            team_result.point
        );
        return_string = [return_string, forff].concat();
    }

    // remove \n character from last line
    return_string.pop();
    return_string
}
