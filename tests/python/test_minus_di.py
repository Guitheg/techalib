import techalib as tb

def test_minus_di_numpy_success(test_numpy_with_generated_data):
    def minus_di(high, low, close):
        return tb.minus_di(high, low, close, 14)

    test_numpy_with_generated_data(
        "minus_di",
        minus_di,
        tb.minus_di_next,
        ["high", "low", "close"],
        ["minus_di"],
        ["prev_minus_di"]
    )

def test_minus_di_pandas_success(test_with_generated_data):
    def minus_di(high, low, close):
        return tb.minus_di(high, low, close, 14)

    test_with_generated_data(
        "minus_di",
        minus_di,
        tb.minus_di_next,
        ["high", "low", "close"],
        ["minus_di"],
        ["prev_minus_di"]
    )

def test_thread_minus_di(thread_test):

    def ad_tx_lambda(data):
        return tb.minus_di(data, data, data, 14, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
