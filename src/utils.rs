use yaml_rust::yaml::Yaml;


pub fn str_vec_from_yaml_vec<'a>(need_vec: &'a Yaml) -> Vec<&'a str> {
    let result: Vec<&str> = need_vec
    .as_vec()
    .unwrap()
    .iter()
    .map(|e| {
        e.as_str().unwrap()
    })
    .collect();

    result
}
