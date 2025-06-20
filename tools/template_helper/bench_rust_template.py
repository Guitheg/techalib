from string import Template
from typing import List
from template_helper.strings import T, T2, format_tab
from pathlib import Path


def bench_rust_init_data(fct_inputs: List[str], fct_params: dict) -> str:
    init = []
    if not fct_inputs:
        init.append("let data: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();")
    else:
        init.extend([
            "let {inp}: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();".format(inp=inp)
            for inp in fct_inputs
        ])

    if fct_params:
        init.extend([
            f"let {param}: {type_} = // TODO: SET PARAM"
            for param, type_ in fct_params.items()
        ])
    return f"\n{T2}".join(init)

def bench_rust_input_args(fct_inputs: List[str], fct_params: dict) -> str:
    args = []
    if not fct_inputs:
        args.append("data")
    else:
        args.extend([f"&{inp}" for inp in fct_inputs])

    args.extend(fct_params)
    return ", ".join(args)
