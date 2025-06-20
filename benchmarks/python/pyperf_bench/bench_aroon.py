#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_aroon():
    runner = pyperf.Runner()
    setup = "import numpy as np; high = np.random.random(10_000_000), low = np.random.random(10_000_000), period = 14"
    runner.timeit("tx.aroon", "tx.aroon(high, low, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.aroon", "ta.AROON(high, low, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_aroon()
