use {
    crate::*,
    gluesql_core::{data::Interval as I, error::IntervalError, prelude::Value::*},
};

test_case!(interval, {
    let g = get_tester!();

    g.run(
        "
CREATE TABLE IntervalLog (
    id INTEGER,
    interval1 INTERVAL,
    interval2 INTERVAL
)",
    )
    .await;

    g.run(
        "
INSERT INTO IntervalLog VALUES
    (1, INTERVAL '1-2' YEAR TO MONTH,         INTERVAL 30 MONTH),
    (2, INTERVAL 12 DAY,                      INTERVAL '35' HOUR),
    (3, INTERVAL '12' MINUTE,                 INTERVAL 300 SECOND),
    (4, INTERVAL '-3 14' DAY TO HOUR,         INTERVAL '3 12:30' DAY TO MINUTE),
    (5, INTERVAL '3 14:00:00' DAY TO SECOND,  INTERVAL '3 12:30:12.1324' DAY TO SECOND),
    (6, INTERVAL '12:00' HOUR TO MINUTE,      INTERVAL '-12:30:12' HOUR TO SECOND),
    (7, INTERVAL '-1000-11' YEAR TO MONTH,    INTERVAL '-30:11' MINUTE TO SECOND);
",
    )
    .await;

    g.test(
        "SELECT * FROM IntervalLog;",
        Ok(select!(
            id  | interval1           | interval2
            I64 | Interval            | Interval;
            1     I::months(14)         I::months(30);
            2     I::days(12)           I::hours(35);
            3     I::minutes(12)        I::minutes(5);
            4     I::hours(-86)         I::minutes(84 * 60 + 30);
            5     I::minutes(86 * 60)   I::microseconds((((84 * 60) + 30) * 60 + 12) * 1_000_000 + 132_400);
            6     I::hours(12)          I::seconds(-(12 * 3600 + 30 * 60 + 12));
            7     I::months(-12_011)    I::seconds(-(30 * 60 + 11))
        ))
    ).await;

    g.test(
        "SELECT
            id,
            interval1 * 2 AS i1,
            interval2 - INTERVAL '-3' YEAR AS i2
        FROM IntervalLog WHERE id = 1",
        Ok(select!(
            id  | i1            | i2
            I64 | Interval      | Interval;
            1     I::months(28)   I::months(66)
        )),
    )
    .await;

    g.test(
        "SELECT
            id,
            interval1 / 3 AS i1,
            interval2 - INTERVAL 3600 SECOND AS i2,
            INTERVAL (20 + 10) SECOND + INTERVAL (10 * 3) SECOND AS i3
        FROM IntervalLog WHERE id = 2;",
        Ok(select!(
            id  | i1         | i2           | i3
            I64 | Interval   | Interval     | Interval;
            2     I::days(4)   I::hours(34)   I::minutes(1)
        )),
    )
    .await;

    g.test(
        "INSERT INTO IntervalLog VALUES (1, INTERVAL '20:00' MINUTE TO HOUR, INTERVAL '1-2' YEAR TO MONTH)",
        Err(IntervalError::UnsupportedRange("Minute".to_owned(), "Hour".to_owned()).into())
    ).await;

    g.test(
        "SELECT INTERVAL '1' YEAR + INTERVAL '1' HOUR FROM IntervalLog;",
        Err(IntervalError::AddBetweenYearToMonthAndHourToSecond.into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '1' YEAR - INTERVAL '1' HOUR FROM IntervalLog;",
        Err(IntervalError::SubtractBetweenYearToMonthAndHourToSecond.into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '1.4' YEAR FROM IntervalLog;",
        Err(IntervalError::FailedToParseInteger("1.4".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '1.4ab' HOUR FROM IntervalLog;",
        Err(IntervalError::FailedToParseDecimal("1.4ab".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '111:34' HOUR TO MINUTE FROM IntervalLog;",
        Err(IntervalError::FailedToParseTime("111:34".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '111' YEAR TO MONTH FROM IntervalLog;",
        Err(IntervalError::FailedToParseYearToMonth("111".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '111' DAY TO HOUR FROM IntervalLog;",
        Err(IntervalError::FailedToParseDayToHour("111".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '111' DAY TO HOUR FROM IntervalLog;",
        Err(IntervalError::FailedToParseDayToHour("111".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '111' DAY TO MINUTE FROM IntervalLog;",
        Err(IntervalError::FailedToParseDayToMinute("111".to_owned()).into()),
    )
    .await;

    g.test(
        "SELECT INTERVAL '111' DAY TO Second FROM IntervalLog;",
        Err(IntervalError::FailedToParseDayToSecond("111".to_owned()).into()),
    )
    .await;
});
