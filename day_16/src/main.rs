use std::ops::RangeInclusive;

type Rule = (RangeInclusive<usize>, RangeInclusive<usize>);
type Ticket = Vec<usize>;

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 28_884);
    println!("Part 1: {}", part_1);
}

fn part_1(input: &str) -> usize {
    let (rules, _my_ticket, nearby_tickets) = parse(&input);

    nearby_tickets
        .iter()
        .flatten()
        .filter(|value| {
            !rules
                .iter()
                .any(|(r1, r2)| r1.contains(value) || r2.contains(value))
        })
        .sum()
}

#[allow(dead_code)]
fn ticket_valid(rules: &[Rule], ticket: &Ticket) -> bool {
    ticket.iter().all(|value| {
        rules
            .iter()
            .any(|(r1, r2)| r1.contains(value) || r2.contains(value))
    })
}

fn parse(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut iter = input.split("\n\n");

    let rules: Vec<_> = iter
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let rules = parse_rules(&rules);

    let my_ticket: Vec<usize> = parse_ticket(iter.next().unwrap().lines().nth(1).unwrap());

    let tickets: Vec<Ticket> = iter
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    (rules, my_ticket, tickets)
}

fn parse_rules(input: &[String]) -> Vec<Rule> {
    input.iter().map(parse_rule_line).collect()
}

fn parse_rule_line(line: &String) -> Rule {
    let to_range = |r: &str| {
        let mut iter = r.split('-');
        let min: usize = iter.next().unwrap().parse().unwrap();
        let max: usize = iter.next().unwrap().parse().unwrap();

        min..=max
    };

    let mut parts = line.split(": ");
    let _rule_name = parts.next().unwrap();
    let mut parts = parts.next().unwrap().split(" or ");

    (
        to_range(parts.next().unwrap()),
        to_range(parts.next().unwrap()),
    )
}

fn parse_ticket(ticket: &str) -> Ticket {
    ticket.split(',').map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "class: 1-3 or 5-7
row space: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn readme_example() {
        assert_eq!(part_1(&INPUT), 71);
    }

    #[test]
    fn test_parse() {
        let (rules, my_ticket, nearby_tickets) = parse(&INPUT);

        assert_eq!(
            rules,
            vec![(1..=3, 5..=7), (6..=11, 33..=44), (13..=40, 45..=50)]
        );

        assert_eq!(my_ticket, vec![7, 1, 14]);

        assert_eq!(
            nearby_tickets,
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ]
        );
    }

    #[test]
    fn test_parse_rules() {
        let input: Vec<_> = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(
            parse_rules(&input),
            vec![(1..=3, 5..=7), (6..=11, 33..=44), (13..=40, 45..=50)]
        );
    }

    #[test]
    fn test_ticket_valid() {
        let rules = vec![(1..=3, 5..=7), (6..=11, 33..=44), (13..=40, 45..=50)];

        assert_eq!(ticket_valid(&rules, &vec![7, 3, 47]), true);
        assert_eq!(ticket_valid(&rules, &vec![40, 4, 50]), false);
        assert_eq!(ticket_valid(&rules, &vec![55, 2, 20]), false);
        assert_eq!(ticket_valid(&rules, &vec![38, 6, 12]), false);
    }
}
