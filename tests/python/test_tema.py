import techalib as tb

def test_tema_numpy_success(test_numpy_with_generated_data):
    def tema(close):
        return tb.tema(close, 30, 0.06451612903225806)

    test_numpy_with_generated_data("tema", tema, tb.tema_next, ["close"], ["tema"])

def test_tema_pandas_success(test_with_generated_data):
    def tema(close):
        return tb.tema(close, 30)

    test_with_generated_data("tema", tema, tb.tema_next, ["close"], ["tema"])


def test_thread_tema(thread_test):
    def tema_tx_lambda(data):
        return tb.tema(data, 30, release_gil = True)

    thread_test(tema_tx_lambda, n_threads=4)
