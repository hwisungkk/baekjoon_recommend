use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn recommend_problems(problems: Vec<(String, i32)>, count: usize) -> Vec<(String, i32)> {
    println!("추천 가능한 문제 개수: {}", problems.len());

    if problems.is_empty() {
        println!("문제 리스트가 비어 있습니다.");
        return vec![];
    }

    let mut rng = thread_rng();
    problems.choose_multiple(&mut rng, count).cloned().collect()
}