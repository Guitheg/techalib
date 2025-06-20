import techalib as tb

def test_atr_numpy_success(test_numpy_with_generated_data):
    def atr(high, low, close):
        return tb.atr(high, low, close, 14)
    test_numpy_with_generated_data("atr", atr, tb.atr_next, ["high", "low", "close"], ["atr"])

def test_atr_pandas_success(test_with_generated_data):
    def atr(high, low, close):
        return tb.atr(high, low, close, 14)
    test_with_generated_data("atr", atr, tb.atr_next, ["high", "low", "close"], ["atr"])

def test_thread_atr(thread_test):
    def atr_tx_lambda(data):
        return tb.atr(data + 2.0, data - 2.0, data, 14, release_gil = True)

    thread_test(atr_tx_lambda, n_threads=4)
