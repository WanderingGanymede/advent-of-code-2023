use std::cmp;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let power_sum: i32 = input
        .lines()
        .map(|line| {
            let mut game = line.split(':');

            let game_desc = game.nth(1).unwrap();
            game_desc
        })
        .map(|desc| {
            let min_cubes = desc
                .split(';')
                .map(|draw| {
                    draw.split(',')
                        .map(|draw_elem| {
                            let mut s = draw_elem.trim().split(' ');
                            let count: i32 = s.next().unwrap().parse().unwrap();
                            let color = s.next().unwrap();
                            (color, count)
                        })
                        .fold((0, 0, 0), |acc, (color, count)| {
                            let mut res: (i32, i32, i32) = acc;
                            match color {
                                "red" => res.0 = count,
                                "green" => res.1 = count,
                                "blue" => res.2 = count,
                                _ => (),
                            };
                            res
                        })
                })
                .fold((0, 0, 0), |acc, x| {
                    (
                        cmp::max(acc.0, x.0),
                        cmp::max(acc.1, x.1),
                        cmp::max(acc.2, x.2),
                    )
                });

            min_cubes
        })
        .fold(0, |acc, x: (i32, i32, i32)| acc + (x.0 * x.1 * x.2));

    power_sum.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_part2() {
        let result = part1(
            " Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
