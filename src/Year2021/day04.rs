use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let mut blocks = input.split("\n\n");
    let draws = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .collect_vec();

    let boards = blocks
        .map(|b| {
            b.lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(|d| d.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    (draws, boards)
}

fn score(board: &Vec<Vec<usize>>, draws: &[usize]) -> usize {
    let unmarked_sum = board
        .iter()
        .flatten()
        .filter(|x| !draws.contains(x))
        .sum::<usize>();
    let last_draw = draws.last().unwrap();
    unmarked_sum * last_draw
}

fn has_won(board: &Vec<Vec<usize>>, draws: &[usize]) -> bool {
    for row in 0..board.len() {
        if (0..board[0].len()).all(|col| draws.contains(&board[row][col])) {
            return true;
        }
    }

    for col in 0..board[0].len() {
        if (0..board.len()).all(|row| draws.contains(&board[row][col])) {
            return true;
        }
    }
    false
}

pub fn day04a(input: &str) -> usize {
    let (draws, boards) = parse_input(input);
    for num_draws in 5..draws.len() {
        let winners = boards
            .iter()
            .filter(|b| has_won(b, &draws[0..num_draws]))
            .collect_vec();
        if winners.len() >= 1 {
            return score(winners[0], &draws[0..num_draws]);
        }
    }
    unreachable!()
}
pub fn day04b(input: &str) -> usize {
    let (draws, boards) = parse_input(input);

    // only consider boards that will win eventually
    let boards = boards.iter().filter(|b| has_won(b, &draws)).collect_vec();

    // going backwards, reducing the number of draws to consider, find the first losing board
    for num_draws in (5..draws.len()).rev() {
        let losers = boards
            .iter()
            .filter(|b| !has_won(b, &draws[0..num_draws]))
            .collect_vec();
        if losers.len() >= 1 {
            // + 1 because the first losing board would win on the next draw
            return score(losers[0], &draws[0..num_draws + 1]);
        }
    }
    unreachable!()
}
