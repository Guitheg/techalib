from typing import List
from pathlib import Path
import argparse
from utils.logger import logger
from utils.kwargs_parser import ParseKwargs
from template_helper.crate_core_template import *
from template_helper.bench_python_pyperf_template import *
from template_helper.bench_python_timeit_template import *
from template_helper.bench_rust_template import *
from template_helper.crate_python_template import *
from template_helper.python_stub_template import *
from template_helper.tests_fuzz_template import *
from template_helper.tests_python_template import *
from template_helper.tests_rust_template import *
from string import Template
import subprocess

ROOT = Path(__file__).parent.parent

DEFAULT_CONTRIBUTOR_NAME = "Unknown"

# TEMPLATES
TEMPLATE_DIR = ROOT / "tools" / "templates"
INSERT_TO_PYLIB = ROOT / "tools" / "templates" / "insert_pylib.template"
INSERT_TO_FUZZ_CARGO = ROOT / "tools" / "templates" / "insert_fuzz_cargo.template"

# WILL BE MODIFIED
#   BENCHMARKS
BENCH_PYTHON_TIMEIT = ROOT / "benchmarks/python/benchmark_timeit.py"
BENCH_RUST = ROOT / "benchmarks/rust/bench.rs"
BENCH_RUST_MOD = ROOT / "benchmarks/rust/indicators/mod.rs"
#   CRATES
CORE_LIB_PATH =  ROOT / "crates/core/src/indicators/mod.rs"
PYBINDING_LIB_PATH = ROOT / "crates/python/src/lib.rs"
#   PYTHON
PYTHON_STUB_INIT = ROOT / "python/techalib/_core/__init__.pyi"
MAPTYPES_PYTHON = ROOT / "python/techalib/maptypes.py"
#   TESTS
FUZZ_TESTS_CARGO = ROOT / "tests/fuzz/Cargo.toml"
RUST_TEST_LIB_PATH = ROOT / "tests/rust/src/lib.rs"

# WILL BE CREATED
TEMPLATES_TO_FILE = {
    "bench_python_pyperf.template": "benchmarks/python/pyperf_bench/bench_{name}.py",
    "bench_python_timeit.template": "benchmarks/python/timeit_bench/{name}.py",
    "bench_rust.template": "benchmarks/rust/indicators/bench_{name}.rs",
    "crate_core.template": "crates/core/src/indicators/{name}.rs",
    "crate_python.template": "crates/python/src/py_{name}.rs",
    "python_stub.template": "python/techalib/_core/{name}.pyi",
    "tests_fuzz.template": "tests/fuzz/fuzz_targets/fuzz_{name}.rs",
    "tests_python.template": "tests/python/test_{name}.py",
    "tests_rust.template": "tests/rust/src/tests_{name}.rs",
}

