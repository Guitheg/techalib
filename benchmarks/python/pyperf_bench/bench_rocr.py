#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_rocr():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000), period = TODO:SET_PARAM"
    runner.timeit("tx.rocr", "tx.rocr(data, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.rocr", "ta.ROCR(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_rocr()
