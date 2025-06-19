import techalib as tb

def test_plus_di_numpy_success(test_numpy_with_generated_data):
    def plus_di(high, low, close):
        return tb.plus_di(high, low, close, 14)

    test_numpy_with_generated_data(
        "plus_di",
        plus_di,
        tb.plus_di_next,
        ["high", "low", "close"],
        ["plus_di"],
        ["prev_plus_di"]
    )

def test_plus_di_pandas_success(test_with_generated_data):
    def plus_di(high, low, close):
        return tb.plus_di(high, low, close, 14)

    test_with_generated_data(
        "plus_di",
        plus_di,
        tb.plus_di_next,
        ["high", "low", "close"],
        ["plus_di"],
        ["prev_plus_di"]
    )

def test_thread_plus_di(thread_test):
    def ad_tx_lambda(data):
        return tb.plus_di(data, data, data, 14, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
