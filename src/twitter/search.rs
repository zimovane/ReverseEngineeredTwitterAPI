use serde_json::json;
use std::cmp;

use crate::twitter::auth::BEARER_TOKEN;

use super::{types::Data, API};

const SEARCH_URL: &str = "https://twitter.com/i/api/graphql/nK1dw4oV3k4w5TdtcAdSww/SearchTimeline";

impl API {
    pub async fn search(
        &self,
        query: String,
        limit: u8,
        cursor: String,
    ) -> std::result::Result<Data, reqwest::Error> {
        let limit = cmp::min(50u8, limit);

        let mut variables = json!(
            {
                "rawQuery":     query,
                "count":        limit,
                "querySource":  "typed_query",
                "product":      "Top"
            }
        );
        let features = json!(
            {
                "rweb_lists_timeline_redesign_enabled":                                    true,
                "responsive_web_graphql_exclude_directive_enabled":                        true,
                "verified_phone_label_enabled":                                            false,
                "creator_subscriptions_tweet_preview_api_enabled":                         true,
                "responsive_web_graphql_timeline_navigation_enabled":                      true,
                "responsive_web_graphql_skip_user_profile_image_extensions_enabled":       false,
                "tweetypie_unmention_optimization_enabled":                                true,
                "responsive_web_edit_tweet_api_enabled":                                   true,
                "graphql_is_translatable_rweb_tweet_is_translatable_enabled":              true,
                "view_counts_everywhere_api_enabled":                                      true,
                "longform_notetweets_consumption_enabled":                                 true,
                "responsive_web_twitter_article_tweet_consumption_enabled":                false,
                "tweet_awards_web_tipping_enabled":                                        false,
                "freedom_of_speech_not_reach_fetch_enabled":                               true,
                "standardized_nudges_misinfo":                                             true,
                "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
                "longform_notetweets_rich_text_read_enabled":                              true,
                "longform_notetweets_inline_media_enabled":                                true,
                "responsive_web_media_download_video_enabled":                             false,
                "responsive_web_enhance_cards_enabled":                                    false,
            }
        );
        let field_toggles = json!(
            {
                "withArticleRichContentState": false
            }
        );
        if cursor.ne("") {
            variables["cursor"] = cursor.into();
        }
        variables["product"] = "Latest".into();
        let q = [
            ("variables", variables.to_string()),
            ("features", features.to_string()),
            ("fieldToggles", field_toggles.to_string()),
        ];

        let req = self
            .client
            .get(SEARCH_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .query(&q)
            .build()
            .unwrap();
        let text = self
            .client
            .execute(req)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:?}", text);

        let res: Data = serde_json::from_str(&text).unwrap();
        return Ok(res);
    }
}