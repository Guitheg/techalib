import techalib as tb

def test_ema_numpy_success(test_numpy_with_generated_data):
    def ema(close):
        return tb.ema(close, 30)

    test_numpy_with_generated_data("ema", ema, tb.ema_next, ["close"], ["ema"])

def test_ema_pandas_success(test_with_generated_data):
    def ema(close):
        return tb.ema(close, 30)

    test_with_generated_data("ema", ema, tb.ema_next, ["close"], ["ema"])


def test_thread_ema(thread_test):
    def ema_tx_lambda(data):
        return tb.ema(data, 30, release_gil = True)

    thread_test(ema_tx_lambda, n_threads=4)
