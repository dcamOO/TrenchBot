use super::common::*;

#[test]
fn sol_conversion_is_exact_and_rejects_invalid_amounts() {
    for (amount, expected) in [
        ("0.000000001", 1),
        ("0.1", 100_000_000),
        ("2.123456789", 2_123_456_789),
        ("18446744073.709551615", u64::MAX),
    ] {
        let mut c = config();
        c.sol_per_trade = amount.into();
        assert_eq!(c.trade_lamports().unwrap(), expected);
    }
    for amount in [
        "0",
        "-1",
        "NaN",
        "0.0000000001",
        "1e-3",
        "18446744073.709551616",
        "1.2.3",
        "",
        ".1",
    ] {
        let mut c = config();
        c.sol_per_trade = amount.into();
        assert!(c.validate().is_err(), "{amount}");
    }
}

#[test]
fn rejects_invalid_configuration() {
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let mut c = config();
        c.stop_loss_percent = bad;
        assert!(c.validate().is_err());
        let mut c = config();
        c.take_profit_percent = bad;
        assert!(c.validate().is_err());
        let mut c = config();
        c.min_ath_market_cap_usd = bad;
        assert!(c.validate().is_err());
    }
    let mut c = config();
    c.stop_loss_percent = 101.0;
    assert!(c.validate().is_err());
    let mut c = config();
    c.max_creator_launches = 0;
    assert!(c.validate().is_err());
    let mut c = config();
    c.max_launch_age_seconds = 0;
    assert!(c.validate().is_err());
}
