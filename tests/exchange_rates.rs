use regex::Regex;
use time::Duration;

#[tokio::test]
async fn test_get_latest_eur_exchange_rate() {
    let rate = nbp_api::exchange_rates::get_latest_rate(iso_currency::Currency::EUR)
        .await
        .unwrap();

    let table_regex = Regex::new("^\\d{3}/[ABC]/NBP/\\d{4}$").unwrap();
    assert!(table_regex.is_match(&rate.table_number));
    assert!(rate.date <= time::OffsetDateTime::now_utc().date());
    assert!(rate.date > time::OffsetDateTime::now_utc().date() - Duration::days(7));
    assert!(rate.rate > rust_decimal::Decimal::ONE);
    assert!(rate.rate < rust_decimal::Decimal::TEN);
}
