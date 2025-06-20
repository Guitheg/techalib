import techalib as tb

def test_trima_numpy_success(test_numpy_with_generated_data):
    def trima(close):
        return tb.trima(close, 30)

    test_numpy_with_generated_data("trima", trima, tb.trima_next, ["close"], ["trima"])

def test_trima_pandas_success(test_with_generated_data):
    def trima(close):
        return tb.trima(close, 30)

    test_with_generated_data("trima", trima, tb.trima_next, ["close"], ["trima"])

def test_thread_trima(thread_test):
    def trima_tx_lambda(data):
        return tb.trima(data, 30, release_gil = True)

    thread_test(trima_tx_lambda, n_threads=4)
