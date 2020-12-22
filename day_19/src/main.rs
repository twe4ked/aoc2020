// --- Day 19: Monster Messages ---
//
// You land in an airport surrounded by dense forest. As you walk to your high-speed train, the
// Elves at the Mythical Information Bureau contact you again. They think their satellite has
// collected an image of a sea monster! Unfortunately, the connection to the satellite is having
// problems, and many of the messages sent back from the satellite have been corrupted.
//
// They sent you a list of the rules valid messages should obey and a list of received messages
// they've collected so far (your puzzle input).
//
// The rules for valid messages (the top part of your puzzle input) are numbered and build upon
// each other. For example:
//
// 0: 1 2
// 1: "a"
// 2: 1 3 | 3 1
// 3: "b"
//
// Some rules, like 3: "b", simply match a single character (in this case, b).
//
// The remaining rules list the sub-rules that must be followed; for example, the rule 0: 1 2 means
// that to match rule 0, the text being checked must match rule 1, and the text after the part that
// matched rule 1 must then match rule 2.
//
// Some of the rules have multiple lists of sub-rules separated by a pipe (|). This means that at
// least one list of sub-rules must match. (The ones that match might be different each time the
// rule is encountered.) For example, the rule 2: 1 3 | 3 1 means that to match rule 2, the text
// being checked must match rule 1 followed by rule 3 or it must match rule 3 followed by rule 1.
//
// Fortunately, there are no loops in the rules, so the list of possible matches will be finite.
// Since rule 1 matches a and rule 3 matches b, rule 2 matches either ab or ba. Therefore, rule 0
// matches aab or aba.
//
// Here's a more interesting example:
//
// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
//
// Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two letters that are the
// same (aa or bb), and rule 3 matches two letters that are different (ab or ba).
//
// Since rule 1 matches rules 2 and 3 once each in either order, it must match two pairs of
// letters, one pair with matching letters and one pair with different letters. This leaves eight
// possibilities: aaab, aaba, bbab, bbba, abaa, abbb, baaa, or babb.
//
// Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule
// 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.
//
// The received messages (the bottom part of your puzzle input) need to be checked against the
// rules so you can determine which are valid and which are corrupted. Including the rules and the
// messages together, this might look like:
//
// 0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: "a"
// 5: "b"
//
// ababbb
// bababa
// abbbab
// aaabbb
// aaaabbb
//
// Your goal is to determine the number of messages that completely match rule 0. In the above
// example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not, producing the answer
// 2. The whole message must match all of rule 0; there can't be extra unmatched characters in the
// message. (For example, aaaabbb might appear to match rule 0 above, but it has an extra unmatched
// b on the end.)
//
// How many messages completely match rule 0?
//
// --- Part Two ---
//
// As you look over the list of messages, you realize your matching rules aren't quite right. To
// fix them, completely replace rules 8: 42 and 11: 42 31 with the following:
//
// 8: 42 | 42 8
// 11: 42 31 | 42 11 31
//
// This small change has a big impact: now, the rules do contain loops, and the list of messages
// they could hypothetically match is infinite. You'll need to determine how these changes affect
// which messages are valid.
//
// Fortunately, many of the rules are unaffected by this change; it might help to start by looking
// at which rules always match the same set of values and how those rules (especially rules 42 and
// 31) are used by the new versions of rules 8 and 11.
//
// (Remember, you only need to handle the rules you have; building a solution that could handle any
// hypothetical combination of rules would be significantly more difficult.)
//
// For example:
//
// 42: 9 14 | 10 1
// 9: 14 27 | 1 26
// 10: 23 14 | 28 1
// 1: "a"
// 11: 42 31
// 5: 1 14 | 15 1
// 19: 14 1 | 14 14
// 12: 24 14 | 19 1
// 16: 15 1 | 14 14
// 31: 14 17 | 1 13
// 6: 14 14 | 1 14
// 2: 1 24 | 14 4
// 0: 8 11
// 13: 14 3 | 1 12
// 15: 1 | 14
// 17: 14 2 | 1 7
// 23: 25 1 | 22 14
// 28: 16 1
// 4: 1 1
// 20: 14 14 | 1 15
// 3: 5 14 | 16 1
// 27: 1 6 | 14 18
// 14: "b"
// 21: 14 1 | 1 14
// 25: 1 1 | 1 14
// 22: 14 14
// 8: 42
// 26: 14 22 | 1 20
// 18: 15 15
// 7: 14 5 | 1 21
// 24: 14 1
//
// abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
// bbabbbbaabaabba
// babbbbaabbbbbabbbbbbaabaaabaaa
// aaabbbbbbaaaabaababaabababbabaaabbababababaaa
// bbbbbbbaaaabbbbaaabbabaaa
// bbbababbbbaaaaaaaabbababaaababaabab
// ababaaaaaabaaab
// ababaaaaabbbaba
// baabbaaaabbaaaababbaababb
// abbbbabbbbaaaababbbbbbaaaababb
// aaaaabbaabaaaaababaa
// aaaabbaaaabbaaa
// aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
// babaaabbbaaabaababbaabababaaab
// aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//
// Without updating rules 8 and 11, these rules only match three messages: bbabbbbaabaabba,
// ababaaaaaabaaab, and ababaaaaabbbaba.
//
// However, after updating rules 8 and 11, a total of 12 messages match:
//
//     bbabbbbaabaabba
//     babbbbaabbbbbabbbbbbaabaaabaaa
//     aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//     bbbbbbbaaaabbbbaaabbabaaa
//     bbbababbbbaaaaaaaabbababaaababaabab
//     ababaaaaaabaaab
//     ababaaaaabbbaba
//     baabbaaaabbaaaababbaababb
//     abbbbabbbbaaaababbbbbbaaaababb
//     aaaaabbaabaaaaababaa
//     aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//     aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//
// After updating rules 8 and 11, how many messages completely match rule 0?

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 291);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);
    println!("Part 2: {}", part_2);
}

