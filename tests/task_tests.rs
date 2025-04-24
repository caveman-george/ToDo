use chrono::TimeDelta;
use chrono::prelude::*;
use regex::Regex;
use todo::estimate_due_datetime;
use todo::{CliCommand, Priority, TaskBuilder, TaskStatus};

#[test]
fn test_duration_regex() {
    let re = Regex::new(r"(^[1-9][0-9]*)([mhdw])$").unwrap();
    let should_pass_array: [&str; 10] =
        ["5m", "5h", "5d", "5w", "1m", "1h", "1d", "1w", "10m", "10h"];

    let should_fail_array: [&str; 10] = [
        "0w", "5", "5m5", "5h5", "5d5", "5w5", "1m1", "1h1", "1d1", "10m10",
    ];

    for test_str in should_pass_array.iter() {
        let matches = re.is_match(test_str);
        if !matches {
            println!("Expected to match but failed: {}", test_str);
        }
        assert!(re.is_match(test_str));
    }

    for test_str in should_fail_array.iter() {
        let matches = re.is_match(test_str);
        if matches {
            println!("Expected to fail but matched: {}", test_str);
        }
        assert!(!matches);
    }
}

#[test]
fn test_estimate_due_datetime() {
    let test_cases = vec![
        ("5m", 5 * 60),
        ("5h", 5 * 60 * 60),
        ("5d", 5 * 24 * 60 * 60),
        ("5w", 5 * 7 * 24 * 60 * 60),
        ("1m", 1 * 60),
        ("1h", 1 * 60 * 60),
        ("1d", 1 * 24 * 60 * 60),
        ("1w", 1 * 7 * 24 * 60 * 60),
        ("10m", 10 * 60),
        ("10h", 10 * 60 * 60),
    ];

    for (input, expected_seconds) in test_cases {
        let result = estimate_due_datetime(input);
        let now = Utc::now();
        let expected = now + TimeDelta::seconds(expected_seconds);

        let result_trunc = result
            .naive_utc()
            .date()
            .and_hms_opt(result.hour(), result.minute(), 0)
            .unwrap()
            .and_utc();

        let expected_trunc = expected
            .naive_utc()
            .date()
            .and_hms_opt(expected.hour(), expected.minute(), 0)
            .unwrap()
            .and_utc();

        assert_eq!(
            result_trunc, expected_trunc,
            "For input '{}', expected time close to {:?} but got {:?}",
            input, expected, result
        );
    }
}

#[test]
fn test_cli_command() {
    let args = vec![
        "path/to/robco".to_string(),
        "--new".to_string(),
        "Bring Snacks".to_string(),
    ];

    let bad_args = vec![
        "path/to/robco".to_string(),
        "this-is-not-a-command".to_string(),
        "Bring Snacks".to_string(),
    ];

    // let command = CliCommand::from(args);
    // assert!(matches!(command, CliCommand::NewTask { .. }));
    let bad_command = CliCommand::from(bad_args);
    assert!(matches!(bad_command, None));
}

#[test]
fn test_task_builder() {
    let task = TaskBuilder::new()
        .id(1)
        .title("Bring Snacks".to_string())
        .description("Bring some snacks for the meeting".to_string())
        .due("1h".to_string())
        .build();

    assert!(task.is_ok());
    let task = task.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.title, "Bring Snacks");
    assert_eq!(
        task.description,
        Some("Bring some snacks for the meeting".to_string())
    );
    assert!(matches!(task.priority, Priority::Low));
    assert!(matches!(task.status, TaskStatus::OnGoing));
}
