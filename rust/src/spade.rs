use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};

// TODO: Copy and work through examples in the spade documentation to learn a bit more rust

pub fn test() -> Result<(), InsertionError> {
    println!("Test this bad boi out!");

    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    // Insert three vertices that span one triangle (face)
    triangulation.insert(Point2::new())
}
