#![cfg(feature = "serde")]

#[allow(unused_imports)]
use standback::prelude::*;
#[allow(deprecated)]
use time::{
    prelude::*, Date, Duration, OffsetDateTime, PrimitiveDateTime, Sign, Time, UtcOffset, Weekday,
};

#[test]
fn time() -> serde_json::Result<()> {
    let original = [Time::midnight(), time!(23:59:59.999_999_999)];
    let serialized = "[[0,0],[86399,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Time; 2]>(serialized)?, original);

    Ok(())
}

#[test]
fn date() -> serde_json::Result<()> {
    let original = [date!(-100_000 - 001), date!(+100_000-366)];
    let serialized = "[[-100000,1],[100000,366]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Date; 2]>(serialized)?, original);

    Ok(())
}

#[test]
fn primitive_date_time() -> serde_json::Result<()> {
    let original = [
        date!(-100_000 - 001).midnight(),
        date!(+100_000-366).with_time(time!(23:59:59.999_999_999)),
    ];
    let serialized = "[[-100000,1,0,0],[100000,366,86399,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(
        serde_json::from_str::<[PrimitiveDateTime; 2]>(serialized)?,
        original
    );

    Ok(())
}

#[test]
fn offset_date_time() -> serde_json::Result<()> {
    let original = [
        date!(-100_000 - 001)
            .midnight()
            .assume_utc()
            .to_offset(offset!(-23:59:59)),
        date!(+100_000-366)
            .with_time(time!(23:59:59.999_999_999))
            .assume_utc()
            .to_offset(offset!(+23:59:59)),
    ];
    let serialized = "[[-100000,1,0,0],[100000,366,86399,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(
        serde_json::from_str::<[OffsetDateTime; 2]>(serialized)?,
        original
    );

    Ok(())
}

#[test]
fn utc_offset() -> serde_json::Result<()> {
    let original = [offset!(-23:59:59), offset!(+23:59:59)];
    let serialized = "[-86399,86399]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(
        serde_json::from_str::<[UtcOffset; 2]>(serialized)?,
        original
    );

    Ok(())
}

#[test]
fn duration() -> serde_json::Result<()> {
    let original = [Duration::min_value(), Duration::max_value()];
    let serialized = "[[-9223372036854775808,-999999999],[9223372036854775807,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Duration; 2]>(serialized)?, original);

    Ok(())
}

#[test]
#[allow(deprecated)]
fn sign() -> serde_json::Result<()> {
    let original = [Sign::Positive, Sign::Zero, Sign::Negative];
    let serialized = "[1,0,-1]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Sign; 3]>(serialized)?, original);

    Ok(())
}

#[test]
fn weekday() -> serde_json::Result<()> {
    let original = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ];
    let serialized = "[1,2,3,4,5,6,7]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Weekday; 7]>(serialized)?, original);

    Ok(())
}
