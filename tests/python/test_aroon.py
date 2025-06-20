import techalib as tb
def test_aroon_numpy_success(test_numpy_with_generated_data):
    def aroon(high, low):
        return tb.aroon(high, low, 14)

    test_numpy_with_generated_data(
        "aroon",
        aroon,
        tb.aroon_next,
        ["high", "low"],
        ["aroon_down", "aroon_up"],
        ["prev_aroon_down", "prev_aroon_up"],
        ["aroondown", "aroonup"]
    )

def test_aroon_pandas_success(test_with_generated_data):
    def aroon(high, low):
        return tb.aroon(high, low, 14)

    test_with_generated_data(
        "aroon",
        aroon,
        tb.aroon_next,
        ["high", "low"],
        ["aroon_down", "aroon_up"],
        ["prev_aroon_down", "prev_aroon_up"],
        ["aroondown", "aroonup"]
    )

def test_thread_aroon(thread_test):
    def ad_tx_lambda(data):
        return tb.aroon(data, data, 14, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
