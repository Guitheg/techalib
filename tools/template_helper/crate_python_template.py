# -*- coding: utf-8 -*-
from typing import List
from template_helper.strings import T, T2, format_tab


def pystate_attributes_definition(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    attrs = []
    if not fct_outputs:
        attrs.append("#[pyo3(get)]")
        attrs.append(f"pub prev_${indicator_name}: Float,")
    else:
        attrs.extend([
            f"#[pyo3(get)]\n{T}pub prev_{inp}: Float," for inp in fct_outputs
        ])

    for state, type_ in fct_states.items():
        if type_.startswith("VecDeque"):
            type_ = type_.replace("VecDeque", "Vec")
        attrs.append(f"#[pyo3(get)]\n{T}pub prev_{state}: {type_},")

    for param, type_ in fct_params.items():
        if type_.startswith("Option"):
            type_ = type_.replace("Option<", "").replace(">", "")
        attrs.append(f"#[pyo3(get)]\n{T}pub {param}: {type_},")

    return f"\n{T}".join(attrs)

def pystate_new(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    args = []
    if not fct_outputs:
        args.append(f"prev_${indicator_name}: Float,")
    else:
        args.extend([
            f"prev_{inp}: Float," for inp in fct_outputs
        ])

    for state, type_ in fct_states.items():
        if type_.startswith("VecDeque"):
            type_ = type_.replace("VecDeque", "Vec")
        args.append(f"prev_{state}: {type_},")

    for param, type_ in fct_params.items():
        if type_.startswith("Option"):
            type_ = type_.replace("Option<", "").replace(">", "")
        args.append(f"{param}: {type_},")

    return f"\n{T2}".join(args)

def pystate_creation(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    attrs = []
    if not fct_outputs:
        attrs.append(f"prev_${indicator_name},")
    else:
        attrs.extend([
            f"prev_{inp}," for inp in fct_outputs
        ])

    for state in fct_states.keys():
        attrs.append(f"prev_{state},")

    for param in fct_params.keys():
        attrs.append(f"{param},")

    return f"\n{T2}{T}".join(attrs)

def pyfrom_state_to_pystate(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    attrs = []
    if not fct_outputs:
        attrs.append(f"prev_${indicator_name}: state.prev_${indicator_name},")
    else:
        attrs.extend([
            f"prev_{inp}: state.prev_{inp}," for inp in fct_outputs
        ])

    for state, type_ in fct_states.items():
        if type_.startswith("VecDeque"):
            attrs.append(f"prev_{state}: state.prev_{state}.into(),")
        else:
            attrs.append(f"prev_{state}: state.prev_{state},")

    for param in fct_params.keys():
        attrs.append(f"{param}: state.{param},")

    return f"\n{T2}{T}".join(attrs)

def pyfrom_pystate_to_state(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    attrs = []
    if not fct_outputs:
        attrs.append(f"prev_${indicator_name}: py_state.prev_${indicator_name},")
    else:
        attrs.extend([
            f"prev_{inp}: py_state.prev_{inp}," for inp in fct_outputs
        ])

    for state, type_ in fct_states.items():
        if type_.startswith("VecDeque"):
            attrs.append(f"prev_{state}: py_state.prev_{state}.into(),")
        else:
            attrs.append(f"prev_{state}: py_state.prev_{state},")

    for param in fct_params.keys():
        attrs.append(f"{param}: py_state.{param},")

    return f"\n{T2}{T}".join(attrs)

def py_signature(
    fct_inputs: List[str],
    fct_params: dict,
) -> str:
    signature = []
    if not fct_inputs:
        signature.append("data")
    else:
        signature.extend(fct_inputs)

    for param in fct_params.keys():
        if param == "period":
            signature.append(f"{param} = 14")
        else:
            signature.append(f"{param} = None")

    return ", ".join(signature)

def py_args(
    fct_inputs: List[str],
    fct_params: dict,
) -> str:
    args = []
    if not fct_inputs:
        args.append("data: PyReadonlyArray1<Float>")
    else:
        args.extend([f"{inp}: PyReadonlyArray1<Float>" for inp in fct_inputs])

    for param, type_ in fct_params.items():
        args.append(f"{param}: {type_}")

    return f",\n{T}".join(args)

def py_outputs(
    fct_outputs: List[str],
) -> str:
    if not fct_outputs:
        return "Py<PyArray1<Float>>"

    outputs = [f"Py<PyArray1<Float>>" for _ in fct_outputs]
    return "({})".format(f", ".join(outputs))

def py_define_data(
    fct_inputs: List[str],
) -> str:
    if not fct_inputs:
        return f"let data_slice = data.as_slice()?;\n{T}let len = data.len();"

    defines = [f"let {inp}_slice = {inp}.as_slice()?;" for inp in fct_inputs]
    return f"\n{T}".join(defines) + f"\n{T}" + "let len = {}.len();".format(fct_inputs[0])

def py_define_outputs(
    fct_outputs: List[str],
) -> str:
    if not fct_outputs:
        return "let mut output = vec![0.0; len];"

    defines = [f"let mut {out} = vec![0.0; len];" for out in fct_outputs]
    return f"\n{T2}".join(defines)

def py_into_fct_input_args(
    fct_inputs: List[str],
    fct_params: dict,
    fct_outputs: List[str]
) -> str:
    args = []
    if not fct_inputs:
        args.append("data_slice")
    else:
        args.extend([f"{inp}_slice" for inp in fct_inputs])

    for param in fct_params.keys():
        args.append(f"{param}")

    if not fct_outputs:
        args.append("output.as_mut_slice()")
    else:
        args.extend([f"{out}.as_mut_slice()" for out in fct_outputs])

    return ", ".join(args)

def py_results_outputs(
    fct_outputs: List[str],
) -> str:
    if not fct_outputs:
        return "output.into_pyarray(py).into(),"

    outputs = [f"{out}.into_pyarray(py).into()" for out in fct_outputs]
    return f"{f',\n{T2}{T}'.join(outputs)}"

def py_define_outputs_pyheap(
    fct_outputs: List[str],
) -> str:
    if not fct_outputs:
        ret = "let output = PyArray1::<Float>::zeros(py, [len], false);\n"
        ret += "let output_slice = unsafe { output.as_slice_mut()? };"

    out_define = []
    for out in fct_outputs:
        out_define.append(f"let {out} = PyArray1::<Float>::zeros(py, [len], false);")
        out_define.append(f"let {out}_slice = unsafe {{ {out}.as_slice_mut()? }};")

    return f"\n{T2}".join(out_define)

def py_into_fct_input_args_pyheap(
    fct_inputs: List[str],
    fct_params: dict,
    fct_outputs: List[str]
) -> str:
    args = []
    if not fct_inputs:
        args.append("data_slice")
    else:
        args.extend([f"{inp}_slice" for inp in fct_inputs])

    for param in fct_params.keys():
        args.append(f"{param}")

    if not fct_outputs:
        args.append("output_slice")
    else:
        args.extend([f"{out}_slice" for out in fct_outputs])

    return ", ".join(args)

def py_results_outputs_pyheap(
    fct_outputs: List[str],
) -> str:
    if not fct_outputs:
        return "output.into(),"

    outputs = [f"{out}.into()" for out in fct_outputs]
    return f"{f',\n{T2}{T}'.join(outputs)}"

def py_next_signature(
    fct_inputs: List[str],
) -> str:
    if not fct_inputs:
        return "data"

    return ", ".join([f"new_{inp}" for inp in fct_inputs])

def py_next_args_definition(
    fct_inputs: List[str],
) -> str:
    if not fct_inputs:
        return "new_value: Float"

    return f",\n{T}".join([f"new_{inp}: Float" for inp in fct_inputs])

def py_next_create_sample(
    indicator_camel_case: str,
    fct_inputs: List[str],
) -> str:
    if not fct_inputs:
        return "new_value;"

    ret = f"{indicator_camel_case}Sample {{\n"
    for inp in fct_inputs:
        ret += f"{T2}{inp}: new_{inp},\n"
    ret += f"{T}}};\n"

    return ret

def py_import_sample(
    indicator_camel_case: str,
    fct_inputs: List[str],
) -> str:
    if len(fct_inputs) > 1:
        return f"{indicator_camel_case}Sample"
    return ""
