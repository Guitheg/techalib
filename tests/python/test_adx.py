import techalib as tb

def test_adx_numpy_success(test_numpy_with_generated_data):
    def adx(high, low, close):
        return tb.adx(high, low, close, 14)

    test_numpy_with_generated_data(
        "adx",
        adx,
        tb.adx_next,
        ["high", "low", "close"],
        ["adx"],
        ["prev_adx"]
    )

def test_adx_pandas_success(test_with_generated_data):
    def adx(high, low, close):
        return tb.adx(high, low, close, 14)

    test_with_generated_data(
        "adx",
        adx,
        tb.adx_next,
        ["high", "low", "close"],
        ["adx"],
        ["prev_adx"]
    )

def test_thread_adx(thread_test):
    def ad_tx_lambda(data):
        return tb.adx(data, data, data, 14, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
