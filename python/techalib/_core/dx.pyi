
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class DxState:
    """State for the Dx computation"""
    prev_dx: float
    prev_true_range: float
    prev_plus_dm: float
    prev_minus_dm: float
    prev_high: float
    prev_low: float
    prev_close: float
    period: int
    ...

class DxResult(NamedTuple):
    """Result of the Dx computation"""
    dx: NDArray
    state: DxState

def dx(
    high: NDArray,
    low: NDArray,
    close: NDArray,
    period: int, # TODO: SET DEFAULT VALUE
    release_gil: bool = False
) -> DxResult | Tuple[NDArray, DxState]:
    # TODO: FILL THE DOCUMENTATION
    """
    Dx: ...
    ----------
    TODO: DESCRIPTION

    Parameters
    ----------
    TODO:ARG_NAME : TODO:ARG_TYPE
        TODO:DESCRIPTION

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    DxResult
        A named tuple containing the result of the Dx computation.
        - dx: NDArray
            The computed values.
        - state: `DxState`
    """
    ...

def dx_next(
    new_high: float,
    new_low: float,
    new_close: float,
    state: DxState
) -> DxState:
    """
    Update the Dx state with the next data.

    Parameters
    ----------
    TODO:ARG_NAME : TODO:ARG_TYPE
        TODO:DESCRIPTION

    state : DxState
        The current state of the Dx computation.

    Returns
    -------
    DxState
        The updated state after including the new value.
    """
    ...
