use spade::{DelaunayTriangulation, InsertionError, Point2, Triangulation};

// TODO: Copy and work through examples in the spade documentation to learn a bit more rust

pub fn test_1() -> Result<(), InsertionError> {
    println!("Test this bad boi out!");

    //
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    // Insert three vertices that span one triangle (face)
    triangulation.insert(Point2::new(0.0, 1.0))?;
    triangulation.insert(Point2::new(1.0, 1.0))?;
    triangulation.insert(Point2::new(0.5, -1.0))?;

    // Checks that 3 points on a plane are detected as 3 vertices, 1 face, and 3 edges.
    assert_eq!(triangulation.num_vertices(), 3);
    assert_eq!(triangulation.num_inner_faces(), 1);
    assert_eq!(triangulation.num_undirected_edges(), 3);
    Ok(())
}
