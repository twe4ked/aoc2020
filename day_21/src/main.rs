fn main() {
    todo!();
}

#[derive(Debug)]
pub struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

peg::parser! {
    grammar parser() for str {
        rule ingredients() -> Vec<&'input str>
            = $(['a'..='z']+) ** " "

        rule allergens() -> Vec<&'input str>
            = $(['a'..='z']+) ** ", "

        pub rule parse_line() -> Food<'input>
            = ingredients:ingredients() " (contains " allergens:allergens() ")" {
                Food {
                    ingredients,
                    allergens,
                }
            }
    }
}

fn parse<'a>(input: &'a str) -> Vec<Food<'a>> {
    input
        .lines()
        .map(parser::parse_line)
        .filter_map(|l| l.ok())
        .collect()
}

fn part_1(input: &str) -> u64 {
    let _foods = dbg!(parse(&input));

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn readme_example() {
        assert_eq!(part_1(&INPUT), 5);
    }
}
