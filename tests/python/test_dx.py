import techalib as tb

def test_dx_numpy_success(test_numpy_with_generated_data):
    def dx(high, low, close):
        return tb.dx(high, low, close, 14)

    test_numpy_with_generated_data(
        "dx",
        dx,
        tb.dx_next,
        ["high", "low", "close"],
        ["dx"],
        ["prev_dx"],
        rtol=1e-5,
    )

def test_dx_pandas_success(test_with_generated_data):
    def dx(high, low, close):
        return tb.dx(high, low, close, 14)

    test_with_generated_data(
        "dx",
        dx,
        tb.dx_next,
        ["high", "low", "close"],
        ["dx"],
        ["prev_dx"],
        rtol=1e-5,
    )

def test_thread_dx(thread_test):
    def ad_tx_lambda(data):
        return tb.dx(data, data, data, 14, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
