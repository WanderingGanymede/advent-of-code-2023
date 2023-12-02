fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let valid_games_ids: Vec<i32> = input
        .lines()
        .map(|line| {
            let mut game = line.split(':');
            let game_id = game.next().unwrap().trim().split(' ').nth(1).unwrap();
            let game_desc = game.next().unwrap();
            (game_id, game_desc)
        })
        .map(|(id, desc)| {
            let game_valid = desc
                .split(';')
                .map(|draw| {
                    draw.split(',')
                        .map(|draw_elem| {
                            let mut s = draw_elem.trim().split(' ');

                            let count: i16 = s.next().unwrap().parse().unwrap();
                            let color = s.next().unwrap();
                            let valid: bool;
                            match color {
                                "green" => valid = count <= 13,
                                "red" => valid = count <= 12,
                                "blue" => valid = count <= 14,
                                _ => valid = true,
                            }
                            valid
                        })
                        .fold(true, |acc, x| acc && x)
                })
                .fold(true, |acc, x| acc && x);
            let int_id: i32 = id.parse().unwrap();
            (int_id, game_valid)
        })
        .filter_map(|(id, valid)| match valid {
            true => Some(id),
            false => None,
        })
        .collect();

    valid_games_ids.iter().fold(0, |acc, i| acc + i).to_string()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_part1() {
        let result = part1(
            " Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
