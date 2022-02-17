fn main() {
    println!("First implementation of my chess");
    let board = Board::new();
    println!("{:#?}", board);

}


#[derive(Debug)]
enum ShapeType{
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King, 
    None
}
#[derive(Debug)]
struct Shape{
    position:Point,
    isFirstMove:bool,
    name:ShapeType,
    color:Color
}


#[derive(Debug)]
struct Board{
    shapes:Vec<Shape>
}

impl Board {
    fn new() -> Self {
        let mut shapes = Vec::new();
        let mut white_pawns:Vec<Shape> = (0..8).map(|i| Shape{
            position:Point{x:i, y:1},
            isFirstMove: true,
            name: ShapeType::Pawn,
            color: Color::White
        }).collect();
        shapes.append(&mut white_pawns);

        let mut black_pawns:Vec<Shape> = (0..8).map(|i| Shape{
            position:Point{x:i, y:6},
            isFirstMove: true,
            name: ShapeType::Pawn,
            color: Color::Black
        }).collect();

        shapes.append(&mut black_pawns);

        
        Board{shapes}
    }
}

#[derive(Debug)]
enum Color {
    White,
    Black
}

#[derive(Debug)]
struct Point{
    x: u8, // horizontal
    y: u8  // vertical
}