use reqwest;
use serde_json::Value;

const BASE_URL: &str = "https://solved.ac/api/v3/search/problem";

pub async fn fetch_problems(tier: &str, tag: &str) -> Result<Vec<(String, i32)>, String> {
    let mut problems = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "{}?query=tag%3A{}+*{}&sort=id&direction=desc&page={}", //문제들 
            BASE_URL, tag, tier, page
        );

        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let body = response.text().await.map_err(|e| e.to_string())?;
        let json: Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        if let Some(items) = json["items"].as_array() {
            if items.is_empty() {
                break; // 더 이상 문제가 없으면 중단
            }

            for item in items.iter() {
                if let (Some(title), Some(id)) = (item["titleKo"].as_str(), item["problemId"].as_i64()) {
                    problems.push((title.to_string(), id as i32));
                }
            }
        } else {
            break;
        }

        page += 1;
    }

    if problems.is_empty() {
        Err("문제 목록이 비어 있습니다.".to_string())
    } else {
        Ok(problems)
    }
}
pub async fn fetch_solved_problems(user_id: &str) -> Result<Vec<i32>, String> {
    let mut solved_problems = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://solved.ac/api/v3/search/problem?query=s%40{}&page={}", // 푼 문제 가져오기
            user_id, page
        );

        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let body = response.text().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;

        if let Some(items) = json["items"].as_array() {
            if items.is_empty() {
                break; // 더 이상 푼 문제가 없으면 종료
            }

            for item in items {
                if let Some(problem_id) = item["problemId"].as_i64() {
                    solved_problems.push(problem_id as i32);
                }
            }
        } else {
            break; // "items" 필드가 없으면 종료
        }

        page += 1; // 다음 페이지로 이동
    }

    Ok(solved_problems)
}
