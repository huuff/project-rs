use project::build_system::BuildSystem;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let project_build_systems = BuildSystem::detect("./")?;

    for build_system in project_build_systems {
        println!("{:?}", build_system);
    }

    Ok(())
}
