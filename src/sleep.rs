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
    pub hr_lowest: f64,
    pub hr_average: f64,
    pub rmssd: i64,
    pub breath_average: f64, // example was an int
    pub temperature_delta: f64,
    pub hypnogram_5min: String,
    pub hr_5min: Vec<i64>,
    pub rmssd_5min: Vec<i64>,
}

pub fn legible_duration(seconds: i64) -> String {
    format!(
        "{} hours, {} minutes",
        seconds / 3600,
        (seconds % 3600) / 60
    )
}

impl SleepNightSummary {
    pub fn short_bulletin(&self) -> String {
        format!("Sleep report (from @ouraring data)! Time in bed: {}; Time asleep: {}; REM sleep: {}; est'd breaths per minute: {} #wellnessbulletin", legible_duration(self.duration), legible_duration(self.total), legible_duration(self.rem), self.breath_average)
    }
}

#[derive(Debug, Deserialize)]
pub struct SleepResponse {
    pub sleep: Vec<SleepNightSummary>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_legible_duration() {
        assert_eq!(legible_duration(25620), "7 hours, 7 minutes");
    }

    #[test]
    fn demo_short_bulletin() {
        let example = SleepNightSummary {
            summary_date: "2017-11-05".to_owned(),
            period_id: 0,
            is_longest: 1,
            timezone: 120,
            bedtime_start: "2017-11-06T02:13:19+02:00".to_owned(),
            bedtime_end: "2017-11-06T08:12:19+02:00".to_owned(),
            score: 70,
            score_total: 57,
            score_disturbances: 83,
            score_efficiency: 99,
            score_latency: 88,
            score_rem: 97,
            score_deep: 59,
            score_alignment: 31,
            total: 20310,
            duration: 21540,
            awake: 1230,
            light: 10260,
            rem: 7140,
            deep: 2910,
            onset_latency: 480,
            restless: 39,
            efficiency: 94,
            midpoint_time: 11010,
            hr_lowest: 49,
            hr_average: 56.375,
            rmssd: 54,
            breath_average: 13.,
            temperature_delta: -0.06,
            hypnogram_5min:
                "443432222211222333321112222222222111133333322221112233333333332232222334"
                    .to_owned(),
            hr_5min: vec![
                0, 53, 51, 0, 50, 50, 49, 49, 50, 50, 51, 52, 52, 51, 53, 58, 60, 60, 59, 58, 58,
                58, 58, 55, 55, 55, 55, 56, 56, 55, 53, 53, 53, 53, 53, 53, 57, 58, 60, 60, 59, 57,
                59, 58, 56, 56, 56, 56, 55, 55, 56, 56, 57, 58, 55, 56, 57, 60, 58, 58, 59, 57, 54,
                54, 53, 52, 52, 55, 53, 54, 56, 0,
            ],
            rmssd_5min: vec![
                0, 0, 62, 0, 75, 52, 56, 56, 64, 57, 55, 78, 77, 83, 70, 35, 21, 25, 49, 44, 48,
                48, 62, 69, 66, 64, 79, 59, 67, 66, 70, 63, 53, 57, 53, 57, 38, 26, 18, 24, 30, 35,
                36, 46, 53, 59, 50, 50, 53, 53, 57, 52, 41, 37, 49, 47, 48, 35, 32, 34, 52, 57, 62,
                57, 70, 81, 81, 65, 69, 72, 64, 0,
            ],
        };
        assert_eq!(example.short_bulletin(), "Sleep report (from @ouraring data)! Time in bed: 5 hours, 59 minutes; Time asleep: 5 hours, 38 minutes; REM sleep: 1 hours, 59 minutes; est\'d breaths per minute: 13 #wellnessbulletin");
    }
}
