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

    pub fn get_daily() {
        let client = reqwest::blocking::Client::new();
        let resp = client.post(LEETCODE_API_ENDPOINT)
        .body(DAILY_CODING_CHALLENGE_QUERY)
        .send();

        match resp {
            Ok(resp) => {
                // dbg!(resp);
                println!("{}", resp.text().unwrap());
            },
            Err(e) => println!("{}", e),
        }

    }
}
