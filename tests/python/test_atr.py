import techalib as tb
from numpy import testing
import numpy as np

def test_atr_numpy_success(csv_loader):
    df = csv_loader("atr")
    result = tb.atr(df["high"].iloc[:-1].to_numpy(), df["low"].iloc[:-1].to_numpy(), df["close"].iloc[:-1].to_numpy(), 14)
    final_result = tb.atr(df["high"].to_numpy(), df["low"].to_numpy(), df["close"].to_numpy(), 14)

    next_state = tb.atr_next(df["high"].iloc[-1], df["low"].iloc[-1], df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.atr == final_result.state.atr)

def test_atr_pandas_success(csv_loader):
    df = csv_loader("atr")
    result = tb.atr(df["high"].iloc[:-1], df["low"].iloc[:-1], df["close"].iloc[:-1], 14)
    final_result = tb.atr(df["high"], df["low"], df["close"], 14)

    next_state = tb.atr_next(df["high"].iloc[-1], df["low"].iloc[-1], df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.atr == final_result.state.atr)

def test_thread_atr(thread_test):
    def atr_tx_lambda(data):
        return tb.atr(data + 2.0, data - 2.0, data, 14, release_gil = True)

    thread_test(atr_tx_lambda, n_threads=4)
