from string import Template
from typing import List
from template_helper.strings import T, T2, format_tab
from pathlib import Path

ROOT = Path(__file__).parent.parent.parent

INNER_SAMPLE_STRUCT = ROOT / "tools" / "templates" / "inner_sample_struct.template"

def struct_result_definition_outputs(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f"pub {indicator_name}: Vec<Float>"
    else:
        return f",\n{T}".join([f"pub {output}: Vec<Float>" for output in fct_outputs])

def struct_state_definition_outputs(indicator_name: str, fct_outputs: List[str]) -> str:
    state_template = "pub prev_{output}: Float"
    if not fct_outputs:
        return format_tab(state_template, output=indicator_name) + ","
    else:
        return f",\n{T}".join([
            format_tab(state_template, output=output) for output in fct_outputs
        ]) + ","

def struct_state_definition_states(fct_states: dict) -> str:
    if not fct_states:
        return ""
    state_template = "pub prev_{state}: {type}"
    return f",\n{T}".join([
        format_tab(state_template, state=state, type=type_) for state, type_ in fct_states.items()
    ]) + ","

def struct_state_definition_params(fct_params: dict) -> str:
    if not fct_params:
        return ""
    param_template = "pub {param}: {type}"
    return f",\n{T}".join([
        format_tab(param_template, param=param, type=type_) for param, type_ in fct_params.items()
    ]) + ","

def state_update_checks(
    fct_inputs: List[str],
    fct_states: dict
) -> str:
    checks = []
    if fct_inputs:
        checks.append(format_tab("check_finite!({});", ", ".join([f'sample.{inp}' for inp in fct_inputs])))
    else:
        checks.append(format_tab("check_finite!({});", f'sample'))

    if fct_states:
        for state, type_ in fct_states.items():
            if type_ == "Float":
                checks.append(f'check_finite!(self.prev_{state});')
            if type_ == "Vec<Float>" or type_ == "VecDeque<Float>":
                checks.append(f'check_vec_finite!(self.prev_{state});')
    return f"\n{T2}".join(checks)

def state_update_next_outputs(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict,
) -> str:
    outputs = []
    if fct_outputs:
        outputs = [f"new_{output}" for output in fct_outputs]
    else:
        outputs = [f"new_{indicator_name}"]

    if fct_states:
        outputs.extend([f"new_{state}" for state in fct_states.keys()])

    if len(outputs) == 1:
        return outputs[0]
    else:
        return f"({', '.join(outputs)})"

def state_update_next_args(
    fct_inputs: List[str],
    fct_states: dict,
    fct_params: dict
) -> str:
    args = []
    if fct_inputs:
        args.extend([f'sample.{inp}' for inp in fct_inputs])
    else:
        args.append('sample')
    if fct_states:
        args.extend([f'self.prev_{state}' for state in fct_states.keys()])
    if fct_params:
        args.extend([f'self.{param}' for param in fct_params.keys()])

    return ", ".join(args)

def state_update_check_output_and_set(
    indicator_name: str,
    fct_outputs: List[str],
    fct_states: dict
) -> str:
    updates = []
    if fct_outputs:
        for output in fct_outputs:
            updates.append(f"check_finite!(&new_{output});")
            updates.append(f"self.prev_{output} = new_{output};")
    else:
        updates.append(f"check_finite!(new_{indicator_name});")
        updates.append(f"self.prev_{indicator_name} = new_{indicator_name};")

    if fct_states:
        for state in fct_states.keys():
            updates.append(f"self.prev_{state} = new_{state};")

    return f"\n{T2}".join(updates)

def state_type(indicator_camel_case: str, fct_inputs: dict) -> str:
    if not fct_inputs:
        return "Float"
    else:
        return format_tab("{IndicatorName}Sample", IndicatorName=indicator_camel_case)

def struct_sample(indicator_name: str, indicator_camel_case: str, fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return ""
    else:
        with open(INNER_SAMPLE_STRUCT, 'r') as f:
            sample_template = Template(f.read())
        inputs_to_sample_template = "pub {input}: Float"
        return sample_template.substitute({
            "IndicatorName": indicator_camel_case,
            "INDICATORNAME": indicator_name.upper(),
            "StructSample_Definition": f",\n{T}".join([
                format_tab(inputs_to_sample_template, input=inp)
                for inp in fct_inputs
            ])
        })

def fct_args(fct_inputs: List[str], fct_params: List[str]) -> str:
    fct_args = []
    if fct_inputs:
        for inp in fct_inputs:
            fct_args.append(f"{inp}: &[Float]")
    else:
        fct_args.append("data: &[Float]")
    if fct_params:
        for param, type_ in fct_params.items():
            fct_args.append(f"{param}: {type_}")
    return f",\n{T}".join(fct_args) if fct_args else ""

def fct_outputs_initialisation(fct_inputs: List[str], fct_outputs: List[str]) -> str:
    data_name = fct_inputs[0] if fct_inputs else "data"
    if not fct_outputs:
        return f"let mut output = vec![0.0; {data_name}.len()];\n"
    else:
        return f"{T}".join([
            format_tab(
                "let mut output_{output} = vec![0.0; {data_name}.len()];\n",
                output=output, data_name=data_name
            ) for output in fct_outputs
        ])

def fct_args_in_intofct(fct_inputs: List[str], fct_params: List[str], fct_outputs: List[str]) -> str:
    if fct_inputs:
        into_args = f",\n{T2}".join([f"{inp}" for inp in fct_inputs])
    else:
        into_args = "data"

    if fct_params:
        into_args += f",\n{T2}" + f",\n{T2}".join([f"{param}" for param in fct_params.keys()])

    if fct_outputs:
        into_args += f",\n{T2}" + f",\n{T2}".join([f"output_{output}.as_mut_slice()" for output in fct_outputs])

    return into_args

def fct_result_creation(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f"{indicator_name}: output,"
    else:
        return f",\n{T}{T}".join([f"{output}: output_{output}" for output in fct_outputs])

def into_fct_output_args(fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return "output: &mut output"
    else:
        return f",\n{T}".join([f"output_{output}: &mut [Float]" for output in fct_outputs])

def into_fct_checks(fct_inputs: List[str], fct_outputs: List[str]) -> str:
    if not fct_inputs:
        data_name = "data"
    else:
        data_name = fct_inputs[0]

    checks = []
    if len(fct_inputs) > 1:
        for inp in fct_inputs[1:]:
            checks.append(format_tab("check_param_eq!({}.len(), {}.len());", data_name, inp))

    if not fct_outputs:
        checks.append(format_tab("check_param_eq!({}.len(), output.len());", data_name))
    else:
        for output in fct_outputs:
            checks.append(format_tab("check_param_eq!({}.len(), output_{}.len());", data_name, output))

    return f"\n{T}".join(checks) + f"\n{T}let len = {data_name}.len();\n"

def into_fct_init_unchecked_input_args(fct_inputs: List[str], fct_outputs: List[str], fct_params: dict) -> str:
    if fct_inputs:
        input_args = ", ".join([f"{inp}" for inp in fct_inputs])
    else:
        input_args = "data"

    if fct_params:
        input_args += ", " + ", ".join([f"{param}" for param in fct_params.keys()])
        input_args += ", lookback"

    if fct_outputs:
        input_args += ", " + ", ".join([f"output_{output}" for output in fct_outputs])

    return input_args

def into_fct_init_unchecked_outputs(fct_outputs: List[str], fct_states: dict) -> str:
    if not fct_outputs and not fct_states:
        return "first_output"

    outputs = []
    if fct_outputs:
        outputs.extend([f"first_output_{output}" for output in fct_outputs])

    if fct_states:
        outputs.extend([f"mut {state}" for state in fct_states.keys()])

    if len(outputs) == 1:
        return outputs[0]
    elif len(outputs) > 1:
        return "({})".format(", ".join(outputs))

def into_fct_init_unchecked_check_outputs_and_set(fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f"check_finite!(first_output);\n{T}output[lookback] = first_output;"

    checks = []
    for output in fct_outputs:
        checks.append(format_tab("output_{output}[lookback] = first_output_{output};", output=output))
        checks.append(format_tab("check_finite_at!(lookback, output_{output});", output=output))

    return f"\n{T}".join(checks)

def into_fct_check_finites_in(fct_inputs: List[str], fct_outputs: List[str]) -> str:
    if not fct_inputs:
        return "check_finite_at!(idx, data);"

    checks = []
    checks.append(format_tab("check_finite_at!(idx, {});", ", ".join(fct_inputs)))
    return f"\n{T2}".join(checks)

def into_fct_check_finites_out(fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return "check_finite_at!(idx, output);"

    checks = []
    checks.append(format_tab("check_finite_at!(idx, {});", ", ".join([f"output_{out}" for out in fct_outputs])))

    return f"\n{T2}".join(checks)

def into_fct_next_out(fct_outputs: List[str], fct_states: dict) -> str:
    outputs = []
    if fct_outputs:
        outputs.extend([f"output_{output}[idx]" for output in fct_outputs])
    else:
        outputs.append("output[idx]")

    if fct_states:
        outputs.extend([f"{state}" for state in fct_states.keys()])

    if len(outputs) == 1:
        return outputs[0]
    else:
        return "({})".format(", ".join(outputs))

def into_fct_next_args(fct_inputs: List[str], fct_states: dict, fct_params: dict) -> str:
    args = []
    if fct_inputs:
        args.extend([f"{inp}[idx]" for inp in fct_inputs])
    else:
        args.append("data[idx]")

    if fct_states:
        args.extend([f"{state}" for state in fct_states.keys()])

    if fct_params:
        args.extend([f"{param}" for param in fct_params.keys()])

    return ", ".join(args)

def into_fct_state_creation(
    indicator_name: str, fct_outputs: List[str], fct_states: dict, fct_params: dict
) -> str:
    state_creation = []
    if fct_outputs:
        state_creation.extend([f"prev_{output}: output_{output}[len - 1]" for output in fct_outputs])
    else:
        state_creation.append(f"prev_{indicator_name}: output[len - 1]")

    if fct_states:
        for state, type_ in fct_states.items():
            if type_ == "Float":
                state_creation.append(f"prev_{state}: {state}")
            elif "VecDeque<Float>":
                state_creation.append(f"prev_{state}: VecDeque::new(), //from(data[len - period..len].to_vec())")
            else:
                raise ValueError(f"Unsupported state type: {type_}")

    if fct_params:
        state_creation.extend([f"{param}" for param in fct_params.keys()])

    return f",\n{T2}".join(state_creation)

def init_unchecked_args(
    fct_inputs: List[str],
    fct_params: dict,
    fct_outputs: List[str]
) -> str:
    args = []
    if fct_inputs:
        args.extend([f"{inp}: &[Float]" for inp in fct_inputs])
    else:
        args.append("data: &[Float]")

    if fct_params:
        args.extend([f"{param}: {type_}" for param, type_ in fct_params.items()])
    args.append("lookback: usize")

    if fct_outputs:
        args.extend([f"output_{output}: &mut [Float]" for output in fct_outputs])

    return f",\n{T}".join(args)

def init_unchecked_return_type(fct_outputs: List[str], fct_states: dict) -> str:
    if not fct_outputs and not fct_states:
        return "Float"
    else:
        return "({})".format(
            ", ".join([f"Float" for _ in fct_outputs + list(fct_states.keys())])
        )

def init_unchecked_return_values(
    fct_outputs: List[str],
    fct_states: dict
) -> str:
    outputs = []
    if fct_outputs:
        outputs.extend([f"0.0" for output in fct_outputs])

    if fct_states:
        outputs.extend([f"0.0" for state in fct_states.keys()])

    if len(outputs) == 1:
        return "0.0"
    else:
        return "({})".format(", ".join(outputs))

def next_args(fct_inputs: List[str], fct_states: dict, fct_params: dict) -> str:
    args = []
    if fct_inputs:
        args.extend([f"new_{inp}: Float" for inp in fct_inputs])
    else:
        args.append("new_value: Float")

    if fct_states:
        args.extend([f"prev_{state}: {type_}" for state, type_ in fct_states.items()])

    if fct_params:
        args.extend([f"{param}: {type_}" for param, type_ in fct_params.items()])

    return f",\n{T}".join(args) if args else ""
