NBP API for Rust
================

Rust client for the [NBP Web API](http://api.nbp.pl/).

Usage
-----

```rust
let rate = nbp_api::exchange_rates::get_latest_rate(iso_currency::Currency::EUR)
    .await
    .unwrap();

println!("EUR exchange rate for {}: {}; table number: {}", rate.date, rate.rate, rate.table_number);
```
