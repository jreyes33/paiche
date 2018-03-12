#[derive(Debug)]
enum Kind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Position(File, Rank);

#[derive(Debug)]
struct Piece {
    kind: Kind,
    position: Position,
}

#[derive(Debug)]
struct Game {
    black_pieces: Vec<Piece>,
    white_pieces: Vec<Piece>,
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
            position: Position(file, rank),
        }
    })
}

macro_rules! pieces {
    ($($piece_code:tt),*) => {
        vec![$(piece!($piece_code),)*]
    }
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
