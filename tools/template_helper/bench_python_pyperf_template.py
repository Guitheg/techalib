from string import Template
from typing import List
from template_helper.strings import T, T2, format_tab
from pathlib import Path

def bench_input_args_init(fct_inputs: List[str], fct_params: List[str]) -> str:
    """
    Generate the input arguments for the benchmark setup.
    """
    args = []
    if not fct_inputs:
        args.append("data = np.random.random(10_000_000)")
    else:
        args.extend([f"{input_} = np.random.random(10_000_000)" for input_ in fct_inputs])

    args.extend([f"{param} = TODO:SET_PARAM" for param in fct_params])
    return ", ".join(args)

def bench_input_args(fct_inputs: List[str], fct_params: List[str]) -> str:
    """
    Generate the input arguments for the benchmark function call.
    """
    args = []
    if not fct_inputs:
        args.append("data")
    else:
        args.extend(fct_inputs)

    args.extend(fct_params)
    return ", ".join(args)
