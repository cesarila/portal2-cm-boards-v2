#![allow(unused)]
#![allow(clippy::all)]

use std::collections::HashMap;
use actix_web::{HttpResponse, Error};
use sqlx::{FromRow, Postgres, types::Type};
use sqlx::postgres::PgValueRef;
use sqlx::decode::Decode;
use sqlx::postgres::types::PgRecordDecoder;
use chrono::NaiveDateTime;

/// One-to-one struct for changelog data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Changelog{
    pub id: i64,
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
}
/// All changelog data except for the ID, for table insertion.
#[derive(Serialize, Deserialize)]
pub struct ChangelogInsert{
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
}

/// One-to-one struct for Category data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Categories{
    pub id: i32,
    pub name: String,
    pub map_id: String, 
    pub rules: String,
}

/// One-to-one struct for chapter data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Chapters{
    pub id: i32,
    pub chapter_name: Option<String>,
    pub is_multiplayer: bool,
    pub game_id: i32,
}

/// One-to-one struct for coop_bundled data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct CoopBundled{
    pub id: i64,
    pub p_id1: String,
    pub p_id2: Option<String>,
    pub p1_is_host: Option<bool>,
    pub cl_id1: i64,
    pub cl_id2: Option<i64>,
}

/// One-to-one struct for demo data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Demos{
    pub id: i64,
    pub drive_url: String,
    pub partner_name: Option<String>,
    pub parsed_successfully: bool,
    pub sar_version: Option<String>,
    pub cl_id: i64,
}

/// One-to-one struct for game data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Games{
    pub id: i32,
    pub game_name: String,
}

/// One-to-one struct for map data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Maps{
    pub id: i32,
    pub steam_id: String,
    pub lp_id: String,
    pub name: String,
    pub chapter_id: Option<i32>,
    pub is_public: bool,
}

/// One-to-one struct for user data.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Users{
    pub profile_number: String,
    pub board_name: Option<String>,
    pub steam_name: Option<String>,
    pub banned: bool,
    pub registred: bool,
    pub avatar: Option<String>,
    pub twitch: Option<String>,
    pub youtube: Option<String>,
    pub title: Option<String>,
    pub admin: i32,
    pub donation_amount: Option<String>,
    pub discord_id: Option<String>,
}

#[derive(Serialize, FromRow, Clone)]
pub struct UsersPage{
    pub user_name: String,
    pub avatar: String,
}

/// The minimal data we want for SP map pages to lower bandwidth usage.
#[derive(Serialize, FromRow)]
pub struct SpMap{
    pub timestamp: Option<NaiveDateTime>,
    #[sqlx(rename = "cl_profile_number")]
    pub profile_number: String,
    pub score: i32,
    pub demo_id: Option<i64>,
    pub youtube_id: Option<String>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32, 
    pub user_name: Option<String>,
    pub avatar: Option<String>,
}

/// The minimal data we want for Coop map pages to lower bandwitch usage.
#[derive(Serialize, FromRow, Clone)]
pub struct CoopMap{
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
    pub p1_is_host: Option<bool>,
    pub demo_id1: Option<i64>,
    pub demo_id2: Option<i64>,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub submission1: bool,
    pub submission2: bool,
    pub note1: Option<String>,
    pub note2: Option<String>,
    pub category_id: i32,
    pub user_name1: String,
    pub user_name2: Option<String>,
    pub avatar1: Option<String>,
    pub avatar2: Option<String>,
}

/// Wrapper for the sp map data and the rank/score.
#[derive(Serialize)]
pub struct SpRanked{
    pub map_data: SpMap,
    pub rank: i32,
    pub points: f32,
}

/// Wrapper for the coop map data and the rank/score.
// TODO: Could have nested map_data values that have less info
#[derive(Serialize)]
pub struct CoopRanked{
    pub map_data: CoopMap,
    pub rank: i32,
    pub points: f32,
}


/// The data for the preview page for all SP Maps
#[derive(Serialize, Deserialize, FromRow)]
pub struct SpPreview{
    #[sqlx(rename = "cl_profile_number")]
    pub profile_number: String,
    pub score: i32,
    pub youtube_id: Option<String>,
    pub category_id: i32, 
    pub user_name: String,
    pub map_id: String
}

/// Wrapper for multiple SpPreviews, prevents repeat data (multiple map_name and map_id copies)
#[derive(Serialize, Deserialize)]
pub struct SpPreviews{
    pub map_id: String,
    pub scores: Vec<SpPreview>,
}

/// The data for the preview page for all Coop Maps
#[derive(Serialize, FromRow, Clone)]
pub struct CoopPreview{
    pub profile_number1: String,
    pub profile_number2: Option<String>,
    pub score: i32,
    pub youtube_id1: Option<String>,
    pub youtube_id2: Option<String>,
    pub category_id: i32, 
    pub user_name1: String,
    pub user_name2: Option<String>,

}

/// Wrapper for prevciewing the top 7 for all Coop maps=.
#[derive(Serialize)]
pub struct CoopPreviews{
    pub map_id: String,
    pub scores: Vec<CoopPreview>,
}

/// Changelog Wrapper that contains additional information on the changelog page.
// #[derive(Serialize, FromRow)]
// pub struct ChangelogPage{
//     pub cl: Changelog,
//     pub map_name: String,
//     pub user_name: String,
//     pub avatar: String,
// }
// TODO: rustc issues.
#[derive(Serialize, FromRow)]
pub struct ChangelogPage{
    pub id: i64,
    pub timestamp: Option<NaiveDateTime>,
    pub profile_number: String,
    pub score: i32,
    pub map_id: String,
    pub demo_id: Option<i64>,
    pub banned: bool,
    pub youtube_id: Option<String>,
    pub previous_id: Option<i32>,
    pub coop_id: Option<i64>,
    pub post_rank: Option<i32>,
    pub pre_rank: Option<i32>,
    pub submission: bool,
    pub note: Option<String>,
    pub category_id: i32,
    pub score_delta: Option<i32>,
    pub verified: Option<bool>,
    pub admin_note: Option<String>,
    pub map_name: String,
    pub user_name: String,
    pub avatar: String,
}

/// All the accepted query parameters for the changelog page.
#[derive(Deserialize, Debug)]
pub struct ChangelogQueryParams{
    pub limit: Option<u64>,
    pub nick_name: Option<String>,
    pub profile_number: Option<String>,
    pub chamber: Option<String>,
    pub sp: bool,
    pub coop: bool,
    pub wr_gain: Option<bool>,
    pub has_demo: Option<bool>,
    pub yt: Option<bool>,
}

/// Wrapper to send a profile number as a search result
#[derive(Deserialize, Debug)]
pub struct UserParams{
    pub profile_number: String,
}

/// Wrapper to send a profile number as a search result
#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreParams{
    pub profile_number: String,
    pub score: i32,
}

/// Banned times for SP
#[derive(Serialize, FromRow)]
pub struct SpBanned{
    pub profile_number: String,
    pub score: i32,
}

/// Banned times for Coop
#[derive(Serialize, FromRow)]
pub struct CoopBanned{
    pub profile_number1: String,
    pub profile_number2: String,
    pub score: i32,
}

/// Wrapper for a player's SP PB history.
#[derive(Serialize, Deserialize)]
pub struct SpPbHistory{
    pub user_name: String,
    pub avatar: Option<String>,
    pub pb_history: Option<Vec<Changelog>>,
}