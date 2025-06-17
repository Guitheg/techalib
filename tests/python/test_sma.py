import techalib as tb

def test_sma_numpy_success(test_numpy_with_generated_data):
    def sma(close):
        return tb.sma(close, 30)

    test_numpy_with_generated_data("sma", sma, tb.sma_next, ["close"], ["sma"])

def test_sma_pandas_success(test_with_generated_data):
    def sma(close):
        return tb.sma(close, 30)

    test_with_generated_data("sma", sma, tb.sma_next, ["close"], ["sma"])

def test_thread_sma(thread_test):
    def sma_tx_lambda(data):
        return tb.sma(data, 30, release_gil = True)

    thread_test(sma_tx_lambda, n_threads=4)
