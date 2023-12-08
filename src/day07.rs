//!  Day 07: Camel Cards
use std::collections::HashMap;

use anyhow::Result;

/// Camel card hand data.
#[derive(Debug)]
struct CamelCardHand {
    /// The cards.
    cards: [Card; 5],
    /// The bid.
    bid: u32,
    /// The hand type.
    hand_type: CamelCardHandType,
}

/// Camel card hand data with Joker.
#[derive(Debug)]
struct CamelCardHandWithJoker {
    /// The cards.
    cards: [CardWithJoker; 5],
    /// The bid.
    bid: u32,
    /// The hand type.
    hand_type: CamelCardHandType,
}

/// A card.
/// One of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
/// The relative strength of each card follows this order,
/// where A is the highest and 2 is the lowest.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    /// A
    A = 12,
    /// K
    K = 11,
    /// Q
    Q = 10,
    /// J
    J = 9,
    /// T
    T = 8,
    /// 9
    Nine = 7,
    /// 8
    Eight = 6,
    /// 7
    Seven = 5,
    /// 6
    Six = 4,
    /// 5
    Five = 3,
    /// 4
    Four = 2,
    /// 3
    Three = 1,
    /// 2
    Two = 0,
}

impl Card {
    /// Parse a card from a character.
    fn parse(ch: char) -> Result<Self> {
        match ch {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => Ok(Self::J),
            'T' => Ok(Self::T),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => anyhow::bail!("invalid card: {}", ch),
        }
    }
}

/// A card with joker.
/// One of A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, or J.
/// The relative strength of each card follows this order,
/// where A is the highest and 2 is the lowest.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardWithJoker {
    /// A
    A = 12,
    /// K
    K = 11,
    /// Q
    Q = 10,
    /// T
    T = 9,
    /// 9
    Nine = 8,
    /// 8
    Eight = 7,
    /// 7
    Seven = 6,
    /// 6
    Six = 5,
    /// 5
    Five = 4,
    /// 4
    Four = 3,
    /// 3
    Three = 2,
    /// 2
    Two = 1,
    /// J
    J = 0,
}

impl CardWithJoker {
    /// Parse a card from a character.
    fn parse(ch: char) -> Result<Self> {
        match ch {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => Ok(Self::J),
            'T' => Ok(Self::T),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => anyhow::bail!("invalid card: {}", ch),
        }
    }
}

/// Camel card hand type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CamelCardHandType {
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind = 6,
    /// Four of a kind, where four cards have the same label and one card has a
    /// different label: AA8AA
    FourOfAKind = 5,
    /// Full house, where three cards have the same label, and the remaining
    /// two cards share a different label: 23332
    FullHouse = 4,
    /// Three of a kind, where three cards have the same label, and the
    /// remaining two cards are each different from any other card in the
    /// hand: TTT98
    ThreeOfAKind = 3,
    /// Two pair, where two cards share one label, two other cards share a
    /// second label, and the remaining card has a third label: 23432
    TwoPair = 2,
    /// One pair, where two cards share one label, and the other three cards
    /// have a different label from the pair and each other: A23A4
    OnePair = 1,
    /// High card, where all cards' labels are distinct: 23456
    HighCard = 0,
}

