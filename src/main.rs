use std::collections::HashSet;

#[derive(Debug)]
enum Kind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Square(File, Rank);

#[derive(Debug)]
struct Piece {
    kind: Kind,
    square: Square,
}

#[derive(Debug)]
struct Game {
    black_pieces: Vec<Piece>,
    white_pieces: Vec<Piece>,
}

macro_rules! square {
    ($piece_code:tt) => ({
        let chars: Vec<char> = stringify!($piece_code).chars().collect();
        let file = match chars[0] {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file (valid values are a-h)"),
        };
        let rank = match chars[1] {
            '1' => Rank::R1,
            '2' => Rank::R2,
            '3' => Rank::R3,
            '4' => Rank::R4,
            '5' => Rank::R5,
            '6' => Rank::R6,
            '7' => Rank::R7,
            '8' => Rank::R8,
            _ => panic!("Invalid rank (valid values are 1-8)"),
        };
        Square(file, rank)
    });
}

macro_rules! squares {
    ($($square_name:tt),*) => {
        vec![$(square!($square_name),)*].into_iter().collect::<HashSet<_>>()
    };
    ($($square_name:tt),+ ,) => {
        squares![$($square_name),*]
    };
}

macro_rules! piece {
    ($piece_code:tt) => ({
        let chars: Vec<char> = stringify!($piece_code).chars().collect();
        let kind = match chars[0] {
            'R' => Kind::Rook,
            'Q' => Kind::Queen,
            'K' => Kind::King,
            'B' => Kind::Bishop,
            'N' => Kind::Knight,
            _   => Kind::Pawn,
        };
        let pawn_offset = match kind { Kind::Pawn => 0, _ => 1 };
        // TODO: deduplicate with square!
        let file = match chars[pawn_offset + 0] {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file (valid values are a-h)"),
        };
        let rank = match chars[pawn_offset + 1] {
            '1' => Rank::R1,
            '2' => Rank::R2,
            '3' => Rank::R3,
            '4' => Rank::R4,
            '5' => Rank::R5,
            '6' => Rank::R6,
            '7' => Rank::R7,
            '8' => Rank::R8,
            _ => panic!("Invalid rank (valid values are 1-8)"),
        };
        Piece {
            kind: kind,
            square: Square(file, rank),
        }
    });
}

macro_rules! pieces {
    ($($piece_code:tt),*) => {
        vec![$(piece!($piece_code),)*]
    };
}

fn main() {
    let game = Game {
        black_pieces: pieces![
            Ra8, Nb8, Bc8, Qd8, Ke8, Bf8, Ng8, Rh8, a7, b7, c7, d7, e7, f7, g7, h7
        ],
        white_pieces: pieces![
            Ra1, Nb1, Bc1, Qd1, Ke1, Bf1, Ng1, Rh1, a2, b2, c2, d2, e2, f2, g2, h2
        ],
    };
    println!("Hello, Paiche! {:?}", game);
}

fn get_moves(piece: Piece) -> HashSet<Square> {
    #[rustfmt_skip]
    let all_squares = squares![
        a8, b8, c8, d8, e8, f8, g8, h8,
        a7, b7, c7, d7, e7, f7, g7, h7,
        a6, b6, c6, d6, e6, f6, g6, h6,
        a5, b5, c5, d5, e5, f5, g5, h5,
        a4, b4, c4, d4, e4, f4, g4, h4,
        a3, b3, c3, d3, e3, f3, g3, h3,
        a2, b2, c2, d2, e2, f2, g2, h2,
        a1, b1, c1, d1, e1, f1, g1, h1,
    ];
    all_squares
        .into_iter()
        .filter(|s| piece.square != *s && (piece.square.0 == s.0 || piece.square.1 == s.1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_moves_for_rooks() {
        let moves_from_a1 = squares![a2, a3, a4, a5, a6, a7, a8, b1, c1, d1, e1, f1, g1, h1];
        assert_eq!(moves_from_a1, get_moves(piece!(Ra1)));
        let moves_from_e4 = squares![e1, e2, e3, e5, e6, e7, e8, a4, b4, c4, d4, f4, g4, h4];
        assert_eq!(moves_from_e4, get_moves(piece!(Re4)));
    }
}
