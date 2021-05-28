use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SleepNightSummary {
    pub summary_date: String,
    pub period_id: i64,
    pub is_longest: i64,
    pub timezone: i64,
    pub bedtime_start: String,
    pub bedtime_end: String,
    pub score: i64,
    pub score_total: i64,
    pub score_disturbances: i64,
    pub score_efficiency: i64,
    pub score_latency: i64,
    pub score_rem: i64,
    pub score_deep: i64,
    pub score_alignment: i64,
    pub total: i64,
    pub duration: i64,
    pub awake: i64,
    pub light: i64,
    pub rem: i64,
    pub deep: i64,
    pub onset_latency: i64,
    pub restless: i64,
    pub efficiency: i64,
    pub midpoint_time: i64,
    pub hr_lowest: i64,
    pub hr_average: f64,
    pub rmssd: i64,
    pub breath_average: i64,
    pub temperature_delta: f64,
    pub hypnogram_5min: String,
    pub hr_5min: Vec<i64>,
    pub rmssd_5min: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SleepResponse {
    pub sleep: Vec<SleepNightSummary>,
}
