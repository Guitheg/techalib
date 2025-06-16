import techalib as tb
from numpy import testing
import numpy as np

def test_ad_numpy_success(test_numpy_with_generated_data):
    def ad(high, low, close, volume):
        return tb.ad(high, low, close, volume)

    test_numpy_with_generated_data("ad", ad, tb.ad_next, ["high", "low", "close", "volume"], ["ad"], rtol=1e-6)

def test_ad_pandas_success(test_with_generated_data):
    def ad(high, low, close, volume):
        return tb.ad(high, low, close, volume)

    test_with_generated_data("ad", ad, tb.ad_next, ["high", "low", "close", "volume"], ["ad"], rtol=1e-6)

def test_thread_ad(thread_test):
    def ad_tx_lambda(data):
        return tb.ad(data + 5.0, data - 5.0, data, data*10, release_gil = True)

    thread_test(ad_tx_lambda, n_threads=4)
