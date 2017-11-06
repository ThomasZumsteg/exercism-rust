type Domino = (usize, usize);

pub fn chain(input: &Vec<Domino>) -> Option<Vec<Domino>> {
    if input.is_empty() { return Some(Vec::new()) }
    for string in strings(input[0..1].to_vec(), input[1..].to_vec()) {
        if string.get(0).unwrap().0 == string.last().unwrap().1 {
            return Some(string);
        }
    }
    None
}

fn strings(chain: Vec<Domino>, pile: Vec<Domino>) -> Vec<Vec<Domino>> {
    if pile.is_empty() { return vec![chain] }
    let root = chain.last().unwrap().1;
    let  mut result = Vec::new();
    for d in 0..pile.len() {
        let mut remainer = pile.clone();
        let mut domino = remainer.remove(d);

        if domino.1 == root { domino = (domino.1, domino.0) } 
        else if domino.0 != root { continue }

        let mut new_chain = chain.clone();
        new_chain.push(domino);
        for string in strings(new_chain, remainer) {
            result.push(string);
        }
    }
    result
}

