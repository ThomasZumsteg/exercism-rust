/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
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
    println!("{:?}", winners);
    Some(winners)
}

fn score(hand: &str) -> (usize, usize) {
    let ranks = [high_card, one_pair, two_pair, three_of_a_kind, 
        straight, flush, full_house, four_of_a_kind, straigh_flush];
    for (r, func) in ranks.iter().enumerate().rev() {
        if let Some(score) = func(hand) { return (r, score) }
    }
    return (0, 0)
}

fn high_card(hand: &str) -> Option<usize> {None}
fn one_pair(hand: &str) -> Option<usize> {None}
fn two_pair(hand: &str) -> Option<usize> {None}
fn three_of_a_kind(hand: &str) -> Option<usize> {None}
fn straight(hand: &str) -> Option<usize> {None}
fn flush(hand: &str) -> Option<usize> {None}
fn full_house(hand: &str) -> Option<usize> {None}
fn four_of_a_kind(hand: &str) -> Option<usize> {None}
fn straigh_flush(hand: &str) -> Option<usize> {None}
