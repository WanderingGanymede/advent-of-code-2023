fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum: i32 = 0;
    for line in lines {
        let mut newline: String = line.to_owned();
        newline = newline
            .replace("one", "on1e")
            .replace("two", "tw2o")
            .replace("three", "thr3e")
            .replace("four", "fo4r")
            .replace("five", "fi5e")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "ni9e");
        let v: Vec<&str> = newline.matches(char::is_numeric).collect();
        let mut c = v[0].to_owned();
        c.push_str(v[v.len() - 1]);
        println!("{newline} , {line} , {c}");
        let coordint: i32 = c.parse().unwrap();
        sum += coordint;
    }
    sum.to_string()
}
