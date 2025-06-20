import techalib as tb

def test_midprice_numpy_success(test_numpy_with_generated_data):
    def midprice(high, low):
        return tb.midprice(high, low, 14)

    test_numpy_with_generated_data("midprice", midprice, tb.midprice_next, ["high", "low"], ["midprice"])

def test_midprice_pandas_success(test_with_generated_data):
    def midprice(high, low):
        return tb.midprice(high, low, 14)

    test_with_generated_data("midprice", midprice, tb.midprice_next, ["high", "low"], ["midprice"])

def test_thread_midprice(thread_test):
    def midprice_tx_lambda(data):
        return tb.midprice(data, data - 2.0, 14, release_gil = True)

    thread_test(midprice_tx_lambda, n_threads=4)
