use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use time::{self, Date, PrimitiveDateTime, Time, format_description::well_known::Rfc3339, macros::format_description};

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
#[serde(default)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub layout: Option<String>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: Option<PrimitiveDateTime>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: Option<PrimitiveDateTime>,
    pub authors: Vec<String>,
    pub extra: HashMap<String, Value>,
}
impl Frontmatter {
    pub fn read(markdown: &str) -> Option<Self> {
        let mut parts = markdown.splitn(3, "---");
        parts.next()?;
        let frontmatter = parts.next()?;
        serde_yaml::from_str(frontmatter).ok()
    }
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        let formats = [
            format_description!("[year]-[month]-[day] [hour]:[minute]"),
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
        ];
        for fmt in &formats {
            if let Ok(dt) = PrimitiveDateTime::parse(s, fmt) {
                return Ok(Some(dt));
            }
        }

        if let Ok(odt) = time::OffsetDateTime::parse(s, &Rfc3339) {
            return Ok(Some(odt.date().with_time(odt.time())));
        }

        if let Ok(date) = Date::parse(s, format_description!("[year]-[month]-[day]")) {
            return Ok(Some(PrimitiveDateTime::new(date, Time::MIDNIGHT)));
        }

        Err(serde::de::Error::custom(format!("Unrecognized datetime format: {}", s)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::from_str;
    use time::macros::datetime;

    #[test]
    fn test_frontmatter() {
        let yaml = r#"
        title: "Test Title"
        description: "Test Description"
        layout: "post.html"
        created_at: 2023-10-01T12:00:00Z
        updated_at: 2023-10-02T12:00:00Z
        "#;
        let frontmatter: Frontmatter = from_str(yaml).unwrap();
        assert_eq!(frontmatter.title, Some("Test Title".to_string()));
        assert_eq!(frontmatter.description, Some("Test Description".to_string()));
        assert_eq!(frontmatter.layout, Some("post.html".to_string()));
        assert_eq!(frontmatter.created_at, Some(datetime!(2023-10-01 12:0:0)));
        assert_eq!(frontmatter.updated_at, Some(datetime!(2023-10-02 12:0:0)));
        assert_eq!(frontmatter.authors, Vec::<String>::new());
        assert_eq!(frontmatter.extra.len(), 0);
    }

    #[test]
    fn test_read_frontmatter() {
        let yaml = r#"
        ---
        title: "Test Title"
        description: "Test Description"
        ---
        # This is a test markdown content
        "#;

        let frontmatter = Frontmatter::read(yaml).unwrap();
        assert_eq!(frontmatter.title, Some("Test Title".to_string()));
        assert_eq!(frontmatter.description, Some("Test Description".to_string()));
    }

    #[derive(Debug, Deserialize, Default)]
    #[serde(default)]
    struct TestWrapper {
        #[serde(deserialize_with = "deserialize_datetime")]
        pub dt: Option<PrimitiveDateTime>,
    }

    #[test]
    fn test_format_ymd() {
        let input = "dt: \"2025-04-30\"";
        let result: TestWrapper = serde_yaml::from_str(input).unwrap();
        let expected = PrimitiveDateTime::new(
            time::Date::from_calendar_date(2025, time::Month::April, 30).unwrap(),
            time::Time::MIDNIGHT,
        );
        assert_eq!(result.dt.unwrap(), expected);
    }

    #[test]
    fn test_format_ymd_hm() {
        let input = "dt: \"2025-04-30 13:45\"";
        let result: TestWrapper = serde_yaml::from_str(input).unwrap();
        let expected = datetime!(2025-04-30 13:45);
        assert_eq!(result.dt.unwrap(), expected);
    }

    #[test]
    fn test_format_ymd_hms() {
        let input = "dt: \"2025-04-30T13:45:30\"";
        let result: TestWrapper = serde_yaml::from_str(input).unwrap();
        let expected = datetime!(2025-04-30 13:45:30);
        assert_eq!(result.dt.unwrap(), expected);
    }

    #[test]
    fn test_format_rfc3339_z() {
        let input = "dt: \"2025-04-30T13:45:30Z\"";
        let result: TestWrapper = serde_yaml::from_str(input).unwrap();
        let expected = datetime!(2025-04-30 13:45:30);
        assert_eq!(result.dt.unwrap(), expected);
    }

    #[test]
    fn test_invalid_format() {
        let input = "dt: \"invalid-date\"";
        let result: Result<TestWrapper, _> = serde_yaml::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_null_datetime() {
        let input = "dt: null";
        let result: TestWrapper = serde_yaml::from_str(input).unwrap();
        assert!(result.dt.is_none());
    }

    #[test]
    fn test_missing_datetime() {
        let input = "";
        let result: TestWrapper = serde_yaml::from_str(input).unwrap();
        assert!(result.dt.is_none());
    }
}
