use rubiks_cube::{self, Cube, Operation};

fn operation_vec_to_string(operations: Vec<Operation>) -> String {
    return operations
        .iter()
        .map(|op| op.to_string())
        .collect::<Vec<_>>()
        .join(" ");
}

fn main() {
    let (cube, applied) = Cube::random(5);
    println!("Applied rotation: {}", operation_vec_to_string(applied));

    let result = rubiks_cube::solve(cube, 5);

    if let Some(result) = result {
        println!("Found: {}", operation_vec_to_string(result));
    } else {
        println!("Not found")
    }
}
