use nom::{
    branch::alt, 
    bytes::complete::tag,
    character::complete::char, 
    combinator::map,
    sequence::tuple,
    IResult
};

fn parse_abc(input: &str) -> IResult<&str, char> {
    alt((char('a'), char('b'), char('c'))) (input)
}

fn parse_abc_sequence(input: &str)
    -> IResult<&str, (char, char, char)> {
        tuple((char('a'), char('b'), char('c'))) (input)
}

fn parse_abc_string(input: &str) -> IResult<&str, &str> {
    tag("abc") (input)
}

fn parse_abc_as_numbers(input: &str)
    -> IResult<&str, u8> {
        alt((
                map(char('a'), |_| 5), 
                map(char('b'), |_| 16),
                map(char('c'), |_| 8),
        )) (input)
}

fn main() {
    //println!("a: {:?}", parse_abc("a"));
    //println!("x: {:?}", parse_abc("x"));
    //println!("bjk: {:?}", parse_abc("bjk"));
    //println!("abc: {:?}", parse_abc_sequence("abc"));
    //println!("bca: {:?}", parse_abc_sequence("bca"));
    //println!("abcjk: {:?}", parse_abc_sequence("abcjk"));
    //println!("abc: {:?}", parse_abc_string("abc"));
    //println!("bca: {:?}", parse_abc_string("bca"));
    //println!("abcjk: {:?}", parse_abc_string("abcjk"));
    println!("a: {:?}", parse_abc_as_numbers("a"));
    println!("x: {:?}", parse_abc_as_numbers("x"));
    println!("bjk: {:?}", parse_abc_as_numbers("bjk"));
}
