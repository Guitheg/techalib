from concurrent.futures import ThreadPoolExecutor
import pytest
from typing import Tuple, Callable
from numpy.typing import NDArray
from numpy import testing
import pandas as pd
import numpy as np
import time
import importlib
import os

DATA_DIR = "tests/data/generated/{feature_name}.csv"
THREAD_TEST_TOL: float = float(os.getenv("THREAD_TEST_TOL", "0.5"))

@pytest.fixture
def test_with_generated_data(csv_loader) -> Callable[[Callable], None]:
    def _test_with_generated_data(
        filepath: str,
        tx_fct: Callable,
        tx_next_fct: Callable,
        input_names: list,
        output_names: list,
        state_out_names: list = None,
        talib_output_names: list = None,
        rtol: float = 1e-7,
        atol: float = 0.0,
    ) -> None:
        if state_out_names is None:
            state_out_names = output_names
        if talib_output_names is None:
            talib_output_names = output_names
        df = csv_loader(filepath)
        next_count = 10
        prev_inputs = [df[name].iloc[:-next_count] for name in input_names]
        result = tx_fct(*prev_inputs)
        for name, talib_name in zip(output_names, talib_output_names):
            out = getattr(result, name, None)
            expected = df[talib_name].iloc[:-next_count]
            testing.assert_allclose(expected, out, rtol=rtol, atol=atol)

        state = result.state
        for i in range(next_count):
            next_inputs = [df[name].iloc[-next_count + i] for name in input_names]
            state = tx_next_fct(*next_inputs, state)
            for state_out_name, out_name in zip(state_out_names, talib_output_names):
                out = getattr(state, state_out_name, None)
                expected = df[out_name].iloc[-next_count + i]
                testing.assert_allclose(expected, out, rtol=rtol, atol=atol)
    return _test_with_generated_data

@pytest.fixture
def test_numpy_with_generated_data(csv_loader) -> Callable[[Callable], None]:
    def _test_numpy_with_generated_data(
        filepath: str,
        tx_fct: Callable,
        tx_next_fct: Callable,
        input_names: list,
        output_names: list,
        state_out_names: list = None,
        talib_output_names: list = None,
        rtol: float = 1e-7,
        atol: float = 0.0,
    ) -> None:
        if state_out_names is None:
            state_out_names = output_names
        if talib_output_names is None:
            talib_output_names = output_names
        df = csv_loader(filepath)
        next_count = 10
        prev_inputs = [np.array(df[name].iloc[:-next_count]) for name in input_names]
        result = tx_fct(*prev_inputs)
        for name, talib_name in zip(output_names, talib_output_names):
            out = getattr(result, name, None)
            expected = np.array(df[talib_name].iloc[:-next_count])
            testing.assert_allclose(expected, out, rtol=rtol, atol=atol)

        state = result.state
        for i in range(next_count):
            next_inputs = [np.array(df[name].iloc[-next_count + i]) for name in input_names]
            state = tx_next_fct(*next_inputs, state)
            for state_out_name, out_name in zip(state_out_names, talib_output_names):
                out = getattr(state, state_out_name, None)
                expected = df[out_name].iloc[-next_count + i]
                testing.assert_allclose(expected, out, rtol=rtol, atol=atol)
    return _test_numpy_with_generated_data

@pytest.fixture
def csv_loader() -> Callable[[str], pd.DataFrame] :
    def _load(feature_name: str) -> pd.DataFrame:
        csv_path = DATA_DIR.format(feature_name = feature_name)
        return pd.read_csv(csv_path, delimiter=",")
    return _load


@pytest.fixture
def thread_test() -> Callable[[Callable], None]:
    def _thread_test(tx_lambda: Callable, n_threads: int = 4, tolerance: float = THREAD_TEST_TOL) -> None:
        data = np.random.rand(5_000_000).astype(np.float64)
        t0 = time.perf_counter()
        for _ in range(n_threads):
            _ = tx_lambda(data)
        seq_time = (time.perf_counter() - t0) / n_threads
        with ThreadPoolExecutor(max_workers=n_threads) as pool:
            t0 = time.perf_counter()
            futures = [pool.submit(tx_lambda, data) for _ in range(n_threads)]
            _ = [future.result() for future in futures]
            conc_time = (time.perf_counter() - t0) / n_threads

        print(f"Sequential time: {seq_time:.4f} seconds")
        print(f"Concurrent time: {conc_time:.4f} seconds")
        conc_compoare = (conc_time * n_threads) * tolerance
        print(f"Concurrent time for comparison with {tolerance*100:.0f}% tolerance: {conc_compoare:.4f} seconds")
        assert conc_compoare < seq_time, f"Concurrent execution should be faster than sequential execution. {conc_time:.4f} <? {seq_time:.4f}"
    return _thread_test
