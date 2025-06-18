from typing import List
from template_helper.strings import T, T2, format_tab

def test_rust_params(fct_params: dict) -> str:
    return ', '.join(f'{param}: {ty}' for param, ty in fct_params.items())


def test_rust_get_data_input(fct_inputs: List[str]) -> str:
    if not fct_inputs:
        ret = ['let input = columns.get("close").unwrap();']
        ret += ['let len = input.len();']
        return f"\n{T}".join(ret)

    ret = f"\n{T}".join(
        f'let {input_} = columns.get("{input_}").unwrap();'
        for input_ in fct_inputs
    )
    ret += f"\n{T}let len = {fct_inputs[0]}.len();"
    return ret

def test_rust_get_expected(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f'let expected = columns.get("{indicator_name}").unwrap();'

    return f"\n{T}".join(
        f'let expected_{output} = columns.get("{output}").unwrap();'
        for output in fct_outputs
    )

def test_rust_fct_args(fct_inputs: List[str], fct_params: dict) -> str:
    args = []

    if not fct_inputs:
        args.append('&input[0..last_idx]')

    args.extend(f'&{input_}[0..last_idx]' for input_ in fct_inputs)
    args.extend(f'{param}' for param in fct_params.keys())
    return ', '.join(args)

def test_rust_check_outputs(indicator_name: str, fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return f"assert_vec_eq_gen_data(&expected[0..last_idx], &result.{indicator_name});"

    checks = []
    for output in fct_outputs:
        checks.append(f"assert_vec_eq_gen_data(&expected_{output}[0..last_idx], &result.{output});")
    return f"\n{T}".join(checks)

def test_rust_create_sample(indicator_camel_case: str, fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return "input[last_idx + i]"

    sample = f"{indicator_camel_case}Sample {{"
    for input_ in fct_inputs:
        sample += f"\n{T2}{T}{input_}: {input_}[last_idx + i],"
    sample += f"\n{T2}}}"
    return sample

def test_rust_check_next_outputs(indicator_name: str, fct_outputs: List[str]) -> str:

    """
    example:

        assert!(
            approx_eq_float(new_state.${indicator_name}, expected[last_idx + i], 1e-8),
            "Next [{}] expected {}, but got {}",
            i,
            expected[last_idx + i],
            new_state.${indicator_name}
        );
    """

    if not fct_outputs:
        ret = "assert!(\n"
        ret += f"{T2}{T}approx_eq_float(new_state.{indicator_name}, expected[last_idx + i], 1e-8),\n"
        ret += f"{T2}{T}\"Next [{{}}] expected {{}} but got {{}}\""
        ret += f"\n{T2}{T}i,\n"
        ret += f"{T2}{T}expected[last_idx + i],\n"
        ret += f"{T2}{T}new_state.{indicator_name}\n"
        ret += f"{T2});"
        return ret
    checks = []
    for output in fct_outputs:
        checks.append(
            f"assert!(\n"
            f"{T2}{T}approx_eq_float(new_state.{output}, expected_{output}[last_idx + i], 1e-8),\n"
            f"{T2}{T}\"Next [{{}}] expected {{}} but got {{}}\""
            f"\n{T2}{T}i,\n"
            f"{T2}{T}expected_{output}[last_idx + i],\n"
            f"{T2}{T}new_state.{output}\n"
            f"{T2});"
        )
    return f"\n{T2}".join(checks)

def test_rust_next_overflow_create_sample(indicator_camel_case: str, fct_inputs: List[str]) -> str:
    if not fct_inputs:
        return f"Float::MIN + 5.0"

    sample = f"{indicator_camel_case}Sample {{"
    for input_ in fct_inputs:
        sample += f"\n{T2}{input_}: Float::MIN + 5.0,"
    sample += f"\n{T}}}"
    return sample

def test_rust_dummy_input_args(fct_inputs: List[str], fct_params: dict) -> str:
    if not fct_inputs:
        return "&data"

    args = [f"&data" for _ in fct_inputs]
    args.extend(f"{param}" for param in fct_params.keys())
    return ', '.join(args)


def test_rust_dummy_output_args(fct_outputs: List[str]) -> str:
    if not fct_outputs:
        return "output.as_mut_slice()"

    return ', '.join(f"output.as_mut_slice()" for _ in fct_outputs)
