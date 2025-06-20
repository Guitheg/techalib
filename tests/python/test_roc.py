import techalib as tb

def test_roc_numpy_success(test_numpy_with_generated_data):
    def roc(close):
        return tb.roc(close, 10)

    test_numpy_with_generated_data("roc", roc, tb.roc_next, ["close"], ["roc"], rtol=1e-6)

def test_roc_pandas_success(test_with_generated_data):
    def roc(close):
        return tb.roc(close, 10)

    test_with_generated_data("roc", roc, tb.roc_next, ["close"], ["roc"], rtol=1e-6)

def test_thread_roc(thread_test):
    def roc_tx_lambda(data):
        return tb.roc(data, 10, release_gil = True)

    thread_test(roc_tx_lambda, n_threads=4)
