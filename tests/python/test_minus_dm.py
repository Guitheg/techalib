import techalib as tb

def test_minus_dm_numpy_success(test_numpy_with_generated_data):
    def minus_dm(high, low):
        return tb.minus_dm(high, low, 14)

    test_numpy_with_generated_data(
        "minus_dm",
        minus_dm,
        tb.minus_dm_next,
        ["high", "low"],
        ["minus_dm"],
        ["prev_minus_dm"]
    )

def test_minus_dm_pandas_success(test_with_generated_data):
    def minus_dm(high, low):
        return tb.minus_dm(high, low, 14)

    test_with_generated_data(
        "minus_dm",
        minus_dm,
        tb.minus_dm_next,
        ["high", "low"],
        ["minus_dm"],
        ["prev_minus_dm"]
    )

def test_thread_minus_dm(thread_test):
    def ad_tx_lambda(data):
        return tb.minus_dm(data, data-.5, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
