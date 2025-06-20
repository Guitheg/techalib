import techalib as tb
def test_aroonosc_numpy_success(test_numpy_with_generated_data):
    def aroonosc(high, low):
        return tb.aroonosc(high, low, 14)

    test_numpy_with_generated_data(
        "aroonosc",
        aroonosc,
        tb.aroonosc_next,
        ["high", "low"],
        ["aroonosc"],
        ["prev_aroonosc"]
    )

def test_aroonosc_pandas_success(test_with_generated_data):
    def aroonosc(high, low):
        return tb.aroonosc(high, low, 14)

    test_with_generated_data(
        "aroonosc",
        aroonosc,
        tb.aroonosc_next,
        ["high", "low"],
        ["aroonosc"],
        ["prev_aroonosc"]
    )

def test_thread_aroonosc(thread_test):
    def ad_tx_lambda(data):
        return tb.aroonosc(data, data, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
