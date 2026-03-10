use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum HandRanking {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
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

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

struct Showdown {
    cards: [Card; 7],
}

impl Showdown {
    pub fn new(board: [Card; 5], hole: [Card; 2]) -> Self {
        let mut buf = [
            board[0], board[1], board[2], board[3], board[4], hole[0], hole[1],
        ];
        buf.sort();
        buf.reverse();

        Self { cards: buf }
    }

    pub fn evaluate(&self) -> (HandRanking, Vec<Card>) {
        if let Some(combination) = self.get_royal_flush() {
            return (HandRanking::RoyalFlush, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_straight_flush() {
            return (HandRanking::StraightFlush, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_four_of_a_kind() {
            return (HandRanking::FourOfAKind, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_full_house() {
            return (HandRanking::FullHouse, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_flush() {
            return (HandRanking::Flush, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_straight() {
            return (HandRanking::Straight, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_three_of_a_kind() {
            return (HandRanking::ThreeOfAKind, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_two_pair() {
            return (HandRanking::TwoPair, self.add_kickers(combination));
        }
        if let Some(combination) = self.get_pair() {
            return (HandRanking::Pair, self.add_kickers(combination));
        }

        (HandRanking::HighCard, self.cards[..5].to_vec())
    }

    fn get_royal_flush(&self) -> Option<Vec<Card>> {
        let straight_flush = self.get_straight_flush()?;
        if straight_flush[0].rank == Rank::Ace {
            return Some(straight_flush);
        }

        None
    }

    fn get_straight_flush(&self) -> Option<Vec<Card>> {
        let suited = self.get_suited()?;
        let straights = Self::find_straights(&suited)?;

        Some(straights[0].clone())
    }

    fn get_four_of_a_kind(&self) -> Option<Vec<Card>> {
        self.get_duplicates(4, 4, None)
    }

    fn get_full_house(&self) -> Option<Vec<Card>> {
        let triple = self.get_duplicates(3, 3, None)?;
        let triple_rank = triple[0].rank;
        let pair = self.get_duplicates(2, 2, Some(triple_rank));
        let pair = match pair {
            Some(p) => p,
            None => self.get_duplicates(3, 2, Some(triple_rank))?,
        };

        let mut ret = triple;
        ret.extend(pair);

        Some(ret)
    }

    fn get_flush(&self) -> Option<Vec<Card>> {
        let suited = self.get_suited()?;

        Some(suited[..5].to_vec())
    }

    fn get_straight(&self) -> Option<Vec<Card>> {
        let straights = Self::find_straights(&self.cards)?;

        Some(straights[0].clone())
    }

    fn get_three_of_a_kind(&self) -> Option<Vec<Card>> {
        self.get_duplicates(3, 3, None)
    }

    fn get_two_pair(&self) -> Option<Vec<Card>> {
        self.get_duplicates(2, 4, None)
    }

    fn get_pair(&self) -> Option<Vec<Card>> {
        self.get_duplicates(2, 2, None)
    }

    fn add_kickers(&self, mut combination: Vec<Card>) -> Vec<Card> {
        if combination.len() >= 5 {
            return combination;
        }

        for card in self.cards {
            if combination.len() < 5 && !combination.contains(&card) {
                combination.push(card);
            }
        }

        combination
    }

    fn get_suited(&self) -> Option<Vec<Card>> {
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

        for suit in &suits {
            if let Some(ret) = self.find_by_suit(*suit)
                && ret.len() >= 5
            {
                return Some(ret);
            }
        }

        None
    }

    fn get_duplicates(
        &self,
        count: u8,
        min_cards: usize,
        exclude_rank: Option<Rank>,
    ) -> Option<Vec<Card>> {
        let mut duplicated: Vec<Card> = vec![];

        for card in self.cards {
            if exclude_rank == Some(card.rank) {
                continue;
            }

            if self.count(card.rank) == count {
                duplicated.push(card);
            }
        }

        if duplicated.len() < min_cards {
            return None;
        }

        Some(duplicated[..min_cards].to_vec())
    }

    fn find_by_suit(&self, suit: Suit) -> Option<Vec<Card>> {
        let mut ret = vec![];

        for card in self.cards {
            if card.suit == suit {
                ret.push(card);
            }
        }

        if ret.is_empty() {
            return None;
        }

        Some(ret)
    }

    fn count(&self, rank: Rank) -> u8 {
        let mut count = 0;
        for card in self.cards {
            if card.rank == rank {
                count += 1;
            }
        }

        count
    }

    fn find_straights(cards: &[Card]) -> Option<Vec<Vec<Card>>> {
        let mut unique = vec![cards[0]];
        for card in &cards[1..] {
            let last_rank = unique[unique.len() - 1].rank;
            if card.rank != last_rank {
                unique.push(*card);
            }
        }

        if unique.len() < 5 {
            return None;
        }

        let mut rows: Vec<Vec<Card>> = vec![];
        for i in 0..=(unique.len() - 5) {
            let mut prev_rank = unique[i].rank;
            let mut temp: Vec<Card> = vec![unique[i]];

            for card in &unique[(i + 1)..(i + 5)] {
                if prev_rank as u8 - card.rank as u8 == 1 {
                    temp.push(*card);
                    prev_rank = card.rank;
                }
            }

            if temp.len() == 5 {
                rows.push(temp);
            }
        }

        let wheel_ranks = [Rank::Five, Rank::Four, Rank::Three, Rank::Two, Rank::Ace];
        let mut wheel = vec![];
        for rank in &wheel_ranks {
            if let Some(ranked_cards) = Self::find_by_rank(&unique, *rank) {
                wheel.push(ranked_cards[0]);
            }
        }
        if wheel.len() == 5 {
            rows.push(wheel);
        }

        if rows.is_empty() {
            return None;
        }
        Some(rows)
    }

    fn find_by_rank(cards: &[Card], rank: Rank) -> Option<Vec<Card>> {
        let mut ret = vec![];

        for card in cards {
            if card.rank == rank {
                ret.push(*card);
            }
        }

        if ret.is_empty() {
            return None;
        }

        Some(ret)
    }
}

fn main() {
    let board = [
        Card {
            rank: Rank::Ten,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Jack,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Queen,
            suit: Suit::Spades,
        },
        Card {
            rank: Rank::King,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Two,
            suit: Suit::Clubs,
        },
    ];

    let hole1 = [
        Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Three,
            suit: Suit::Clubs,
        },
    ];

    let hole2 = [
        Card {
            rank: Rank::Nine,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Eight,
            suit: Suit::Spades,
        },
    ];

    let score1 = Showdown::new(board, hole1).evaluate();
    let score2 = Showdown::new(board, hole2).evaluate();

    match score1.cmp(&score2) {
        Ordering::Greater => println!("Player 1 wins with {:?} against {:?}!", score1.0, score2.0),
        Ordering::Less => println!("Player 2 wins with {:?} against {:?}!", score2.0, score1.0),
        Ordering::Equal => println!("Split pot!"),
    }
}
