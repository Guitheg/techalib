import techalib as tb

def test_dema_numpy_success(test_numpy_with_generated_data):
    def dema(close):
        return tb.dema(close, 30)

    test_numpy_with_generated_data("dema", dema, tb.dema_next, ["close"], ["dema"])

def test_dema_pandas_success(test_with_generated_data):
    def dema(close):
        return tb.dema(close, 30)

    test_with_generated_data("dema", dema, tb.dema_next, ["close"], ["dema"])


def test_thread_dema(thread_test):
    def dema_tx_lambda(data):
        return tb.dema(data, 30, release_gil = True)

    thread_test(dema_tx_lambda, n_threads=4)
