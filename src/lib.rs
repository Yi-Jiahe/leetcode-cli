pub mod endpoints {
    const LEETCODE_API_ENDPOINT: &str = "https://leetcode.com/graphql";
    const DAILY_CODING_CHALLENGE_QUERY: &str = "
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
        println!("{}", csrftoken);

        let cookie = format!("csrftoken={};", csrftoken);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("referer", "https://leetcode.com/problemset/all/".parse().unwrap());
        headers.insert("cookie", cookie.parse().unwrap());
        headers.insert("x-csrftoken", format!("{};", csrftoken).parse().unwrap());


        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        let resp = client.post(LEETCODE_API_ENDPOINT)
        .body(DAILY_CODING_CHALLENGE_QUERY)
        .send()?;

        println!("{}", resp.text()?);

        Ok(())
    }
}
