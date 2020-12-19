use std::collections::HashMap;
use std::ops::RangeInclusive;

type Ticket = Vec<usize>;

#[derive(Clone, Debug, PartialEq)]
struct Rule {
    name: String,
    range_1: RangeInclusive<usize>,
    range_2: RangeInclusive<usize>,
}

impl Rule {
    fn new(name: String, range_1: RangeInclusive<usize>, range_2: RangeInclusive<usize>) -> Self {
        Self {
            name,
            range_1,
            range_2,
        }
    }

    fn valid(&self, value: &usize) -> bool {
        self.range_1.contains(value) || self.range_2.contains(value)
    }
}

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 28_884);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);
    assert_eq!(part_2, 1_001_849_322_119);
    println!("Part 2: {}", part_2);
}

fn part_1(input: &str) -> usize {
    let (rules, _my_ticket, nearby_tickets) = parse(&input);

    nearby_tickets
        .iter()
        .flatten()
        .filter(|value| !rules.iter().any(|rule| rule.valid(value)))
        .sum()
}

fn part_2(input: &str) -> usize {
    let (rules, my_ticket, nearby_tickets) = parse(&input);

    let valid_tickets: Vec<_> = nearby_tickets
        .into_iter()
        .filter(|ticket| ticket_valid(&rules, ticket))
        .collect();

    determine_indexes(&rules, valid_tickets)
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| my_ticket[*idx])
        .product()
}

fn determine_indexes(rules: &[Rule], tickets: Vec<Ticket>) -> HashMap<String, usize> {
    let rules = rules.to_vec();
    let mut indexes_to_rules = HashMap::new();

    for (i, column) in transpose(tickets).iter().enumerate() {
        let x: Vec<_> = rules
            .iter()
            .filter(|rule| column.iter().all(|value| rule.valid(value)))
            .map(|rule| rule.name.clone())
            .collect();
        indexes_to_rules.insert(i, x);
    }

    let mut ret = HashMap::new();
    loop {
        let x: Option<(usize, String)> = {
            (&indexes_to_rules
                .iter()
                .find(|(_k, v)| v.len() == 1)
                .map(|(k, v)| (*k, v.first().unwrap().clone())))
                .clone()
        };

        if let Some((idx, only_one_valid_rule_name)) = x {
            for rules in indexes_to_rules.values_mut() {
                if rules.len() > 1 {
                    let mut x = rules.to_vec();
                    x.retain(|n| n != &only_one_valid_rule_name);
                    *rules = x;
                }
            }
            indexes_to_rules.remove(&idx);

            ret.insert(only_one_valid_rule_name.clone(), idx);
            1;
        } else {
            break;
        }
    }

    ret
}

// https://stackoverflow.com/a/64499219/826820
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn ticket_valid(rules: &[Rule], ticket: &Ticket) -> bool {
    ticket
        .iter()
        .all(|value| rules.iter().any(|rule| rule.valid(value)))
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
    let name = parts.next().unwrap().to_string();
    let mut parts = parts.next().unwrap().split(" or ");

    Rule::new(
        name,
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
    fn test_determine_indexes() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let (rules, _my_ticket, nearby_tickets) = parse(&input);

        let mut expected = HashMap::new();
        expected.insert("class".to_string(), 1);
        expected.insert("row".to_string(), 0);
        expected.insert("seat".to_string(), 2);

        assert_eq!(determine_indexes(&rules, nearby_tickets), expected);
    }

    #[test]
    fn test_parse() {
        let (rules, my_ticket, nearby_tickets) = parse(&INPUT);

        assert_eq!(
            rules,
            vec![
                Rule::new("class".to_string(), 1..=3, 5..=7),
                Rule::new("row space".to_string(), 6..=11, 33..=44),
                Rule::new("seat".to_string(), 13..=40, 45..=50)
            ]
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
            vec![
                Rule::new("class".to_string(), 1..=3, 5..=7),
                Rule::new("row".to_string(), 6..=11, 33..=44),
                Rule::new("seat".to_string(), 13..=40, 45..=50)
            ]
        );
    }

    #[test]
    fn test_ticket_valid() {
        let rules = vec![
            Rule::new("class".to_string(), 1..=3, 5..=7),
            Rule::new("row".to_string(), 6..=11, 33..=44),
            Rule::new("seat".to_string(), 13..=40, 45..=50),
        ];

        assert_eq!(ticket_valid(&rules, &vec![7, 3, 47]), true);
        assert_eq!(ticket_valid(&rules, &vec![40, 4, 50]), false);
        assert_eq!(ticket_valid(&rules, &vec![55, 2, 20]), false);
        assert_eq!(ticket_valid(&rules, &vec![38, 6, 12]), false);
    }
}
