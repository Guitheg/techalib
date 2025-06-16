#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_ad():
    runner = pyperf.Runner()
    setup = "import numpy as np; close = np.random.random(10_000_000); low = np.clip(close - 0.1, 0, 1); high = np.clip(close + 0.1, 0, 1); volume = np.random.random(10_000_000) * 100; period = 100"
    runner.timeit("tx.ad", "tx.ad(high, low, close, volume)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.ad", "ta.AD(high, low, close, volume)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_ad()
