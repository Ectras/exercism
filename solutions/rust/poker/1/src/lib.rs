use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
};

use itertools::{Itertools, MinMaxResult};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut candidates: Option<(HandValue, Vec<&str>)> = None;

    for hand in hands {
        let (ranks, colors): (Vec<_>, Vec<_>) =
            hand.split_ascii_whitespace().map(parse_card).unzip();
        let val = value(&ranks, &colors);

        if let Some((old_val, hands)) = &mut candidates {
            match val.cmp(&old_val) {
                // Same value -> add to current winning hands
                Ordering::Equal => hands.push(*hand),

                // Better than old value -> this is the new winning candidate
                Ordering::Greater => {
                    *hands = vec![*hand];
                    *old_val = val;
                }

                // Worse value -> ignore hand
                _ => (),
            }
        } else {
            // If we don't have a candidate yet, use this hand as candidate
            candidates = Some((val, vec![*hand]));
        }
    }
    candidates.map(|(_, hands)| hands).unwrap()
}

fn parse_card(input: &str) -> (Rank, Suit) {
    assert!((2..=3).contains(&input.len()));
    let (rank, suit) = input.split_at(input.len() - 1);
    let suit = suit.chars().next().unwrap();
    (rank.try_into().unwrap(), suit.try_into().unwrap())
}

fn value(ranks: &[Rank], colors: &[Suit]) -> HandValue {
    let ranks = RankVec::new_hand(ranks);

    let flush = is_flush(&colors);
    let straight = ranks.is_straight();
    if flush && straight {
        let highest = ranks.highest();
        if highest == Rank::Ace {
            return HandValue::RoyalFlush;
        } else {
            return HandValue::StraightFlush { highest };
        }
    }

    let counts = count_ranks(&ranks);
    if let Some(four_kind) = counts.get(&4) {
        // Four of a kind + 1 single
        let single = counts[&1][0];
        return HandValue::FourOfAKind {
            rank: four_kind[0],
            single,
        };
    }

    if let Some(three_kind) = counts.get(&3) {
        if let Some(pair_kind) = counts.get(&2) {
            return HandValue::FullHouse {
                three: three_kind[0],
                pair: pair_kind[0],
            };
        }
    }

    if flush {
        return HandValue::Flush { ranks };
    }

    if straight {
        return HandValue::Straight {
            highest: ranks.highest(),
        };
    }

    if let Some(three_kind) = counts.get(&3) {
        // Three of a kind + 2 singles
        let singles = &counts[&1];
        return HandValue::ThreeOfAKind {
            rank: three_kind[0],
            singles: RankVec::new(&singles),
        };
    }

    if let Some(pair_kind) = counts.get(&2) {
        if pair_kind.len() == 2 {
            // Two pairs + 1 single
            let MinMaxResult::MinMax(lower, higher) = pair_kind.iter().copied().minmax() else {
                unreachable!()
            };
            let single = counts[&1][0];
            return HandValue::TwoPair {
                lower,
                higher,
                single,
            };
        } else {
            // One pair + 3 single
            let rank = pair_kind[0];
            let singles = &counts[&1];
            return HandValue::OnePair {
                rank,
                singles: RankVec::new(&singles),
            };
        }
    }

    HandValue::HighCard { ranks }
}

fn count_ranks(ranks: &RankVec) -> HashMap<usize, Vec<Rank>> {
    let counts = ranks.0.iter().copied().counts();
    let mut out = HashMap::<_, Vec<_>>::new();
    for (rank, count) in counts {
        out.entry(count).or_default().push(rank);
    }
    out
}

fn is_flush(colors: &[Suit]) -> bool {
    colors.iter().all_equal()
}

// Credits for this enum to https://users.rust-lang.org/t/are-poker-hands-ord-or-partialord/82644/5
#[derive(Debug, Clone, PartialEq, Eq)]
enum HandValue {
    HighCard {
        ranks: RankVec,
    },
    OnePair {
        rank: Rank,
        singles: RankVec,
    },
    TwoPair {
        lower: Rank,
        higher: Rank,
        single: Rank,
    },
    ThreeOfAKind {
        rank: Rank,
        singles: RankVec,
    },
    Straight {
        highest: Rank,
    },
    Flush {
        ranks: RankVec,
    },
    FullHouse {
        three: Rank,
        pair: Rank,
    },
    FourOfAKind {
        rank: Rank,
        single: Rank,
    },
    StraightFlush {
        highest: Rank,
    },
    RoyalFlush,
}