/// Parse the camel card hand from the input.
fn parse_camel_card_hand(line: &str) -> Result<CamelCardHand> {
    let parts: Vec<_> = line.split_whitespace().collect();

    // Parse the hand.
    let cards: [Card; 5] = parts
        .first()
        .ok_or_else(|| anyhow::anyhow!("missing cards"))?
        .chars()
        .map(|ch| Card::parse(ch).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid number of cards"))?;

    // Parse the bid.
    let bid = parts
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("missing bid"))?
        .parse::<u32>()?;

    if cards.len() != 5 {
        return Err(anyhow::anyhow!("invalid number of cards"));
    }

    // Determine the hand type.
    let same_chars = same_chars(&cards);
    let hand_type = if same_chars.len() == 1 {
        // Five of a kind, where all five cards have the same label: AAAAA
        CamelCardHandType::FiveOfAKind
    } else if same_chars.len() == 2
        && same_chars.values().all(|&v| v == 4 || v == 1)
    {
        // Four of a kind, where four cards have the same label and one card has
        // a different label: AA8AA
        CamelCardHandType::FourOfAKind
    } else if same_chars.len() == 2
        && same_chars.values().all(|&v| v == 3 || v == 2)
    {
        // Full house, where three cards have the same label, and the remaining
        // two cards share a different label: 23332
        CamelCardHandType::FullHouse
    } else if same_chars.len() == 3
        && same_chars.values().all(|&v| v == 3 || v == 1)
    {
        // Three of a kind, where three cards have the same label, and the
        // remaining two cards are each different from any other card in
        // the hand: TTT98
        CamelCardHandType::ThreeOfAKind
    } else if same_chars.len() == 3
        && same_chars.values().all(|&v| v == 2 || v == 1)
    {
        // Two pair, where two cards share one label, two other cards share a
        // second label, and the remaining card has a third label: 23432
        CamelCardHandType::TwoPair
    } else if same_chars.len() == 4
        && same_chars.values().all(|&v| v == 2 || v == 1)
    {
        // One pair, where two cards share one label, and the other three cards
        // have a different label from the pair and each other: A23A4
        CamelCardHandType::OnePair
    } else {
        // High card, where all cards' labels are distinct: 23456
        CamelCardHandType::HighCard
    };

    Ok(CamelCardHand {
        cards,
        bid,
        hand_type,
    })
}

/// Parse the camel card hand with joker from the input.
fn parse_camel_card_hand_with_joker(
    line: &str,
) -> Result<CamelCardHandWithJoker> {
    let parts: Vec<_> = line.split_whitespace().collect();

    // Parse the hand.
    let cards: [CardWithJoker; 5] = parts
        .first()
        .ok_or_else(|| anyhow::anyhow!("missing cards"))?
        .chars()
        .map(|ch| CardWithJoker::parse(ch).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid number of cards"))?;

    // Parse the bid.
    let bid = parts
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("missing bid"))?
        .parse::<u32>()?;

    if cards.len() != 5 {
        return Err(anyhow::anyhow!("invalid number of cards"));
    }

    // Determine the hand type.
    let same_chars = same_chars_with_joker(&cards);
    let hand_type = if same_chars.len() == 1 {
        // Five of a kind, where all five cards have the same label: AAAAA
        CamelCardHandType::FiveOfAKind
    } else if same_chars.len() == 2
        && same_chars.values().all(|&v| v == 4 || v == 1)
    {
        // Four of a kind, where four cards have the same label and one card has
        // a different label: AA8AA
        CamelCardHandType::FourOfAKind
    } else if same_chars.len() == 2
        && same_chars.values().all(|&v| v == 3 || v == 2)
    {
        // Full house, where three cards have the same label, and the remaining
        // two cards share a different label: 23332
        CamelCardHandType::FullHouse
    } else if same_chars.len() == 3
        && same_chars.values().all(|&v| v == 3 || v == 1)
    {
        // Three of a kind, where three cards have the same label, and the
        // remaining two cards are each different from any other card in
        // the hand: TTT98
        CamelCardHandType::ThreeOfAKind
    } else if same_chars.len() == 3
        && same_chars.values().all(|&v| v == 2 || v == 1)
    {
        // Two pair, where two cards share one label, two other cards share a
        // second label, and the remaining card has a third label: 23432
        CamelCardHandType::TwoPair
    } else if same_chars.len() == 4
        && same_chars.values().all(|&v| v == 2 || v == 1)
    {
        // One pair, where two cards share one label, and the other three cards
        // have a different label from the pair and each other: A23A4
        CamelCardHandType::OnePair
    } else {
        // High card, where all cards' labels are distinct: 23456
        CamelCardHandType::HighCard
    };

    Ok(CamelCardHandWithJoker {
        cards,
        bid,
        hand_type,
    })
}

