use std::collections::VecDeque;

pub fn winning_score(players: usize, last_marble: u32) -> u32 {
    let mut queue = VecDeque::new();
    let mut scores = vec![0; players];

    queue.push_back(0);

    for i in 1..last_marble + 1 {
        if i % 23 == 0 {
            rotate_right(&mut queue, 7);
            let value = queue.pop_back().unwrap();
            scores[(i as usize % players) as usize] += i + value;
            rotate_left(&mut queue, 1);
        } else {
            rotate_left(&mut queue, 1);
            queue.push_back(i);
        }
    }

    *scores.iter().max().unwrap()
}

fn rotate_right(queue: &mut VecDeque<u32>, count: usize) {
    for _ in 0..count {
        let value = queue.pop_back().unwrap();
        queue.push_front(value);
    }
}

fn rotate_left(queue: &mut VecDeque<u32>, count: usize) {
    for _ in 0..count {
        let value = queue.pop_front().unwrap();
        queue.push_back(value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    macro_rules! winning_score_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (players, last_marble, expected) = $value;
                    let result = winning_score(players, last_marble);
                    assert_that!(result).is_equal_to(expected);
                }
            )*
        }
    }

    winning_score_tests! {
        winning_score_9: (9, 25, 32),
        winning_score_10: (10, 1618, 8317),
        winning_score_13: (13, 7999, 146373),
        winning_score_17: (17, 1104, 2764),
        winning_score_21: (21, 6111, 54718),
        winning_score_30: (30, 5807, 37305),
    }

    #[test]
    fn winning_score_input() {
        let score = winning_score(486, 70833);
        assert_that!(score).is_equal_to(373597);
    }

    #[test]
    fn winning_score_input_larger() {
        let score = winning_score(486, 70833 * 100);
        assert_that!(score).is_equal_to(2954067253);
    }
}
