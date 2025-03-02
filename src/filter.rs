pub fn apply_filters(problems: Vec<(String, i32)>, solved_problems: &[i32]) -> Vec<(String, i32)> {
    problems.into_iter()
        .filter(|(title, id)| {
            let korean = title.chars().any(|c| c >= '\u{AC00}' && c <= '\u{D7A3}');  //한글 문제들만 받도록
            !solved_problems.contains(id) && korean
        })  
        .collect()
}