impl HandValue {
    fn category(&self) -> u8 {
        match self {
            HandValue::HighCard { .. } => 0,
            HandValue::OnePair { .. } => 1,
            HandValue::TwoPair { .. } => 2,
            HandValue::ThreeOfAKind { .. } => 3,
            HandValue::Straight { .. } => 4,
            HandValue::Flush { .. } => 5,
            HandValue::FullHouse { .. } => 6,
            HandValue::FourOfAKind { .. } => 7,
            HandValue::StraightFlush { .. } => 8,
            HandValue::RoyalFlush => 9,
        }
    }
}

impl Ord for HandValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // High Card
            (
                HandValue::HighCard { ranks: self_ranks },
                HandValue::HighCard { ranks: other_ranks },
            ) => self_ranks.cmp(other_ranks),

            // One Pair
            (
                HandValue::OnePair {
                    rank: self_rank,
                    singles: self_singles,
                },
                HandValue::OnePair {
                    rank: other_rank,
                    singles: other_singles,
                },
            ) => self_rank
                .cmp(other_rank)
                .then(self_singles.cmp(other_singles)),

            // Two Pair
            (
                HandValue::TwoPair {
                    lower: self_lower,
                    higher: self_higher,
                    single: self_single,
                },
                HandValue::TwoPair {
                    lower: other_lower,
                    higher: other_higher,
                    single: other_single,
                },
            ) => self_higher
                .cmp(other_higher)
                .then(self_lower.cmp(other_lower))
                .then(self_single.cmp(other_single)),

            // Three of a Kind
            (
                HandValue::ThreeOfAKind {
                    rank: self_rank,
                    singles: self_singles,
                },
                HandValue::ThreeOfAKind {
                    rank: other_rank,
                    singles: other_singles,
                },
            ) => self_rank
                .cmp(other_rank)
                .then(self_singles.cmp(other_singles)),

            // Straight
            (
                HandValue::Straight {
                    highest: self_highest,
                },
                HandValue::Straight {
                    highest: other_highest,
                },
            ) => self_highest.cmp(other_highest),

            // Flush
            (HandValue::Flush { ranks: self_ranks }, HandValue::Flush { ranks: other_ranks }) => {
                self_ranks.cmp(other_ranks)
            }

            // Full House
            (
                HandValue::FullHouse {
                    three: self_three,
                    pair: self_pair,
                },
                HandValue::FullHouse {
                    three: other_three,
                    pair: other_pair,
                },
            ) => self_three.cmp(other_three).then(self_pair.cmp(other_pair)),

            // Four of a Kind
            (
                HandValue::FourOfAKind {
                    rank: self_rank,
                    single: self_single,
                },
                HandValue::FourOfAKind {
                    rank: other_rank,
                    single: other_single,
                },
            ) => self_rank
                .cmp(other_rank)
                .then(self_single.cmp(other_single)),

            // Straight flush
            (
                HandValue::StraightFlush {
                    highest: self_highest,
                },
                HandValue::StraightFlush {
                    highest: other_highest,
                },
            ) => self_highest.cmp(other_highest),

            // Royal flush
            (HandValue::RoyalFlush, HandValue::RoyalFlush) => Ordering::Equal,

            // Mixed types
            _ => self.category().cmp(&other.category()),
        }
    }
}

impl PartialOrd for HandValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl TryFrom<char> for Suit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'H' => Ok(Self::Hearts),
            'D' => Ok(Self::Diamonds),
            'S' => Ok(Self::Spades),
            'C' => Ok(Self::Clubs),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RankVec(Vec<Rank>);

impl RankVec {
    fn new(ranks: &[Rank]) -> Self {
        Self(
            ranks
                .iter()
                .copied()
                .sorted_unstable_by_key(|&r| Reverse(r))
                .collect_vec(),
        )
    }

    fn new_hand(ranks: &[Rank]) -> Self {
        let sorted = Self::new(ranks);
        // If we have a straight where the ace should be the lowest rank,
        // return the Ranks with the `LowAce` instead.
        match &sorted.0[..] {
            [Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] => RankVec(vec![
                Rank::Five,
                Rank::Four,
                Rank::Three,
                Rank::Two,
                Rank::LowAce,
            ]),
            _ => sorted,
        }
    }

    fn highest(&self) -> Rank {
        *self.0.first().unwrap()
    }

    fn is_straight(&self) -> bool {
        self.0
            .iter()
            .map(|&r| self.highest() as i8 - r as i8)
            .eq(0..5)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    LowAce = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<&str> for Rank {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            number => {
                let number = number.parse().map_err(|_| ())?;
                match number {
                    2 => Ok(Self::Two),
                    3 => Ok(Self::Three),
                    4 => Ok(Self::Four),
                    5 => Ok(Self::Five),
                    6 => Ok(Self::Six),
                    7 => Ok(Self::Seven),
                    8 => Ok(Self::Eight),
                    9 => Ok(Self::Nine),
                    10 => Ok(Self::Ten),
                    _ => Err(()),
                }
            }
        }
    }
}
