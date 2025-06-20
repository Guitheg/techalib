Possible inputs
---
- indicator_name
- IndicatorName
- INDICATORNAME
- ContributorName (default: 'your git name' or 'Unknown')
- StructResult_DefinitionOutputs
- StructState_DefinitionOutputs
- StructState_DefinitionStates
- StructState_DefinitionParams
- StateUpdate_Checks
- StateUpdate_Next_Outputs
- StateUpdate_Next_Args
- StateUpdate_CheckOutputAndSet
- StateType
- StructSample
- StructSample_Definition
- Fct_Args
- Fct_OutputsInitialisation
- Fct_ArgsIn_IntoFct
- Fct_ResultCreation
- IntoFct_OuputsArgs
- IntoFct_Checks
- IntoFct_InitUnchecked_InputArgs
- IntoFct_InitUnchecked_Outputs
- IntoFct_InitUnchecked_CheckOutputsAndSet
- IntoFct_CheckFinitesIn
- IntoFct_CheckFinitesOut
- IntoFct_Next_Out
- IntoFct_Next_Args
- IntoFct_StateCreation
- InitUnchecked_Args
- InitUnchecked_ReturnType
- InitUnchecked_ReturnValues
- Next_Args
- Next_ReturnType
- Next_ReturnValues
- Bench_InputArgs_Init
- Bench_InputArgs
- Bench_Init_Param
- Bench_Data_Init_5M
- Bench_Data_Init_1M
- Bench_Data_Init_50K
- Bench_Data_Args
- Bench_Length
- Bench_Rust_Init_Data
- Bench_Rust_Input_Args
- PyState_Attributes_Definition
- PyState_New
- PyState_Creation
- PyFrom_State_To_PyState
- PyFrom_PyState_To_State
- Py_Signature
- Py_Args
- Py_Outputs
- Py_Define_Data
- Py_Define_Outputs
- Py_IntoFct_Input_Args
- Py_Define_Outputs_PyHeap
- Py_IntoFct_Input_Args_PyHeap
- Py_Results_Outputs_PyHeap
- Py_Next_Signature
- Py_Next_Args_Definition
- Py_Next_CreateSample
- ImportSample
- PyStub_State_Attributes
- PyStub_Results_Attributes
- PyStub_Args
- PyStup_Next_Args
- Test_Fuzz_Define_Signature
- Test_Fuzz_Get_Inputs
- Test_Fuzz_Set_Params
- Test_Fuzz_Input_Fct
- Pytest_Input_Data
- Pytest_Input_Param
- Pytest_Input_Ohlcv
- Pytest_Output
- Pytest_State_Output
- Test_Rust_Params
- Test_Rust_Get_Data_Input
- Test_Rust_Get_Expected
- Test_Rust_Fct_Args
- Test_Rust_Check_outputs
- Test_Rust_Create_Sample
- Test_Rust_Check_Next_Outputs
- Test_Rust_Next_Overflow_Create_Sample
- Test_Rust_Dummy_Input_Args
- Test_Rust_Dummy_Output_Args

Usage
---

Uses the `Possible inputs` in the template files inside `${}` such as:

```
/// ${INDICATORNAME} function
fn ${indicator_name}(
    //TODO
) -> ${IndicatorName}State {
    ...
}
```

The previous sample will produce with "ema" as `indicator name`:
```
/// EMA function
fn ema(
    //TODO
) -> EmaState {
    ...
}
```
