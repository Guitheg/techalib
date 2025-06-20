import techalib as tb

def test_wma_numpy_success(test_numpy_with_generated_data):
    def wma(close):
        return tb.wma(close, 30)

    test_numpy_with_generated_data("wma", wma, tb.wma_next, ["close"], ["wma"])

def test_wma_pandas_success(test_with_generated_data):
    def wma(close):
        return tb.wma(close, 30)

    test_with_generated_data("wma", wma, tb.wma_next, ["close"], ["wma"])

def test_thread_wma(thread_test):
    def wma_tx_lambda(data):
        return tb.wma(data, 30, release_gil = True)

    thread_test(wma_tx_lambda, n_threads=4)
