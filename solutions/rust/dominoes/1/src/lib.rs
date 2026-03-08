fn recurse(chain: &[(u8, u8)], remaining_input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if remaining_input.is_empty() {
        // Check for solution
        for i in 0..chain.len() - 1 {
            let current = chain[i];
            let next = chain[i + 1];
            if current.1 != next.0 {
                return None;
            }
        }

        if chain.first().unwrap().0 != chain.last().unwrap().1 {
            return None;
        }

        return Some(chain.to_vec());
    }

    let mut new_chain = Vec::with_capacity(chain.len() + 1);
    new_chain.extend_from_slice(chain);
    let mut new_remaining = remaining_input.to_vec();

    for (index, &(a, b)) in remaining_input.iter().enumerate() {
        let removed = new_remaining.remove(index);
        for next in [(a, b), (b, a)] {
            if chain.last().map(|last| last.1 == next.0).unwrap_or(true) {
                new_chain.push(next);

                if let Some(solution) = recurse(&new_chain, &new_remaining) {
                    return Some(solution);
                }

                new_chain.pop();
            }
        }
        new_remaining.insert(index, removed);
    }
    None
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(Vec::new());
    }

    recurse(&[], input)
}