/// Determine if the cards have the same characters, and how many there are.
fn same_chars(chars: &[Card; 5]) -> HashMap<&Card, usize> {
    let mut char_count = HashMap::new();

    for ch in chars {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    char_count
}

/// Determine if the cards have the same characters, and how many there are.
fn same_chars_with_joker(
    chars: &[CardWithJoker; 5],
) -> HashMap<&CardWithJoker, usize> {
    let mut char_count = HashMap::new();

    for ch in chars {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    // Check if we have a joker.
    if let Some(joker_count) = char_count.clone().get(&CardWithJoker::J) {
        // The fact we could have all jokers took me WAY too long to realize.
        if joker_count != &5 {
            // Remove the joker.
            char_count.remove(&CardWithJoker::J);

            // Add the joker to whatever card we have the most of.
            let max_count = char_count.values().max().unwrap_or(&0);
            for (ch, count) in char_count.iter() {
                if count == max_count {
                    // Add the joker to this card.
                    *char_count.entry(ch).or_insert(0) += joker_count;
                    break;
                }
            }
        }
    }

    char_count
}

/// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship (<https://en.wikipedia.org/wiki/Airship>). (At least it's a *cool* airship!) It drops you off at the edge of a vast desert and descends back to Island Island.
///
/// "Did you bring the parts?"
///
/// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel (<https://en.wikipedia.org/wiki/Dromedary>).
///
/// "Did you bring the parts?" she asks again, louder this time. You aren't sure
/// what parts she's looking for; you're here to figure out why the sand
/// stopped.
///
/// "The parts! For the sand, yes! Come with me; I will show you." She beckons
/// you onto the camel.
///
/// After riding a bit across the sands of Desert Island, you can see what look
/// like very large rocks covering half of the horizon. The Elf explains that
/// the rocks are all along the part of Desert Island that is directly above
/// Island Island, making it hard to even get there. Normally, they use big
/// machines to move the rocks and filter the sand, but the machines have broken
/// down because Desert Island recently stopped receiving the *parts* they need
/// to fix the machines.
///
/// You've already assumed it'll be your job to figure out why the parts stopped
/// when she asks if you can help. You agree automatically.
///
/// Because the journey will take a few days, she offers to teach you the game of *Camel Cards*. Camel Cards is sort of similar to poker (<https://en.wikipedia.org/wiki/List_of_poker_hands>) except it's designed to be easier to play while riding a camel.
///
/// In Camel Cards, you get a list of *hands*, and your goal is to order them
/// based on the *strength* of each hand. A hand consists of *five cards*
/// labeled one of `A`, `K`, `Q`, `J`, `T`, `9`, `8`, `7`, `6`, `5`, `4`, `3`,
/// or `2`. The relative strength of each card follows this order, where `A` is
/// the highest and `2` is the lowest.
///
/// Every hand is exactly one *type*. From strongest to weakest, they are:
///
/// * *Five of a kind*, where all five cards have the same label: `AAAAA`
/// * *Four of a kind*, where four cards have the same label and one card has a
///   different label: `AA8AA`
/// * *Full house*, where three cards have the same label, and the remaining two
///   cards share a different label: `23332`
/// * *Three of a kind*, where three cards have the same label, and the
///   remaining two cards are each different from any other card in the hand:
///   `TTT98`
/// * *Two pair*, where two cards share one label, two other cards share a
///   second label, and the remaining card has a third label: `23432`
/// * *One pair*, where two cards share one label, and the other three cards
///   have a different label from the pair and each other: `A23A4`
/// * *High card*, where all cards' labels are distinct: `23456`
///
/// Hands are primarily ordered based on type; for example, every *full house*
/// is stronger than any *three of a kind*.
///
/// If two hands have the same type, a second ordering rule takes effect. Start
/// by comparing the *first card in each hand*. If these cards are different,
/// the hand with the stronger first card is considered stronger. If the first
/// card in each hand have the *same label*, however, then move on to
/// considering the *second card in each hand*. If they differ, the hand with
/// the higher second card wins; otherwise, continue with the third card in each
/// hand, then the fourth, then the fifth.
///
/// So, `33332` and `2AAAA` are both *four of a kind* hands, but `33332` is
/// stronger because its first card is stronger. Similarly, `77888` and `77788`
/// are both a *full house*, but `77888` is stronger because its third card is
/// stronger (and both hands have the same first and second card).
///
/// To play Camel Cards, you are given a list of hands and their corresponding
/// *bid* (your puzzle input). For example:
///
/// ```ignore
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// This example shows five hands; each hand is followed by its *bid* amount.
/// Each hand wins an amount equal to its bid multiplied by its *rank*, where
/// the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on
/// up to the strongest hand. Because there are five hands in this example, the
/// strongest hand will have rank 5 and its bid will be multiplied by 5.
///
/// So, the first step is to put the hands in order of strength:
///
/// * `32T3K` is the only *one pair* and the other hands are all a stronger
///   type, so it gets rank *1*.
/// * `KK677` and `KTJJT` are both *two pair*. Their first cards both have the
///   same label, but the second card of `KK677` is stronger (`K` vs `T`), so
///   `KTJJT` gets rank *2* and `KK677` gets rank *3*.
/// * `T55J5` and `QQQJA` are both *three of a kind*. `QQQJA` has a stronger
///   first card, so it gets rank *5* and `T55J5` gets rank *4*.
///
/// Now, you can determine the total winnings of this set of hands by adding up
/// the result of multiplying each hand's bid with its rank (`765` \* 1 + `220`
/// \* 2 + `28` \* 3 + `684` \* 4 + `483` \* 5). So the *total winnings* in this
/// example are `*6440*`.
///
/// Find the rank of every hand in your set. *What are the total winnings?*
pub fn solve_part_1(input: &str) -> Result<u32> {
    let mut hands = input
        .lines()
        .map(parse_camel_card_hand)
        .collect::<Result<Vec<_>>>()?;

    // Sort the hands by their hand type and then by their cards.
    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then_with(|| a.cards.cmp(&b.cards))
    });

    // Determine the winnings.
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i as u32 + 1);
    }

    Ok(winnings)
}

