fn main() {
    println!("First implementation of my chess");
    let point1 = Point::new();
    println!("{:?}", point1)
}


#[derive(Debug)]
enum Shape{
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King, 
    None
}


#[derive(Debug)]
struct Board{
    
}

impl Board {
    fn new() -> Self {
        Board{}
    }
}

enum Color {
    White,
    Black
}

#[derive(Debug)]
struct Point{
    x: u8,
    y: u8,
    shape: Shape,
}

impl Point{
    fn new() -> Self{
        Point{ x: 0, y: 0, shape:Shape::King }
    }
}