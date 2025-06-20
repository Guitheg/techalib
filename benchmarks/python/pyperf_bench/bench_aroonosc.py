#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_aroonosc():
    runner = pyperf.Runner()
    setup = "import numpy as np; high = np.random.random(10_000_000), low = np.random.random(10_000_000), period = 14"
    runner.timeit("tx.aroonosc", "tx.aroonosc(high, low, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.aroonosc", "ta.AROONOSC(high, low, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_aroonosc()
