from typing import List
from template_helper.strings import T, T2, format_tab

def test_fuzz_define_signature(
    fct_inputs: List[str],
    fct_params: dict
) -> str:
    """
    Generate the signature for the fuzz test input.
    """
    args = []
    if not fct_inputs:
        args.append("Vec<Float>")
    else:
        args.extend([f"Vec<Float>" for _ in fct_inputs])

    for param, type_ in fct_params.items():
        args.append(f"{type_}")

    return f"{', '.join(args)}"

def test_fuzz_get_inputs(fct_inputs: List[str], fct_params: dict) -> str:
    inps = []
    if not fct_inputs:
        inps.append("data")
    else:
        inps.extend([f"{inp}" for inp in fct_inputs])

    for param in fct_params.keys():
        inps.append(f"{param}")
    return ", ".join(inps)

def test_fuzz_set_params(fct_inputs: List[str], fct_params: dict) -> str:
    params = []
    params.append("let len = {inp}.len();".format(inp=fct_inputs[0] if fct_inputs else "data"))
    for param, type_ in fct_params.items():
        if type_.startswith("Option"):
            params.append(f"let {param} = None; // TODO: SET VALUES")
        elif type_ in ["usize", "u32", "u64", "u16", "u8", "i32", "i64", "i16", "i8"]:
            params.append(f"let {param} = ({param} as usize % len.saturating_add(1)).max(1);")
        elif type_ == "Float":
            params.append(f"let {param} = 0.5; // TODO: SET VALUES")
    return f"\n{T}".join(params)

def test_fuzz_input_fct(fct_inputs: List[str], fct_params: dict) -> str:
    inputs = []
    if not fct_inputs:
        inputs.append("data")
    else:
        inputs.extend([f"&{inp}" for inp in fct_inputs])

    for param in fct_params.keys():
        inputs.append(f"{param}")

    return ", ".join(inputs)
