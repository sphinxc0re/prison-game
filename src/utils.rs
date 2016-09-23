/// This is a general utils module for abtstraction of simple computations.

use yaml_rust::yaml::Yaml;


pub fn str_vec_from_yaml_vec<'a>(need_vec: Yaml) -> Vec<String> {
    need_vec
    .as_vec()
    .unwrap()
    .iter()
    .map(|e| {
        String::from(e.as_str().unwrap())
    })
    .collect()
}
