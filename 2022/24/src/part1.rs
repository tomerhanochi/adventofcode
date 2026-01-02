use std::collections::HashSet;

use crate::{
    input::{Cell, Input},
    output::Output,
};

pub(crate) fn solve(input: &Input) -> Output {
    let mut valley = input.valley.clone();

    let mut minutes = 0;
    let mut players = HashSet::<(usize, usize)>::new();
    players.insert((1, 0));
    loop {
        minutes += 1;
        players =
            players
                .iter()
                .flat_map(|player| {
                    let mut moves = vec![*player, (player.0 - 1, player.1), (player.0 + 1, player.1), (player.0, player.1 + 1)];
                    if player.1 > 0 {
                        moves.push((player.0, player.1 - 1));
                    }
                    moves
                })
                .filter(
                    |possible_move| matches!(valley[*possible_move], Cell::Blizzards(ref blizzards) if blizzards.is_empty()),
                )
                .collect::<HashSet<_>>();
        if minutes == 1 {
            break;
        }
    }
    println!("{:?}", players);

    Output { minutes }
}
