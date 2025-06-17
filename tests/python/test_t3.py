import techalib as tb

def test_t3_numpy_success(test_numpy_with_generated_data):
    def t3(close):
        return tb.t3(close, 20)

    test_numpy_with_generated_data("t3", t3, tb.t3_next, ["close"], ["t3"])

def test_t3_pandas_success(test_with_generated_data):
    def t3(close):
        return tb.t3(close, 20)

    test_with_generated_data("t3", t3, tb.t3_next, ["close"], ["t3"])


def test_thread_t3(thread_test):
    def t3_tx_lambda(data):
        return tb.t3(data, 20, release_gil = True)

    thread_test(t3_tx_lambda, n_threads=4)
