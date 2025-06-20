import techalib as tb

def test_plus_dm_numpy_success(test_numpy_with_generated_data):
    def plus_dm(high, low):
        return tb.plus_dm(high, low, 14)

    test_numpy_with_generated_data(
        "plus_dm",
        plus_dm,
        tb.plus_dm_next,
        ["high", "low"],
        ["plus_dm"],
        ["prev_plus_dm"]
    )

def test_plus_dm_pandas_success(test_with_generated_data):
    def plus_dm(high, low):
        return tb.plus_dm(high, low, 14)

    test_with_generated_data(
        "plus_dm",
        plus_dm,
        tb.plus_dm_next,
        ["high", "low"],
        ["plus_dm"],
        ["prev_plus_dm"]
    )

def test_thread_plus_dm(thread_test):
    def ad_tx_lambda(data):
        return tb.plus_dm(data, data-.5, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
