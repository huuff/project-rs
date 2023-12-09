use std::{
    error::Error,
    fs,
};

#[derive(Clone, Copy, Debug)]
pub enum BuildSystem {
    Cargo,
    Npm,
}

const ALL_BUILD_SYSTEMS: [BuildSystem; 2] = [BuildSystem::Cargo, BuildSystem::Npm];

impl BuildSystem {
    pub fn detect(path: &str) -> Result<Vec<BuildSystem>, Box<dyn Error>> {
        // TODO: Maybe log the entries for debugging?
        let entries = fs::read_dir(path)?
            .filter_map(|it| it.ok().map(|it| it.path()))
            .filter_map(|path| {
                path.file_name().map(|file_name| {
                    file_name.to_string_lossy().into_owned()
                })
            })
            .collect::<Vec<String>>()
            ;

        let mut result = vec![];
        for build_system in ALL_BUILD_SYSTEMS {
            if build_system.is_in(&entries) {
                result.push(build_system);
            } 
        }


        Ok(result)
    }

    fn is_in<'a>(&self, paths: &Vec<String>) -> bool {
        // TODO: Use once_cell?
        let target_file = match self {
            BuildSystem::Cargo => "Cargo.toml",
            BuildSystem::Npm => "package.json",
        };

        // TODO: Refactor?
        for path in paths {
            if path == target_file {
                return true;
            }
        }

        false
    }
}