/// To make things a little more interesting, the Elf introduces one additional rule. Now, `J` cards are jokers (<https://en.wikipedia.org/wiki/Joker_(playing_card>)) - wildcards that can act like whatever card would make the hand the strongest type possible.
///
/// To balance this, *`J` cards are now the weakest* individual cards, weaker
/// even than `2`. The other cards stay in the same order: `A`, `K`, `Q`, `T`,
/// `9`, `8`, `7`, `6`, `5`, `4`, `3`, `2`, `J`.
///
/// `J` cards can pretend to be whatever card is best for the purpose of
/// determining hand type; for example, `QJJQ2` is now considered *four of a
/// kind*. However, for the purpose of breaking ties between two hands of the
/// same type, `J` is always treated as `J`, not the card it's pretending to be:
/// `JKKK2` is weaker than `QQQQ2` because `J` is weaker than `Q`.
///
/// Now, the above example goes very differently:
///
/// ```ignore
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// * `32T3K` is still the only *one pair*; it doesn't contain any jokers, so
///   its strength doesn't increase.
/// * `KK677` is now the only *two pair*, making it the second-weakest hand.
/// * `T55J5`, `KTJJT`, and `QQQJA` are now all *four of a kind*! `T55J5` gets
///   rank 3, `QQQJA` gets rank 4, and `KTJJT` gets rank 5.
///
/// With the new joker rule, the total winnings in this example are `*5905*`.
///
/// Using the new joker rule, find the rank of every hand in your set. *What are
/// the new total winnings?*
pub fn solve_part_2(input: &str) -> Result<u32> {
    let mut hands = input
        .lines()
        .map(parse_camel_card_hand_with_joker)
        .collect::<Result<Vec<_>>>()?;

    // Sort the hands by their hand type and then by their cards.
    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then_with(|| a.cards.cmp(&b.cards))
    });

    // Determine the winnings.
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i as u32 + 1);
    }

    Ok(winnings)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day07.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 246424613);
    }

    #[test]
    fn test_solve_part_2() {
        // Make sure the sample works.
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(super::solve_part_2(input).unwrap(), 5905);

        // Load the file.
        let input = include_str!("../input/day07.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 248256639);
    }
}