def create_from_template(
    indicator_name: str,
    indicator_camel_case: str = None,
    contributor_name: str = DEFAULT_CONTRIBUTOR_NAME,
    fct_inputs: List[str] = None,
    fct_outputs: List[str] = None,
    fct_states: dict = None,
    fct_params: dict = None
):
    logger.info(f"🔧 Creating templates for {indicator_name} (camel_case: {indicator_camel_case}, author: {contributor_name})")
    indicator_camel_case = indicator_name.capitalize() if (indicator_camel_case is None) else indicator_camel_case
    for template_file, target_file in TEMPLATES_TO_FILE.items():
        template_path = TEMPLATE_DIR / template_file
        target_path = Path(__file__).parent.parent / target_file.format(name=indicator_name)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        with open(template_path, 'r') as template_f:
            template_content = Template(template_f.read())
        with open(target_path, 'w') as target_f:
            target_f.write(template_content.substitute({
                "indicator_name": indicator_name,
                "IndicatorName": indicator_camel_case,
                "INDICATORNAME": indicator_name.upper(),
                "ContributorName": contributor_name,
                "StructResult_DefinitionOutputs": struct_result_definition_outputs(indicator_name, fct_outputs),
                "StructState_DefinitionOutputs": struct_state_definition_outputs(indicator_name, fct_outputs),
                "StructState_DefinitionStates": struct_state_definition_states(fct_states),
                "StructState_DefinitionParams": struct_state_definition_params(fct_params),
                "StateUpdate_Checks": state_update_checks(fct_inputs, fct_states),
                "StateUpdate_Next_Outputs": state_update_next_outputs(indicator_name, fct_outputs, fct_states),
                "StateUpdate_Next_Args": state_update_next_args(fct_inputs, fct_states, fct_params),
                "StateUpdate_CheckOutputAndSet": state_update_check_output_and_set(indicator_name, fct_outputs, fct_states),
                "StateType": state_type(indicator_camel_case, fct_inputs),
                "StructSample": struct_sample(indicator_name, indicator_camel_case, fct_inputs),
                "StructSample_Definition": struct_sample(indicator_name, indicator_camel_case, fct_inputs),
                "Fct_Args": fct_args(fct_inputs, fct_params),
                "Fct_OutputsInitialisation": fct_outputs_initialisation(fct_inputs, fct_outputs),
                "Fct_ArgsIn_IntoFct": fct_args_in_intofct(fct_inputs, fct_params, fct_outputs),
                "Fct_ResultCreation": fct_result_creation(indicator_name, fct_outputs),
                "IntoFct_OuputsArgs": into_fct_output_args(fct_outputs),
                "IntoFct_Checks": into_fct_checks(fct_inputs, fct_outputs),
                "IntoFct_InitUnchecked_InputArgs": into_fct_init_unchecked_input_args(fct_inputs, fct_outputs, fct_params),
                "IntoFct_InitUnchecked_Outputs": into_fct_init_unchecked_outputs(fct_outputs, fct_states),
                "IntoFct_InitUnchecked_CheckOutputsAndSet": into_fct_init_unchecked_check_outputs_and_set(fct_outputs),
                "IntoFct_CheckFinitesIn": into_fct_check_finites_in(fct_inputs, fct_outputs),
                "IntoFct_CheckFinitesOut": into_fct_check_finites_out(fct_outputs),
                "IntoFct_Next_Out": into_fct_next_out(fct_outputs, fct_states),
                "IntoFct_Next_Args": into_fct_next_args(fct_inputs, fct_states, fct_params),
                "IntoFct_StateCreation": into_fct_state_creation(indicator_name, fct_outputs, fct_states, fct_params),
                "InitUnchecked_Args": init_unchecked_args(fct_inputs, fct_params, fct_outputs),
                "InitUnchecked_ReturnType": init_unchecked_return_type(fct_outputs, fct_states),
                "InitUnchecked_ReturnValues": init_unchecked_return_values(fct_outputs, fct_states),
                "Next_Args": next_args(fct_inputs, fct_states, fct_params),
                "Next_ReturnType": init_unchecked_return_type(fct_outputs, fct_states),
                "Next_ReturnValues": init_unchecked_return_values(fct_outputs, fct_states),
                "Bench_InputArgs_Init": bench_input_args_init(fct_inputs, fct_params),
                "Bench_InputArgs": bench_input_args(fct_inputs, fct_params),
                "Bench_Init_Param": bench_init_param(fct_params),
                "Bench_Data_Init_5M": bench_data_init_5m(fct_inputs),
                "Bench_Data_Init_1M": bench_data_init_1m(fct_inputs),
                "Bench_Data_Init_50K": bench_data_init_50k(fct_inputs),
                "Bench_Data_Args": bench_data_args(fct_inputs, fct_params),
                "Bench_Length": bench_length(fct_inputs),
                "Bench_Rust_Init_Data": bench_rust_init_data(fct_inputs, fct_params),
                "Bench_Rust_Input_Args": bench_rust_input_args(fct_inputs, fct_params),
                "PyState_Attributes_Definition": pystate_attributes_definition(indicator_name, fct_outputs, fct_states, fct_params),
                "PyState_New": pystate_new(indicator_name, fct_outputs, fct_states, fct_params),
                "PyState_Creation": pystate_creation(indicator_name, fct_outputs, fct_states, fct_params),
                "PyFrom_State_To_PyState": pyfrom_state_to_pystate(indicator_name, fct_outputs, fct_states, fct_params),
                "PyFrom_PyState_To_State": pyfrom_pystate_to_state(indicator_name, fct_outputs, fct_states, fct_params),
                "Py_Signature": py_signature(fct_inputs, fct_params),
                "Py_Args": py_args(fct_inputs, fct_params),
                "Py_Outputs": py_outputs(fct_outputs),
                "Py_Define_Data": py_define_data(fct_inputs),
                "Py_Define_Outputs": py_define_outputs(fct_outputs),
                "Py_IntoFct_Input_Args": py_into_fct_input_args(fct_inputs, fct_params, fct_outputs),
                "Py_Results_Outputs": py_results_outputs(fct_outputs),
                "Py_Define_Outputs_PyHeap": py_define_outputs_pyheap(fct_outputs),
                "Py_IntoFct_Input_Args_PyHeap": py_into_fct_input_args_pyheap(fct_inputs, fct_params, fct_outputs),
                "Py_Results_Outputs_PyHeap": py_results_outputs_pyheap(fct_outputs),
                "Py_Next_Signature": py_next_signature(fct_inputs),
                "Py_Next_Args_Definition": py_next_args_definition(fct_inputs),
                "Py_Next_CreateSample": py_next_create_sample(indicator_camel_case, fct_inputs),
                "ImportSample": import_sample(indicator_camel_case, fct_inputs),
                "PyStub_State_Attributes": py_stub_state_attributes(indicator_name, fct_outputs, fct_states, fct_params),
                "PyStub_Results_Attributes": py_stub_results_attributes(indicator_name, fct_outputs),
                "PyStub_Args": py_stub_args(fct_inputs, fct_params),
                "PyStup_Next_Args": py_stub_next_args(fct_inputs),
                "Test_Fuzz_Define_Signature": test_fuzz_define_signature(fct_inputs, fct_params),
                "Test_Fuzz_Get_Inputs": test_fuzz_get_inputs(fct_inputs, fct_params),
                "Test_Fuzz_Set_Params": test_fuzz_set_params(fct_inputs, fct_params),
                "Test_Fuzz_Input_Fct": test_fuzz_input_fct(fct_inputs, fct_params),
                "Pytest_Input_Data": pytest_input_data(fct_inputs),
                "Pytest_Input_Param": pytest_input_param(fct_params),
                "Pytest_Input_Ohlcv": pytest_input_ohlcv(fct_inputs),
                "Pytest_Output": pytest_output(indicator_name, fct_outputs),
                "Pytest_State_Output": pytest_state_output(indicator_name, fct_outputs),
                "Test_Rust_Params": test_rust_params(fct_params),
                "Test_Rust_Get_Data_Input": test_rust_get_data_input(fct_inputs),
                "Test_Rust_Get_Expected": test_rust_get_expected(indicator_name, fct_outputs),
                "Test_Rust_Fct_Args": test_rust_fct_args(fct_inputs, fct_params),
                "Test_Rust_Check_outputs": test_rust_check_outputs(indicator_name, fct_outputs),
                "Test_Rust_Create_Sample": test_rust_create_sample(indicator_camel_case, fct_inputs),
                "Test_Rust_Check_Next_Outputs": test_rust_check_next_outputs(indicator_name, fct_outputs),
                "Test_Rust_Next_Overflow_Create_Sample": test_rust_next_overflow_create_sample(indicator_camel_case, fct_inputs),
                "Test_Rust_Dummy_Input_Args": test_rust_dummy_input_args(fct_inputs, fct_params),
                "Test_Rust_Dummy_Output_Args": test_rust_dummy_output_args(fct_outputs),
            }))
        logger.info(f"➕ Created {target_path}")

