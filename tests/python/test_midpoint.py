import techalib as tb

def test_midpoint_numpy_success(test_numpy_with_generated_data):
    def midpoint(close):
        return tb.midpoint(close, 14)

    test_numpy_with_generated_data("midpoint", midpoint, tb.midpoint_next, ["close"], ["midpoint"])

def test_midpoint_pandas_success(test_with_generated_data):
    def midpoint(close):
        return tb.midpoint(close, 14)

    test_with_generated_data("midpoint", midpoint, tb.midpoint_next, ["close"], ["midpoint"])

def test_thread_midpoint(thread_test):
    def midpoint_tx_lambda(data):
        return tb.midpoint(data, 14, release_gil = True)

    thread_test(midpoint_tx_lambda, n_threads=4)
