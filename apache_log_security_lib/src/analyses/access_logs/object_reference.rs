use analyses::access_logs::AccessLog;
use analyses::Incident;
use helper::url;

/// Analyses access log for object reference
///
/// Steps done before detection:
///   1. url decoding
pub fn analyse<'a>(log: &'a AccessLog) -> Option<Incident<'a>> {
    let disallowed = vec!["/etc/", "/tmp/", "/../", "\\system32", ".cgi", "c:", "/test-cgi/", "/bash", ".dll", "/cgi-bin/"];

    let result = disallowed.iter().fold(false, |acc, &x| {
        acc || url::url_decode(&log.path.to_lowercase()).contains(x)
    });

    if result {
        Some(Incident {
            reason: "Object Reference Attack",
            log,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    fn create_log(path: String) -> super::AccessLog {
        let date_time = "2015-2-18T23:16:9.15Z".parse::<DateTime<Utc>>().unwrap();
        super::AccessLog::new(200, "".to_string(), path, date_time, 0)
    }

    #[test]
    #[should_panic]
    fn test_select_00() {
        let log = create_log("SELECT *".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_referece_00() {
        let log = create_log("/../".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_referece_01() {
        let log = create_log("/../\\".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_non_referece_00() {
        let log = create_log("/a.txt".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_non_referece_01() {
        let log = create_log("/./".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_non_referece_02() {
        let log = create_log("/safe/string".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_non_referece_03() {
        let log = create_log("/safe/string?with=query".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_non_referece_04() {
        let log = create_log("/safe/#hash".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_etc_00() {
        let log = create_log("/etc/".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_system_00() {
        let log = create_log("\\system32".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_middle_00() {
        let log = create_log("/dwdaw/\\system32".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_query_00() {
        let log = create_log("/dwdaw?q=\\system32".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_hash_url_00() {
        let log = create_log("/dwdaw#?q=\\system32".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_encoded_00() {
        let log = create_log("%2Fetc/".to_string());
        super::analyse(&log).unwrap();
    }

    #[test]
    fn test_encoded_01() {
        let log = create_log("%2Fetc%2F".to_string());
        super::analyse(&log).unwrap();
    }
}