def add_to_bench_timeit(indicator_name: str):
    with open(BENCH_PYTHON_TIMEIT, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("from timeit_bench")), len(lines))
    lines.insert(insert_position, f"from timeit_bench.{indicator_name} import benchmark_{indicator_name}\n")

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("BENCHMARKS = {")), len(lines))
    lines.insert(insert_position+1, f"    '{indicator_name}': benchmark_{indicator_name},\n")

    with open(BENCH_PYTHON_TIMEIT, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {BENCH_PYTHON_TIMEIT}")

def add_to_core(indicator_name: str):
    with open(CORE_LIB_PATH, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("pub mod ")), len(lines))

    lines.insert(insert_position, f"pub mod {indicator_name};\n")

    with open(CORE_LIB_PATH, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {CORE_LIB_PATH}")

def add_to_pybindings(indicator_name: str, indicator_camel_case: str = None):
    with open(PYBINDING_LIB_PATH, 'r') as f:
        lines = f.readlines()

    wrap_insertion = next((i for i, line in enumerate(lines) if line.startswith("Ok(())")), len(lines))
    with open(INSERT_TO_PYLIB, 'r') as template_f:
        template_content = Template(template_f.read())
    lines.insert(wrap_insertion-2, template_content.substitute({
        "indicator_name": indicator_name,
        "IndicatorName": indicator_name.capitalize() if (indicator_camel_case is None) else indicator_camel_case,
    }))

    mod_insertion = next((i for i, line in enumerate(lines) if line.startswith("mod ")), len(lines))
    lines.insert(mod_insertion, f"mod py_{indicator_name};\n")

    with open(PYBINDING_LIB_PATH, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {PYBINDING_LIB_PATH}")

def add_to_python_stub_init(indicator_name: str):
    with open(PYTHON_STUB_INIT, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("from .")), len(lines))
    lines.insert(insert_position, f"from .{indicator_name} import *\n")

    with open(PYTHON_STUB_INIT, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {PYTHON_STUB_INIT}")

def add_to_fuzz_tests_cargo(indicator_name: str):
    with open(FUZZ_TESTS_CARGO, 'r') as f:
        lines = f.readlines()

    with open(INSERT_TO_FUZZ_CARGO, 'r') as template_f:
        template_content = Template(template_f.read())
    lines.insert(len(lines), template_content.substitute({
        "indicator_name": indicator_name,
    }))

    with open(FUZZ_TESTS_CARGO, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {FUZZ_TESTS_CARGO}")

def add_rust_test(indicator_name: str):
    with open(RUST_TEST_LIB_PATH, 'r') as f:
        lines = f.readlines()

    lines.insert(len(lines), f"#[cfg(test)]\npub(crate) mod tests_{indicator_name};\n")

    with open(RUST_TEST_LIB_PATH, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {RUST_TEST_LIB_PATH}")

def add_maptypes(indicator_name: str):
    with open(MAPTYPES_PYTHON, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("FCT_TO_NAMEDTUPLE = {")), len(lines))
    lines.insert(insert_position + 1, f'    "{indicator_name}": namedtuple("{indicator_name.capitalize()}Result", ["{indicator_name}", "state"]), #TODO: ADD OUTPUTS ARGS\n')

    with open(MAPTYPES_PYTHON, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {MAPTYPES_PYTHON}")

def add_rust_benchmark(indicator_name: str):
    with open(BENCH_RUST, 'r') as f:
        lines = f.readlines()
    insert_position = next((i for i, line in enumerate(lines) if line.startswith("criterion::criterion_main! {")), len(lines))
    lines.insert(insert_position + 1, f"indicators::bench_{indicator_name}::bench,")

    with open(BENCH_RUST, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {BENCH_RUST}")

def add_rust_benchmark_to_mod(indicator_name: str):
    with open(BENCH_RUST_MOD, 'r') as f:
        lines = f.readlines()
    lines.insert(len(lines), f"pub(crate) mod bench_{indicator_name};")

    with open(BENCH_RUST_MOD, 'w') as f:
        f.writelines(lines)

    logger.info(f"🔄 Updated {BENCH_RUST_MOD}")

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("name", type=str)
    parser.add_argument(
        "-c",
        "--camel_case",
        type=str
    )
    parser.add_argument(
        "-i",
        "--inputs",
        type=str,
        nargs="*",
        help="Inputs of the indicator, e.g. 'input1', 'input2'.",
        default=[]
    )
    parser.add_argument(
        "-o",
        "--outputs",
        type=str,
        nargs="*",
        help="Outputs of the indicator, e.g. 'value1', 'value2'.",
        default=[]
    )
    parser.add_argument(
        "-s",
        "--states",
        nargs="*",
        action=ParseKwargs,
        help="States of the indicator, e.g. 'state1=type1', 'state2=type2'.",
        default={}
    )
    parser.add_argument(
        "-p",
        "--parameters",
        nargs="*",
        action=ParseKwargs,
        help="Parameters of the indicator, e.g. 'param1=value1', 'param2=value2'.",
        default={}
    )
    return parser.parse_args()

def get_contributor_name() -> str:
    try:
        contributor_name = subprocess.check_output(
            ["git", "config", "user.name"], stderr=subprocess.DEVNULL
        ).decode().strip()
        if not contributor_name:
            contributor_name = DEFAULT_CONTRIBUTOR_NAME
    except Exception:
        contributor_name = DEFAULT_CONTRIBUTOR_NAME
    return contributor_name

def main():
    args = parse_args()
    logger.info(f"✨ Adding new indicator template: {args.name}")
    contributor_name = get_contributor_name()
    create_from_template(
        args.name,
        indicator_camel_case = args.camel_case,
        contributor_name = contributor_name,
        fct_inputs = args.inputs,
        fct_outputs = args.outputs,
        fct_states = args.states,
        fct_params = args.parameters
    )
    add_to_bench_timeit(args.name)
    add_to_python_stub_init(args.name)
    add_to_fuzz_tests_cargo(args.name)
    add_rust_test(args.name)
    add_to_core(args.name)
    add_to_pybindings(args.name, args.camel_case)
    add_maptypes(args.name)
    add_rust_benchmark(args.name)
    add_rust_benchmark_to_mod(args.name)
    logger.info(f"✅ Successfully added new indicator template: {args.name}")

if __name__ == "__main__":
    main()
