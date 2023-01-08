/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let iter_hand = hands.into_iter().map(|each_hand| {
        let cards = each_hand.split(' ');
        let mut total_value = 0;
        for each_card in cards {
            let chars: Vec<char> = each_card.chars().collect();
            if chars[0] == 'A' {
                total_value += 140
            } else if chars[0] == 'K' {
                total_value += 130
            } else if chars[0] == 'Q' {
                total_value += 120
            } else if chars[0] == 'J' {
                total_value += 110
            } else {
                total_value += chars[0].to_digit(10).unwrap();
            }
        }
        total_value
    }).collect::<Vec<u32>>();

    println!("iter hands is {:?}", iter_hand);


    if hands.len() == 1 {
        return hands.to_vec()
    }
    hands.to_vec()
}

// A K Q J 10 9 8 7 6 5 4 3 2 1
