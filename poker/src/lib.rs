use std::collections::HashMap;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut winners: Vec<&'a str> = Vec::new();
    for hand in hands {
        if winners.len() == 0 || 
            score(hand) == score(winners[0]) { winners.push(hand) }
        else if score(hand) > score(winners[0]) {
            winners.clear();
            winners.push(hand);
        }
    }
    Some(winners)
}

fn score(hand: &str) -> (usize, Vec<usize>) {
    let ranks = [high_card, one_pair, two_pair, three_of_a_kind, 
        straight, flush, full_house, four_of_a_kind, straigh_flush];
    let (values, suits) = values_and_suits(hand);
    for (r, func) in ranks.iter().enumerate().rev() {
        if let Some(score) = func(&values, &suits) { return (r, score) }
    }
    return (0, Vec::new())
}

fn value(v: &str) -> usize {
    if v == "_" { panic!("Oops") }
    "_ 2 3 4 5 6 7 8 9 10 J Q K A".split(" ").position(|i| i == v).unwrap()
}

fn suit(s: &str) -> usize { "HSCD".split("").position(|i| i == s).unwrap() }

fn values_and_suits(hand: &str) -> (Vec<(usize, usize)>, HashMap<usize, usize>) {
    let mut value_map: HashMap<usize, usize> = HashMap::new();
    let mut suits: HashMap<usize, usize> = HashMap::new();
    for card in hand.split(' ') {
        *value_map.entry(value(&card[..card.len()-1])).or_insert(0) += 1;
        *suits.entry(suit(&card[card.len()-1..])).or_insert(0) += 1;
    }
    let mut values =  value_map.iter().collect::<Vec<(&usize, &usize)>>();
    values.sort_by(|&a, &b| if a.1 != b.1 { a.1.cmp(b.1) } else { a.0.cmp(b.0) });
    println!("Here: {:?}", values);
    (values.iter().cloned().map(|(&a, &b)| (a, b)).rev().collect(), suits)
}

type Values = Vec<(usize, usize)>;
type Suits = HashMap<usize, usize>;

fn high_card(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let counts: Vec<usize> = values.iter().map(|v| v.1).collect();
    if counts == [1,1,1,1,1] { Some(values.iter().map(|v| v.0).collect()) }
    else { None }
}

fn one_pair(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let counts: Vec<usize> = values.iter().map(|v| v.1).collect();
    if counts == [2,1,1,1] { Some(values.iter().map(|v| v.0).collect()) }
    else { None }
}

fn two_pair(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let counts: Vec<usize> = values.iter().map(|v| v.1).collect();
    if counts == [2,2,1] { Some(values.iter().map(|v| v.0).collect()) }
    else { None }
}

fn three_of_a_kind(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let counts: Vec<usize> = values.iter().map(|v| v.1).collect();
    if counts == [3,1,1] { Some(values.iter().map(|v| v.0).collect()) }
    else { None }
}

fn straight(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let largest = values.first().unwrap().0 as isize;
    let diffs = values.iter().map(|&v| largest - (v.0 as isize))
        .collect::<Vec<isize>>();
    if diffs == &[0,1,2,3,4] {
        Some(values.iter().map(|&v| v.0).collect()) } 
    else if largest == 13 && diffs == [0, 9, 10, 11, 12] { Some(vec![0,1,2,3,4]) }
    else { None }
}

fn flush(values: &Values, suits: &Suits) -> Option<Vec<usize>> {
    if suits.values().len() == 1 {
        Some(values.iter().map(|&v| v.0).collect())
    } else { None }
}

fn full_house(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let counts: Vec<usize> = values.iter().map(|v| v.1).collect();
    if counts == [3,2] { Some(values.iter().map(|v| v.0).collect()) }
    else { None }
}

fn four_of_a_kind(values: &Values, _: &Suits) -> Option<Vec<usize>> {
    let counts: Vec<usize> = values.iter().map(|v| v.1).collect();
    if counts == [4,1] { Some(values.iter().map(|v| v.0).collect()) }
    else { None }
}

fn straigh_flush(values: &Values, suits: &Suits) -> Option<Vec<usize>> {
    if straight(values, suits).is_some() && flush(values, suits).is_some() {
        straight(values, suits)
    } else { None }
}
