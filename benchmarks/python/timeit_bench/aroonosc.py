import numpy as np
import timeit
from . import print_benchmark
import techalib as tx
import talib

def benchmark_aroonosc():
    period = 14

    iterations = 50
    high = np.random.random(5_000_000).astype(np.float64)
    low = np.random.random(5_000_000).astype(np.float64)

    duration = timeit.timeit(lambda: talib.AROONOSC(high, low, period), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.aroonosc(high, low, period), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    high = np.random.random(1_000_000).astype(np.float64)
    low = np.random.random(1_000_000).astype(np.float64)

    duration = timeit.timeit(lambda: talib.AROONOSC(high, low, period), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.aroonosc(high, low, period), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    high = np.random.random(50_000).astype(np.float64)
    low = np.random.random(50_000).astype(np.float64)

    duration = timeit.timeit(lambda: talib.AROONOSC(high, low, period), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.aroonosc(high, low, period), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)
