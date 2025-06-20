from typing import List
from template_helper.strings import T, T2, format_tab

def pytest_input_data(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return "data"
    input_data = ", ".join(f"{input_}" for input_ in fct_inputs)
    return input_data

def pytest_input_param(fct_params: dict) -> str:
    return ", ".join([f"{param}" for param in fct_params.keys()])

def pytest_input_ohlcv(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return '"close"'
    return ", ".join([f'"{inp}"' for inp in fct_inputs])

def pytest_output(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f'"{indicator_name}"'
    return ", ".join([f'"{output}"' for output in fct_outputs])

def pytest_state_output(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f'"prev_{indicator_name}"'
    return ", ".join([f'"prev_{output}"' for output in fct_outputs])
