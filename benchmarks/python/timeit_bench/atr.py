import numpy as np
import timeit
from . import print_benchmark
import techalib as tx
import talib

def benchmark_atr():
    iterations = 50
    close = np.random.random(5_000_000)
    low = np.clip(close - 0.1, 0, 1)
    high = np.clip(close + 0.1, 0, 1)

    duration = timeit.timeit(lambda: talib.ATR(high, low, close, 14), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.atr(high, low, close, 14), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(close)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    close = np.random.random(1_000_000)
    low = np.clip(close - 0.1, 0, 1)
    high = np.clip(close + 0.1, 0, 1)

    duration = timeit.timeit(lambda: talib.ATR(high, low, close, 14), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.atr(high, low, close, 14), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(close)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    close = np.random.random(50_000)
    low = np.clip(close - 0.1, 0, 1)
    high = np.clip(close + 0.1, 0, 1)

    duration = timeit.timeit(lambda: talib.ATR(high, low, close, 14), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.atr(high, low, close, 14), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(close)}, rust=average_time_rs, c=average_time_c)
