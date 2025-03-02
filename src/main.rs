mod api;
mod filter;
mod recommender;

use clap::{Command, Arg};
use tokio; // 비동기 실행

#[tokio::main]
async fn main() {
    println!(
        r#"
=========================================
백준 문제 추천기
solved.ac API를 이용해 원하는 난이도 & 태그의 문제를 추천합니다.
-----------------------------------------
사용법: 
  cargo run -- --tier <난이도> --tag <태그> [--recommend <추천방식>] [--user <백준 handle>]

예제:
  cargo run -- --tier g --tag greedy --count 3 -- user glnthd02

옵션:
  -t, --tier       문제 난이도 (b, s, g, p, d, r)  [각각 브론즈, 실버, 골드, 플레티넘, 다이아, 루비를 의미]
  -g, --tag        문제 태그 (dp, greedy, math, graph)
  -c, --count      추천받을 숫자를 입력 (1, 2, 3..)
=========================================
"#
    );
    let matches = Command::new("백준 문제 추천기")
        .arg(
            Arg::new("tier")
                .short('t')
                .long("tier")
                .value_name("TIER")
                .help("문제 난이도 (bronze, silver, gold, platinum, diamond, ruby)")
                .required(true),
        )
        .arg(
            Arg::new("tag")
                .short('g')
                .long("tag")
                .value_name("TAG")
                .help("문제 태그 (dp, greedy, math, graph, etc.)")
                .required(true),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("추천받을 문제 개수")
                .default_value("1"),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_name("USER_ID")
                .help("사용자의 백준 ID (푼 문제 제외)")
                .required(false),
        )
        .get_matches();

    let tier = matches.get_one::<String>("tier").unwrap();
    let tag = matches.get_one::<String>("tag").unwrap();
    let count: usize = matches.get_one::<String>("count").unwrap().parse().unwrap_or(1);
    let user_id = matches.get_one::<String>("user");

    let solved_problems = if let Some(user) = user_id {
        match api::fetch_solved_problems(user).await {
            Ok(solved) => solved,
            Err(_) => {
                println!("사용자 정보가 없습니다.");
                vec![]
            }
        }
    } else {
        vec![]
    };

    match api::fetch_problems(tier, tag).await {
        Ok(problems) => {
            let filtered_problems = filter::apply_filters(problems, &solved_problems);
            let recommended = recommender::recommend_problems(filtered_problems, count);

            if recommended.is_empty() {
                println!("추천할 문제가 없습니다.");
            } else {
                println!("🔹 추천 문제:");
                for (title, id) in recommended {
                    println!("- [{}] https://www.acmicpc.net/problem/{}", title, id);
                }
            }
        }
        Err(err) => println!("API 요청 오류: {}", err),
    }
}