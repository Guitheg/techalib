import techalib as tb

def test_kama_numpy_success(test_numpy_with_generated_data):
    def kama(close):
        return tb.kama(close, 30)

    test_numpy_with_generated_data("kama", kama, tb.kama_next, ["close"], ["kama"])

def test_kama_pandas_success(test_with_generated_data):
    def kama(close):
        return tb.kama(close, 30)

    test_with_generated_data("kama", kama, tb.kama_next, ["close"], ["kama"])

def test_thread_kama(thread_test):
    def kama_tx_lambda(data):
        return tb.kama(data, 30, release_gil = True)

    thread_test(kama_tx_lambda, n_threads=4)
