// --- Day 18: Operation Order ---
//
// As you look out the window and notice a heavily-forested continent slowly appear over the
// horizon, you are interrupted by the child sitting next to you. They're curious if you could help
// them with their math homework.
//
// Unfortunately, it seems like this "math" follows different rules than you remember.
//
// The homework (your puzzle input) consists of a series of expressions that consist of addition
// (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate
// that the expression inside must be evaluated before it can be used by the surrounding
// expression. Addition still finds the sum of the numbers on both sides of the operator, and
// multiplication still finds the product.
//
// However, the rules of operator precedence have changed. Rather than evaluating multiplication
// before addition, the operators have the same precedence, and are evaluated left-to-right
// regardless of the order in which they appear.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
//   3   * 3 + 4 * 5 + 6
//       9   + 4 * 5 + 6
//          13   * 5 + 6
//              65   + 6
//                  71
//
// Parentheses can override this order; for example, here is what happens if parentheses are added
// to form 1 + (2 * 3) + (4 * (5 + 6)):
//
// 1 + (2 * 3) + (4 * (5 + 6))
// 1 +    6    + (4 * (5 + 6))
//      7      + (4 * (5 + 6))
//      7      + (4 *   11   )
//      7      +     44
//             51
//
// Here are a few more examples:
//
//     2 * 3 + (4 * 5) becomes 26.
//     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//
// Before you can help with the homework, you need to understand it yourself. Evaluate the
// expression on each line of the homework; what is the sum of the resulting values?
//
// --- Part Two ---
//
// You manage to answer the child's questions and they finish part 1 of their homework, but get
// stuck when they reach the next section: advanced math.
//
// Now, addition and multiplication have different precedence levels, but they're not the ones
// you're familiar with. Instead, addition is evaluated before multiplication.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
//   3   * 3 + 4 * 5 + 6
//   3   *   7   * 5 + 6
//   3   *   7   *  11
//      21       *  11
//          231
//
// Here are the other examples from above:
//
//     1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//     2 * 3 + (4 * 5) becomes 46.
//     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//
// What do you get if you add up the results of evaluating the homework problems using these new
// rules?

fn main() {
    let input = include_str!("../input");

    let part_1 = part_1(&input);
    assert_eq!(part_1, 12956356593940);
    println!("Part 1: {}", part_1);
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (_input, tokens) = parse(&line).unwrap();
            evaluate(tokens)
        })
        .sum()
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::{all_consuming, map, map_res, recognize};
    use nom::multi::many1;
    use nom::IResult;

    #[derive(Debug, Clone)]
    pub enum Token {
        Num(u64),
        Add,
        Multiply,
        Open,
        Close,
    }

    fn add(input: &str) -> IResult<&str, Token> {
        map(tag(" + "), |_| Token::Add)(input)
    }

    fn multiply(input: &str) -> IResult<&str, Token> {
        map(tag(" * "), |_| Token::Multiply)(input)
    }

    fn number(input: &str) -> IResult<&str, Token> {
        map(map_res(recognize(digit1), str::parse), |n: u64| {
            Token::Num(n)
        })(input)
    }

    fn open(input: &str) -> IResult<&str, Token> {
        map(tag("("), |_| Token::Open)(input)
    }

    fn close(input: &str) -> IResult<&str, Token> {
        map(tag(")"), |_| Token::Close)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Token>> {
        all_consuming(many1(alt(
            // Parse any of the following tokens:
            (number, add, multiply, open, close),
        )))(input)
    }
}

use crate::parser::{parse, Token};

fn evaluate(tokens: Vec<Token>) -> u64 {
    let tokens: Vec<_> = tokens.into_iter().rev().collect();

    let (_rest, acc) = inner(tokens, None);

    acc
}

fn inner(mut tokens: Vec<Token>, mut last: Option<Token>) -> (Vec<Token>, u64) {
    let mut acc = 0;

    while let Some(token) = tokens.pop() {
        match token {
            Token::Num(n) => match last {
                Some(Token::Multiply) => acc *= n,
                Some(Token::Add) => acc += n,
                None => acc = n,
                x => panic!("{:?}", x),
            },
            Token::Add | Token::Multiply => {
                last = Some(token);
            }
            Token::Open => {
                let res = inner(tokens.clone(), None);
                tokens = res.0;
                tokens.push(Token::Num(res.1));
            }
            Token::Close => {
                return (tokens, acc);
            }
        };
    }

    (tokens, acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let (_input, tokens) = parse(&input).unwrap();
        assert_eq!(evaluate(tokens), 71);

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let (_input, tokens) = parse(&input).unwrap();
        assert_eq!(evaluate(tokens), 51);

        assert_eq!(part_1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
