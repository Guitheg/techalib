#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_atr():
    runner = pyperf.Runner()
    setup = "import numpy as np; close = np.random.random(10_000_000); low = np.clip(close - 0.1, 0, 1); high = np.clip(close + 0.1, 0, 1); period = 100"
    runner.timeit("tx.atr", "tx.atr(close, high, low, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.atr", "ta.ATR(close, high, low, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_atr()
