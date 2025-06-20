pub(crate) mod indicators;

criterion::criterion_main! {
indicators::bench_aroonosc::bench,indicators::bench_aroon::bench,indicators::bench_rocr::bench,indicators::bench_adx::bench,indicators::bench_dx::bench,indicators::bench_plus_di::bench,indicators::bench_minus_di::bench,indicators::bench_plus_dm::bench,indicators::bench_minus_dm::bench,indicators::bench_ad::bench,indicators::bench_atr::bench,indicators::bench_roc::bench,indicators::bench_midprice::bench,indicators::bench_midpoint::bench,indicators::bench_kama::bench,indicators::bench_t3::bench,indicators::bench_trima::bench,indicators::bench_tema::bench,indicators::bench_dema::bench,indicators::bench_wma::bench,    indicators::bench_bbands::bench,
    indicators::bench_ema::bench,
    indicators::bench_sma::bench,
    indicators::bench_rsi::bench,
    indicators::bench_macd::bench
}
