const MAX_REMOVAL: i32 = 2;

// User is the one moving first
pub enum Player {
    User,
    Computer,
}

pub fn best_move(remaining_crabs: i32, calculate_for: Player) -> i32 {
    let (best_move, _) = match calculate_for {
        Player::User => minimax(remaining_crabs, true),
        Player::Computer => minimax(remaining_crabs, false),
    };
    best_move
}

fn minimax(remaining_crabs: i32, maximizing: bool) -> (i32, i32) {
    if remaining_crabs == 0 {
        return (0, 0);
    }

    // User is the one maximizing
    if maximizing {
        let mut best_score = std::i32::MIN;
        let mut best_move = 1;

        for removal in 1..=MAX_REMOVAL {
            if remaining_crabs >= removal {
                let (score, _) = minimax(remaining_crabs - removal, false);
                if score > best_score {
                    best_score = score;
                    best_move = removal;
                }
            }
        }

        (best_move, best_score)
    }
    // Computer is the one minimizing
    else {
        let mut best_score = std::i32::MAX;
        let mut best_move = 1;

        for removal in 1..=MAX_REMOVAL {
            if remaining_crabs >= removal {
                let (score, _) = minimax(remaining_crabs - removal, true);
                if score < best_score {
                    best_score = score;
                    best_move = removal;
                }
            }
        }

        (best_move, best_score)
    }
}
