#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_minus_dm():
    runner = pyperf.Runner()
    setup = "import numpy as np; high = np.random.random(10_000_000); low = np.clip(high - 0.1, 0, 1); period = 100"
    runner.timeit("tx.minus_dm", "tx.minus_dm(high, low, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.minus_dm", "ta.MINUS_DM(high, low, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_minus_dm()
