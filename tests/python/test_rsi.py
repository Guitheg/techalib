import techalib as tb

def test_rsi_numpy_success(test_numpy_with_generated_data):
    def rsi(close):
        return tb.rsi(close, 14)

    test_numpy_with_generated_data("rsi", rsi, tb.rsi_next, ["close"], ["rsi"])

def test_rsi_pandas_success(test_with_generated_data):
    def rsi(close):
        return tb.rsi(close, 14)

    test_with_generated_data("rsi", rsi, tb.rsi_next, ["close"], ["rsi"])

def test_thread_rsi(thread_test):
    def rsi_tx_lambda(data):
        return tb.rsi(data, 30, release_gil = True)

    thread_test(rsi_tx_lambda, n_threads=4)
