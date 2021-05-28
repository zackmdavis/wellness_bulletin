use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActivityDaySummary {
    pub met_min_high: i64,
    pub cal_active: i64,
    pub score_training_volume: i64,
    pub steps: i64,
    pub score: i64,
    pub score_training_frequency: i64,
    pub target_calories: i64,
    pub day_end: String,
    pub score_move_every_hour: i64,
    pub class_5min: String,
    pub target_km: f64,
    pub cal_total: i64,
    pub daily_movement: i64,
    pub day_start: String,
    pub target_miles: f64,
    pub score_recovery_time: i64,
    pub inactivity_alerts: i64,
    pub high: i64,
    pub met_min_low: i64,
    pub inactive: i64,
    pub score_meet_daily_targets: i64,
    pub total: i64,
    pub met_min_medium: i64,
    pub summary_date: String,
    pub score_stay_active: i64,
    pub average_met: f64,
    pub non_wear: i64,
    pub to_target_km: f64,
    pub rest: i64,
    pub to_target_miles: f64,
    pub medium: i64,
    pub low: i64,
    pub timezone: i64,
    pub met_1min: Vec<f64>,
    pub rest_mode_state: i64,
    pub met_min_inactive: i64,
}

#[derive(Debug, Deserialize)]
pub struct ActivityResponse {
    activity: Vec<ActivityDaySummary>,
}
