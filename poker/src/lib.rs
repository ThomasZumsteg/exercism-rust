/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::collections::HashMap;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut winners: Vec<&'a str> = Vec::new();
    for hand in hands {
        println!("{:?}: {}", score(hand), hand);
        if winners.len() == 0 || 
            score(hand) == score(winners[0]) { winners.push(hand) }
        else if score(hand) > score(winners[0]) {
            winners.clear();
            winners.push(hand);
        }
    }
    println!("{:?}", winners);
    Some(winners)
}

fn score(hand: &str) -> (usize, Vec<usize>) {
    let ranks = [high_card, one_pair, two_pair, three_of_a_kind, 
        straight, flush, full_house, four_of_a_kind, straigh_flush];
    for (r, func) in ranks.iter().enumerate().rev() {
        if let Some(score) = func(hand) { return (r, score) }
    }
    return (0, Vec::new())
}

fn value(v: &str) -> usize {
    "2 3 4 5 6 7 8 9 10 J Q K A".split(" ").position(|i| i == v).unwrap()
}

fn suit(s: &str) -> usize { "HSCD".split("").position(|i| i == s).unwrap() }

fn suits_and_values(hand: &str) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut values: HashMap<usize, usize> = HashMap::new();
    let mut suits: HashMap<usize, usize> = HashMap::new();
    for card in hand.split(' ') {
        *values.entry(value(&card[..card.len()-1])).or_insert(0) += 1;
    }
    (values, suits)
}

fn high_card(hand: &str) -> Option<Vec<usize>> {
    let (values, _) = suits_and_values(hand);
    if values.values().cloned().collect::<Vec<usize>>() == [1,1,1,1,1] { 
        let mut result: Vec<usize> = values.keys().cloned().collect();
        result.sort();
        Some(result)
    } else { None }
}

fn one_pair(hand: &str) -> Option<Vec<usize>> {
    let mut value_map = suits_and_values(hand).0;
    let mut value_list = value_map.values().cloned().collect::<Vec<usize>>(); 
    value_list.sort();
    if value_list == [1,1,1,2] {
        let mut result = value_map.keys().cloned().collect::<Vec<usize>>();
        result.sort();
        Some(result)
    } else {None}
}

fn two_pair(hand: &str) -> Option<Vec<usize>> {None}
fn three_of_a_kind(hand: &str) -> Option<Vec<usize>> {None}
fn straight(hand: &str) -> Option<Vec<usize>> {None}
fn flush(hand: &str) -> Option<Vec<usize>> {None}
fn full_house(hand: &str) -> Option<Vec<usize>> {None}
fn four_of_a_kind(hand: &str) -> Option<Vec<usize>> {None}
fn straigh_flush(hand: &str) -> Option<Vec<usize>> {None}
