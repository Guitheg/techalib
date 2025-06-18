from typing import List
from template_helper.strings import T, T2, format_tab

def py_stub_state_attributes(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    attrs = []
    if not fct_outputs:
        attrs.append(f"prev_${indicator_name}: float,")
    else:
        attrs.extend([
            f"prev_{inp}: float" for inp in fct_outputs
        ])

    for state, type_ in fct_states.items():
        if type_.startswith("VecDeque"):
            type_ = type_.replace("VecDeque", "List").replace("<", "[").replace(">", "]")
        elif type_.startswith("Vec"):
            type_ = type_.replace("Vec", "List").replace("<", "[").replace(">", "]")
        type_ = type_.replace("Float", "float")
        if type_ in ["usize", "u32", "u64", "u16", "u8", "i32", "i64", "i16", "i8"]:
            type_ = "int"
        attrs.append(f"prev_{state}: {type_}")

    for param, type_ in fct_params.items():
        if type_.startswith("Option"):
            type_ = type_.replace("Option<", "").replace(">", "")
        if type_ == "Float":
            type_ = "float"
        elif type_ in ["usize", "u32", "u64", "u16", "u8", "i32", "i64", "i16", "i8"]:
            type_ = "int"
        attrs.append(f"{param}: {type_}")

    return f"\n{T}".join(attrs)

def py_stub_results_attributes(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f"{indicator_name}: NDArray"
    else:
        return f"\n{T}".join([f"{inp}: NDArray" for inp in fct_outputs])

def py_stub_args(fct_inputs: List[str], fct_params: dict) -> str:
    args = []
    for inp in fct_inputs:
        args.append(f"{inp}: NDArray,")

    for param, type_ in fct_params.items():
        if type_.startswith("Option<"):
            type_ = type_.replace("Option<", "").replace(">", "")
        if type_ == "Float":
            type_ = "float"
        elif type_ in ["usize", "u32", "u64", "u16", "u8", "i32", "i64", "i16", "i8"]:
            type_ = "int"
        args.append(f"{param}: {type_}, # TODO: SET DEFAULT VALUE")

    return f"\n{T}".join(args)

def py_stub_next_args(fct_inputs: List[str]) -> str:
    args = []
    for inp in fct_inputs:
        args.append(f"new_{inp}: float,")

    return f"\n{T}".join(args)
