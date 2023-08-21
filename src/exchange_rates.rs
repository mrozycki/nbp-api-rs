use reqwest::StatusCode;

use crate::Error;

mod api {
    use serde::{Deserialize, Deserializer};
    use time::macros::format_description;

    fn deserialize_date<'de, D>(deserializer: D) -> Result<time::Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let format = format_description!("[year]-[month]-[day]");
        time::Date::parse(&s, &format).map_err(serde::de::Error::custom)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(super) struct Rate {
        pub no: String,
        #[serde(deserialize_with = "deserialize_date")]
        pub effective_date: time::Date,
        pub mid: rust_decimal::Decimal,
    }

    #[derive(Deserialize)]
    pub(super) struct RateResponse {
        pub rates: Vec<Rate>,
    }
}

#[derive(Clone, Debug)]
pub struct SingleRateResponse {
    pub date: time::Date,
    pub table_number: String,
    pub rate: rust_decimal::Decimal,
}

/// Returns the latest available exchange rate for the provided currency.
pub async fn get_latest_rate(
    currency: iso_currency::Currency,
) -> Result<SingleRateResponse, Error> {
    let res = reqwest::get(format!(
        "http://api.nbp.pl/api/exchangerates/rates/a/{}",
        currency.code()
    ))
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        Error::ServerError
    })?
    .json::<api::RateResponse>()
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        match e.status() {
            Some(StatusCode::NOT_FOUND) => Error::NoData,
            _ => Error::ServerError,
        }
    })?;
    let rate = res.rates.first().ok_or(Error::NoData)?;

    Ok(SingleRateResponse {
        date: rate.effective_date,
        table_number: rate.no.clone(),
        rate: rate.mid,
    })
}
