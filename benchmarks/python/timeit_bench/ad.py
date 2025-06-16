import numpy as np
import timeit
from . import print_benchmark
import techalib as tx
import talib

def benchmark_ad():
    iterations = 50
    close = np.random.random(5_000_000)
    low = np.clip(close - 0.1, 0, 1)
    high = np.clip(close + 0.1, 0, 1)
    volume = np.random.random(5_000_000) * 100

    duration = timeit.timeit(lambda: talib.AD(high, low, close, volume), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.ad(high, low, close, volume), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    close = np.random.random(1_000_000)
    low = np.clip(close - 0.1, 0, 1)
    high = np.clip(close + 0.1, 0, 1)
    volume = np.random.random(1_000_000) * 100

    duration = timeit.timeit(lambda: talib.AD(high, low, close, volume), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.ad(high, low, close, volume), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    close = np.random.random(50_000)
    low = np.clip(close - 0.1, 0, 1)
    high = np.clip(close + 0.1, 0, 1)
    volume = np.random.random(50_000) * 100

    duration = timeit.timeit(lambda: talib.AD(high, low, close, volume), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.ad(high, low, close, volume), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)
