use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use std::fmt;

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

struct Piles(Vec<Vec<Crate>>);

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        let mut items = (0..ins.quantity)
            .map(|_| self.0[ins.src].pop().unwrap())
            .collect_vec();
        items.reverse();
        for c in items {
            self.0[ins.dst].push(c);
        }
    }
}

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let crate_lines = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect::<Vec<Vec<Option<Crate>>>>();

    let mut crate_stacks: Vec<Vec<Crate>> = vec![];
    for _ in 0..crate_lines[0].len() {
        crate_stacks.push(vec![]);
    }

    for line in crate_lines.into_iter().rev() {
        for (i, maybe_c) in line.into_iter().enumerate() {
            match maybe_c {
                Some(c) => {
                    crate_stacks[i].push(c);
                }
                None => (),
            }
        }
    }

    let mut piles = Piles(crate_stacks);

    assert!(lines.next().unwrap().is_empty());

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        piles.apply(ins);
    }

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(input)
}

fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(input)
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(input)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(input)
}

fn parse_pile_number(input: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(input)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}
