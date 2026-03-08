use itertools::Itertools;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

fn count(dice: &Dice, number: u8) -> u8 {
    dice.iter().filter(|&&d| d == number).count() as _
}

fn is_house(dice: &Dice) -> bool {
    let counts = dice.iter().counts();
    if let Some((c1, c2)) = counts.into_values().collect_tuple() {
        c1 == 3 && c2 == 2 || c1 == 2 && c2 == 3
    } else {
        false
    }
}

fn is_little_straight(dice: &Dice) -> bool {
    dice.iter().all_unique() && !dice.contains(&6)
}

fn is_big_straight(dice: &Dice) -> bool {
    dice.iter().all_unique() && !dice.contains(&1)
}

fn is_four_of_a_kind(dice: &Dice) -> bool {
    dice.iter().counts().into_iter().any(|(_, c)| c >= 4)
}

fn is_yacht(dice: &Dice) -> bool {
    dice.iter().all_equal()
}

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => count(&dice, 1),
        Category::Twos => 2 * count(&dice, 2),
        Category::Threes => 3 * count(&dice, 3),
        Category::Fours => 4 * count(&dice, 4),
        Category::Fives => 5 * count(&dice, 5),
        Category::Sixes => 6 * count(&dice, 6),
        Category::FullHouse if is_house(&dice) => dice.iter().sum(),
        Category::FourOfAKind if is_four_of_a_kind(&dice) => dice
            .iter()
            .counts()
            .into_iter()
            .filter_map(|(d, c)| bool::then(c >= 4, || 4 * d))
            .exactly_one()
            .unwrap(),
        Category::LittleStraight if is_little_straight(&dice) => 30,
        Category::BigStraight if is_big_straight(&dice) => 30,
        Category::Choice => dice.iter().sum(),
        Category::Yacht if is_yacht(&dice) => 50,
        _ => 0,
    }
}
