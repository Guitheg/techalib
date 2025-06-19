#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_dx():
    runner = pyperf.Runner()
    # TODO: DEFINE SETUP (PARAM VALUES ETC.)
    setup = "import numpy as np; high = np.random.random(10_000_000), low = np.random.random(10_000_000), close = np.random.random(10_000_000), period = TODO:SET_PARAM"
    runner.timeit("tx.dx", "tx.dx(high, low, close, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.dx", "ta.DX(high, low, close, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_dx()
