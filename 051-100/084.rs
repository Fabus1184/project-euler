#!/usr/bin/env rust-script

// cargo-deps: rand = "0.8.5", itertools = "0.10.5"

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Field {
    GO,  A1, CC1,  A2,  T1,  R1,  B1, CH1, B2,  B3, JAIL,
    C1,  U1,  C2,  C3,  R2,  D1,  CC2, D2,  D3, FP,  E1,
    CH2, E2,  E3,  R3,  F1,  F2,  U2,  F3, G2J, G1,  G2,
    CC3, G3,  R4,  CH3, H1,  T2,  H2
}

fn community_chest(current: Field) -> Field {
    match rand::random::<u8>() % 16 {
        0 => Field::GO,
        1 => Field::JAIL,
        _ => current
    }
}

fn chance(current: Field) -> Field {
    fn next_railway(current: Field) -> Field {
        match current {
            Field::CH1 => Field::R2,
            Field::CH2 => Field::R3,
            Field::CH3 => Field::R1,
            _ => unreachable!()
        }
    }

    fn next_utility(current: Field) -> Field {
        match current {
            Field::CH1 => Field::U1,
            Field::CH2 => Field::U2,
            Field::CH3 => Field::U1,
            _ => unreachable!()
        }
    }

    match rand::random::<u8>() % 16 {
        0 => Field::GO,
        1 => Field::JAIL,
        2 => Field::C1,
        3 => Field::E3,
        4 => Field::H2,
        5 => Field::R1,
        6 | 7 => next_railway(current),
        8 => next_utility(current),
        9 => unsafe { std::mem::transmute((current as u8 - 3) % 40) },
        _ => current
    }
}

fn turn(mut current: Field) -> Field {
    let die1 = rand::random::<u8>() % 4 + 1;
    let die2 = rand::random::<u8>() % 4 + 1;

    let mut n = die1 + die2;

    if die1 == die2 {
        let mut doubles = 1;
        while n == 0 {
            let die1 = rand::random::<u8>() % 4 + 1;
            let die2 = rand::random::<u8>() % 4 + 1;
            if die1 == die2 {
                doubles += 1;
            } else {
                n = die1 + die2;
            }
            if doubles == 3 {
                return Field::JAIL;
            }
        }
    }

    current = unsafe { std::mem::transmute((current as u8 + n) % 40) };

    match current {
        Field::G2J => Field::JAIL,
        Field::CH1 | Field::CH2 | Field::CH3 => chance(current),
        Field::CC1 | Field::CC2 | Field::CC3 => community_chest(current),
        _ => current
    }
}

let mut current = Field::GO;
let mut counts = [0usize; 40];
const MAX: usize = 1_000_000;
for i in 0..MAX {
    current = turn(current);
    counts[current as usize] += 1;

    if i % 10_000 == 0 {
        current = Field::GO;
    }
}

println!("{}", counts
    .iter()
    .enumerate()
    .sorted_by_key(|&(_, c)| c)
    .rev()
    .take(3)
    .map(|(i,_ )| i.to_string())
    .join("")
);
