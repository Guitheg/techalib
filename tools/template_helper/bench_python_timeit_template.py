from string import Template
from typing import List
from template_helper.strings import T, T2, format_tab
from pathlib import Path

def bench_init_param(fct_params: List[str]) -> str:
    if not fct_params:
        return ""
    else:
        return "\n".join([
            "{param} = # TODO: SET PARAM".format(param=param) for param in fct_params
        ])

def bench_data_init_5m(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return "data = np.random.random(5_000_000).astype(np.float64)"
    else:
        return f"\n{T}".join([
            "{inp} = np.random.random(5_000_000).astype(np.float64)".format(inp=inp)
            for inp in fct_inputs
        ])

def bench_data_init_1m(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return "data = np.random.random(1_000_000).astype(np.float64)"
    else:
        return f"\n{T}".join([
            "{inp} = np.random.random(1_000_000).astype(np.float64)".format(inp=inp)
            for inp in fct_inputs
        ])

def bench_data_init_50k(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return "data = np.random.random(50_000).astype(np.float64)"
    else:
        return f"\n{T}".join([
            "{inp} = np.random.random(50_000).astype(np.float64)".format(inp=inp)
            for inp in fct_inputs
        ])

def bench_data_args(fct_inputs: List[str], fct_params: List[str]) -> str:
    args = []
    if not fct_inputs:
        args.append("data")
    else:
        args.extend(fct_inputs)

    args.extend(fct_params)
    return ", ".join(args)

def bench_length(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return "len(data)"
    else:
        return "len({})".format(fct_inputs[0])
