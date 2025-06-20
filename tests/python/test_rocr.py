import techalib as tb

def test_rocr_numpy_success(test_numpy_with_generated_data):
    def rocr(data):
        return tb.rocr(data, 14)

    test_numpy_with_generated_data(
        "rocr",
        rocr,
        tb.rocr_next,
        ["close"],
        ["rocr"],
        ["prev_rocr"]
    )

def test_rocr_pandas_success(test_with_generated_data):
    def rocr(data):
        return tb.rocr(data, 14)

    test_with_generated_data(
        "rocr",
        rocr,
        tb.rocr_next,
        ["close"],
        ["rocr"],
        ["prev_rocr"]
    )

def test_thread_rocr(thread_test):
    def ad_tx_lambda(data):
        return tb.rocr(data, 14, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
