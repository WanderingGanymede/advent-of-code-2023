fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum: i32 = 0;
    for line in lines {
        let v: Vec<&str> = line.matches(char::is_numeric).collect();
        let mut c = v[0].to_owned();
        c.push_str(v[v.len() - 1]);
        let coordint: i32 = c.parse().unwrap();
        sum += coordint;
        println!("{}", c);
    }
    sum.to_string()
}
