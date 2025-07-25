#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_plus_di():
    runner = pyperf.Runner()
    setup = "import numpy as np; high = np.random.random(10_000_000), low = np.random.random(10_000_000), close = np.random.random(10_000_000), period = 14"
    runner.timeit("tx.plus_di", "tx.plus_di(high, low, close, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.plus_di", "ta.PLUS_DI(high, low, close, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_plus_di()
