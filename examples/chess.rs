//! Chess.com
use chromia::{chromia::connect_to_page, client::Client};
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

fn parse_board(client: &mut Client) -> [Option<Piece>; 64] {
    let mut squares: [Option<Piece>; 64] = [None; 64];
    let resp: Value = client.execute("Array.from(document.getElementsByClassName('piece')).map(element => element.className).toString()");
    let text = resp["result"]["value"].as_str().unwrap();
    for square_text in text.split(',') {
        let piece = match &square_text[6..8] {
            "br" => Piece::BlackRook,
            "bn" => Piece::BlackKnight,
            "bb" => Piece::BlackBishop,
            "bk" => Piece::BlackKing,
            "bq" => Piece::BlackQueen,
            "bp" => Piece::BlackPawn,
            "wr" => Piece::WhiteRook,
            "wn" => Piece::WhiteKnight,
            "wb" => Piece::WhiteBishop,
            "wk" => Piece::WhiteKing,
            "wq" => Piece::WhiteQueen,
            "wp" => Piece::WhitePawn,
            x => panic!("Unknown piece {x}"),
        };

        let square_x: usize = square_text[16..17].parse().unwrap();
        let square_y: usize = square_text[17..18].parse().unwrap();
        squares[(square_x - 1) + (8 - square_y) * 8] = Some(piece);
    }
    squares
}

fn main() {
    let mut client = connect_to_page("chess.com", 9222).unwrap();
    let board = parse_board(&mut client);

    // pretty print board
    for (i, square) in board.iter().enumerate() {
        if i % 8 == 0 {
            println!();
        }
        print!(
            "{} ",
            square
                .map(|x| match x {
                    Piece::BlackPawn => "♙",
                    Piece::BlackKnight => "♘",
                    Piece::BlackBishop => "♗",
                    Piece::BlackRook => "♖",
                    Piece::BlackQueen => "♕",
                    Piece::BlackKing => "♔",
                    Piece::WhitePawn => "♟",
                    Piece::WhiteKnight => "♞",
                    Piece::WhiteBishop => "♝",
                    Piece::WhiteRook => "♜",
                    Piece::WhiteQueen => "♛",
                    Piece::WhiteKing => "♚",
                })
                .unwrap_or(" ")
        );
    }
    println!();
}
