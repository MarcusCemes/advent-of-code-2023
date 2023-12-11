advent_of_code::solution!(7);

use std::iter;

use arrayvec::ArrayVec;
use itertools::Itertools;

const HAND_SIZE: usize = 5;
const JOKER: u8 = 0;

#[derive(Copy, Clone)]
enum GameType {
    Normal,
    Joker,
}

struct Hand<'a> {
    bid: u32,
    cards: &'a str,
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, GameType::Normal))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, GameType::Joker))
}

/* == Functions == */

fn solve(input: &str, game_type: GameType) -> u32 {
    input
        .lines()
        .flat_map(Hand::parse_str)
        .map(|x| (x.score(game_type), x)) // pre-compute score for efficient sorting
        .sorted_unstable_by_key(|(score, _)| *score)
        .map(|(_, hand)| hand.bid)
        .enumerate()
        .fold(0, |acc, (i, bid)| acc + (i as u32 + 1) * bid) // compute winnings
}

impl Hand<'_> {
    fn parse_str(input: &str) -> Option<Hand> {
        input.split_once(' ').map(|(cards, bid)| Hand {
            bid: bid.parse().unwrap(),
            cards,
        })
    }

    /// Generates a "meaningless" u32 score value that represents the "value" of a hand.
    /// When comparing hands, the hand with the highest score is the winning hand, allowing
    /// for much more efficient sorting of winning hands.
    ///
    /// The score is composed of six 4-bit values. The most significant part (MSB) is the
    /// class score (TYPE), followed by the five card scores (CRD1, ..., CRD5):
    ///
    ///     0000 0000 TYPE CRD1 CRD2 CRD3 CRD4 CRD5
    ///
    /// The TYPE is higher for stronger hands (e.g. Full House is stronger than Three of a
    /// kind), and the CRD values are higher for stronger cards (e.g. T > 9, K > Q).
    fn score(&self, game_type: GameType) -> u32 {
        type CS = ArrayVec<u8, HAND_SIZE>; // static array, no allocation
        let card_scores: CS = self.card_numbers(game_type).collect();
        let class = card_class(card_scores.iter().copied());
        let score_parts = iter::once(class).chain(card_scores.iter().copied());
        join_u4(score_parts)
    }

    /// Returns an iterator of card scores for this hand.
    fn card_numbers(&self, game_t: GameType) -> impl Iterator<Item = u8> + '_ {
        self.cards.bytes().map(move |card| card_score(card, game_t))
    }
}

/// Join 4-bit values into a single 32-bit value. Does not check for overflow.
fn join_u4(scores: impl Iterator<Item = u8>) -> u32 {
    scores.fold(0, |acc, x| {
        debug_assert!(x < 16);
        acc * 16 + x as u32
    })
}

/// Returns the class score for a hand by determining the two-highest
/// card counts, which are sufficient to differentiate between all
/// possible types of hand (e.g. Full House vs. Three of a kind).
fn card_class(card_scores: impl Iterator<Item = u8>) -> u8 {
    match most_common_card_counts(card_scores) {
        (5, _) => 6, // Five of a kind
        (4, _) => 5, // Four of a kind
        (3, 2) => 4, // Full house
        (3, _) => 3, // Three of a kind
        (2, 2) => 2, // Two pair
        (2, _) => 1, // One pair
        (1, _) => 0, // Sucks to be you
        _ => panic!(),
    }
}

/// Returns the two most common card counts in a hand, where the first
/// value is the most common card count and the second value is the
/// second most common card count (may be equal to the first for Full House
/// or Two pair).
///
/// If the game type is Joker, any joker cards will become the most frequent card
/// (incrementing the count of the most common card by one for each joker, unless
/// the most common card is in fact the joker, in which case the second most common
/// card will be incremented instead and promoted to first place).
///
/// Given the simple class system, the most valuable hand is always the one where
/// jokers transform into the most common card. Therefore, it's not possible to
/// get a Full house or Two pair with jokers present.
fn most_common_card_counts(card_scores: impl Iterator<Item = u8>) -> (u8, u8) {
    let mut joker_count = 0;

    let frequencies = card_scores
        .inspect(|x| joker_count += (*x == JOKER) as u8)
        .sorted_unstable()
        .dedup_with_count();

    let highest_counts = frequencies
        .map(|(count, _)| count as u8)
        .sorted_unstable()
        .rev();

    let (a, b) = highest_counts
        .chain(iter::repeat(0)) // append a zero for Five of a kind
        .take(2)
        .collect_tuple()
        .unwrap();

    if a == joker_count {
        let new_a = b + joker_count;
        let new_b = new_a != HAND_SIZE as u8; // new_b is 0 <=> Five of a kind
        return (new_a, new_b as u8);
    }

    (a + joker_count, b)
}

/// The card score determines how valuable a card is, ranging from 0 to 13 for
/// the card order (J, 2, 3, ..., 9, T, J, Q, K, A). Depending on the game type,
/// the joker is either assigned a value of 0 (transformed) or 10 (standard).
fn card_score(card: u8, game_type: GameType) -> u8 {
    match (card as char).to_digit(10) {
        Some(digit) => digit as u8 - 1,
        None => match card {
            b'T' => 9,
            b'J' => match game_type {
                GameType::Normal => 10,
                GameType::Joker => JOKER,
            },
            b'Q' => 11,
            b'K' => 12,
            b'A' => 13,
            _ => panic!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&read_example_part(DAY, 1));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&read_example_part(DAY, 2));
        assert_eq!(result, Some(4466));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&read_example_part(DAY, 1));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&read_example_part(DAY, 2));
        assert_eq!(result, Some(4657));
    }

    #[test]
    fn card_scores() {
        assert_eq!(sc("AAAAA 0"), 0x6ddddd);
        assert_eq!(sc("AA8AA 0"), 0x5dd7dd);
        assert_eq!(sc("23332 0"), 0x412221);
        assert_eq!(sc("TTT98 0"), 0x399987);
        assert_eq!(sc("23432 0"), 0x212321);
        assert_eq!(sc("A23A4 0"), 0x1d12d3);
        assert_eq!(sc("23456 0"), 0x012345);
    }

    #[test]
    fn top_card_counts() {
        assert_eq!(tcc("QQQQQ 0"), (5, 0));
        assert_eq!(tcc("KKKK7 0"), (4, 1));
        assert_eq!(tcc("KKK77 0"), (3, 2));
        assert_eq!(tcc("KKK76 0"), (3, 1));
        assert_eq!(tcc("KK677 0"), (2, 2));
    }

    #[test]
    fn join_scores() {
        assert_eq!(js([3, 2, 1]), 0x321);
        assert_eq!(js([9, 3, 8]), 0x938);
    }

    fn tcc(hand_str: &str) -> (u8, u8) {
        let hand = Hand::parse_str(hand_str).unwrap();
        super::most_common_card_counts(hand.card_numbers(GameType::Normal))
    }

    fn sc(hand: &str) -> u32 {
        Hand::parse_str(hand).unwrap().score(GameType::Normal)
    }

    fn js(hand: impl IntoIterator<Item = u8>) -> u32 {
        super::join_u4(hand.into_iter())
    }
}
