extern crate serde_json;

pub mod endpoints {
    use serde_json::Value;

    const LEETCODE_API_ENDPOINT: &str = "https://leetcode.com/graphql";

    const QUESTION_OF_TODAY_QUERY: &str = "
    query questionOfToday {
        activeDailyCodingChallengeQuestion {
            date
            userStatus
            link
            question {
                acRate
                difficulty
                freqBar
                frontendQuestionId: questionFrontendId
                isFavor
                paidOnly: isPaidOnly
                status
                title
                titleSlug
                hasVideoSolution
                hasSolution
                topicTags {
                    name
                    id
                    slug
                }
            }
        }
    }";

    pub fn get_daily(csrftoken: &str) -> Result<(), reqwest::Error> {
        let cookie = format!("csrftoken={};", csrftoken);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("referer", "https://leetcode.com/problemset/all/".parse().unwrap());
        headers.insert("cookie", cookie.parse().unwrap());
        headers.insert("x-csrftoken", format!("{}", csrftoken).parse().unwrap());
        headers.insert("content-type", "application/json".parse().unwrap());

        let mut map = std::collections::HashMap::new();
        map.insert("query", QUESTION_OF_TODAY_QUERY);
        map.insert("variables", "{}");

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;
        
        let resp = client.post(LEETCODE_API_ENDPOINT)
        .json(&map)
        .send()?;

        let v: Value = match serde_json::from_str(&resp.text()?) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse response to json \n {}", error),
        };

        dbg!(&v);

        let link = &v["data"]["activeDailyCodingChallengeQuestion"]["link"];
        println!("Link: {}", link);

        Ok(())
    }
}
