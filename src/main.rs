use rubiks_cube::{self, Cube, Operation};

fn main() {
    let cube = Cube::new()
        .operation_applied(&Operation::R)
        .operation_applied(&Operation::B)
        .operation_applied(&Operation::R2)
        .operation_applied(&Operation::F3);

    let result = rubiks_cube::solve(cube, 10);

    if let Some(result) = result {
        let result = result
            .into_iter()
            .map(|op| op.to_string())
            .collect::<Vec<_>>();
        let result = result.join(" ");

        println!("Found: {result}")
    } else {
        println!("Not found")
    }
}
