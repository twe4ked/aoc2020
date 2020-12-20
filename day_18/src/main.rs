fn main() {}

#[derive(Debug, PartialEq)]
enum Expression {
    Num(usize),
    Add(Box<Expression>, Box<Expression>),
    Mulitply(Box<Expression>, Box<Expression>),
    Paren(Box<Expression>),
}

fn parse(line: &str, lhs: Option<Expression>) -> Expression {
    if let Some(line) = line.strip_prefix('(') {
        Expression::Paren(Box::new(parse(line, None)))
    } else if let Some(line) = line.strip_suffix(')') {
        parse(line, None)
    } else {
        let mut iter = line.splitn(2, ' ');

        let current = iter.next().unwrap();

        if let Ok(n) = current.parse::<usize>() {
            let n = Expression::Num(n);
            if let Some(line) = iter.next() {
                parse(line, Some(n))
            } else {
                n
            }
        } else {
            match current {
                "+" => Expression::Add(
                    Box::new(lhs.expect("no LHS")),
                    Box::new(parse(iter.next().expect("no RHS"), None)),
                ),
                "*" => Expression::Mulitply(
                    Box::new(lhs.expect("no LHS")),
                    Box::new(parse(iter.next().expect("no RHS"), None)),
                ),
                _ => panic!("unknown: {}", current),
            }
        }
    }
}

fn evaluate(expression: Expression) -> usize {
    match expression {
        Expression::Num(n) => n,
        Expression::Add(e1, e2) => evaluate(*e1) + evaluate(*e2),
        Expression::Mulitply(e1, e2) => evaluate(*e1) * evaluate(*e2),
        Expression::Paren(e) => evaluate(*e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_example() {
        let line = "1 + 2 * 3 + 4 * 5 + 6";

        assert_eq!(evaluate(parse(&line, None)), 71);
    }

    #[test]
    fn test_parse() {
        let e: Option<Expression> = None;
        assert_eq!(
            parse(&"(4 * (5 + 6))", e),
            *paren(multiply(num(4), paren(add(num(5), num(6)))))
        );
    }

    #[test]
    fn test_evaluate() {
        // (4 * (5 + 6))
        let expression = paren(multiply(num(4), paren(add(num(5), num(6)))));
        assert_eq!(evaluate(*expression), 44);
    }

    fn paren(e: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Paren(e))
    }

    fn add(e1: Box<Expression>, e2: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Add(e1, e2))
    }

    fn multiply(e1: Box<Expression>, e2: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Mulitply(e1, e2))
    }

    fn num(n: usize) -> Box<Expression> {
        Box::new(Expression::Num(n))
    }
}