fn part_1(input: &str) -> usize {
    let (rules, messages) = parse(&input);

    messages
        .iter()
        .filter(|message| matches(&rules, message))
        .count()
}

fn part_2(input: &str) -> usize {
    let (mut rules, messages) = parse(&input);

    let (idx, rule) = parse_line("8: 42 | 42 8");
    rules.insert(idx, rule);

    let (idx, rule) = parse_line("11: 42 31 | 42 11 31");
    rules.insert(idx, rule);

    messages
        .iter()
        .filter(|message| matches(&rules, message))
        .count()
}

fn matches(rules: &HashMap<u64, Node>, input: &str) -> bool {
    match check(rules, input, &0) {
        // If input is not empty, the input was too long
        MatchResult::Matched(remaining) => remaining.is_empty(),
        MatchResult::NotMatched => false,
    }
}

enum MatchResult<'a> {
    Matched(&'a str),
    NotMatched,
}

fn check<'a>(rules: &HashMap<u64, Node>, input: &'a str, rule_idx: &u64) -> MatchResult<'a> {
    if input.is_empty() {
        // The input was too short
        return MatchResult::NotMatched;
    }

    let follow = |rule_idxs: &[u64]| {
        let mut my_input = input;
        for rule_idx in rule_idxs {
            match check(&rules, &my_input, rule_idx) {
                MatchResult::Matched(remaining) => my_input = remaining,
                not_matched => return not_matched,
            }
        }

        MatchResult::Matched(my_input)
    };

    match &rules[rule_idx] {
        Node::Char(c) => {
            if input.starts_with(c) {
                MatchResult::Matched(&input[1..])
            } else {
                MatchResult::NotMatched
            }
        }
        Node::Single(rule_idxs) => follow(rule_idxs),
        Node::Split(rule_idxs_1, rule_idxs_2) => match follow(rule_idxs_1) {
            MatchResult::NotMatched => follow(rule_idxs_2),
            matched => matched,
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Node<'a> {
    Char(&'a str),
    Single(Vec<u64>),
    Split(Vec<u64>, Vec<u64>),
}

// Example input:
//      0: 1
//      1: 2 3 | 3 2
//      2: "a"
//
//  Output:
//      (0, Single(1))
//      (1, Split(vec![2, 3], vec![3, 2]))
//      (2, Char("a"))
fn parse_line(input: &str) -> (u64, Node) {
    let mut iter = input.split(": ");
    let idx = iter.next().unwrap().parse().unwrap();

    let rest = iter.next().unwrap();

    let parse_numbers =
        |s: &str| -> Vec<u64> { s.split(' ').map(|c| c.parse().unwrap()).collect() };

    let node = if rest.starts_with('"') {
        Node::Char(&rest[1..2])
    } else {
        let mut iter = rest.split(" | ");
        let first = parse_numbers(iter.next().unwrap());

        if let Some(second) = iter.next() {
            let second = parse_numbers(second);
            Node::Split(first, second)
        } else {
            Node::Single(first)
        }
    };

    (idx, node)
}

fn parse(input: &str) -> (HashMap<u64, Node>, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap();
    let messages = iter.next().unwrap().lines().collect();

    let rules = rules.lines().fold(HashMap::new(), |mut acc, line| {
        let (idx, rule) = parse_line(line);
        acc.insert(idx, rule);
        acc
    });

    (rules, messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Flattened:
    // 0: a ((a a | b b) (a b | b a) | (a b | b a) (a a | b b)) b
    const INPUT: &'static str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn readme_example() {
        assert_eq!(part_1(&INPUT), 2);
    }

    #[test]
    fn readme_example_part_2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        // currently matching:
        // "bbabbbbaabaabba",
        // "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
        // "ababaaaaaabaaab",
        // "ababaaaaabbbaba",
        // "baabbaaaabbaaaababbaababb",
        // "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",

        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 12);
    }

    #[test]
    fn test_parse() {
        let (rules, messages) = parse(&INPUT);

        assert_eq!(
            vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb",],
            messages
        );

        let expected_rules = {
            let mut map = HashMap::new();
            map.insert(0, Node::Single(vec![4, 1, 5]));
            map.insert(1, Node::Split(vec![2, 3], vec![3, 2]));
            map.insert(2, Node::Split(vec![4, 4], vec![5, 5]));
            map.insert(3, Node::Split(vec![4, 5], vec![5, 4]));
            map.insert(4, Node::Char("a"));
            map.insert(5, Node::Char("b"));
            map
        };

        assert_eq!(rules, expected_rules);
    }

    #[test]
    fn test_matches() {
        let rules = {
            let mut map = HashMap::new();
            map.insert(0, Node::Single(vec![1, 2]));
            map.insert(1, Node::Char("a"));
            map.insert(2, Node::Char("b"));
            map
        };

        assert!(matches(&rules, "ab"), "should match");

        assert!(!matches(&rules, "a"), "too short");
        assert!(!matches(&rules, "ba"), "backwards");
        assert!(!matches(&rules, "ax"), "wrong char");
        assert!(!matches(&rules, "abb"), "too long");

        // README example:
        let (rules, _messages) = parse(&INPUT);
        assert!(matches(&rules, "ababbb"));
        assert!(matches(&rules, "abbbab"));
        assert!(!matches(&rules, "bababa"));
        assert!(!matches(&rules, "aaabbb"));
        assert!(!matches(&rules, "aaaabbb"));

        let rules = {
            let mut map = HashMap::new();
            map.insert(0, Node::Split(vec![1, 2], vec![1, 2]));
            map.insert(1, Node::Char("a"));
            map.insert(2, Node::Char("b"));
            map
        };

        // Regression test: Updating the input when a rule does not match
        assert!(!matches(&rules, "aab"), "should not match");
    }
}
