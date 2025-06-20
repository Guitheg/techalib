
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class RocrState:
    """State for the Rocr computation"""
    prev_rocr: float
    prev_roc_window: List[float]
    period: int
    ...

class RocrResult(NamedTuple):
    """Result of the Rocr computation"""
    rocr: NDArray
    state: RocrState

def rocr(
    data: NDArray,
    period: int = 10,
    release_gil: bool = False
) -> RocrResult | Tuple[NDArray, RocrState]:
    """
    Rocr: Rate of change ratio (ROCR) indicator.
    ----------

    Parameters
    ----------
    data : NDArray
        Input data array, typically a price series.
    period : int, default 10
        The number of periods over which to calculate the ROCR.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    RocrResult
        A named tuple containing the result of the Rocr computation.
        - rocr: NDArray
            The computed values.
        - state: `RocrState`
    """
    ...

def rocr_next(
    new_value: float,
    state: RocrState
) -> RocrState:
    """
    Update the Rocr state with the next data.

    Parameters
    ----------
    new_value : float
        The new value to include in the Rocr computation
    state : RocrState
        The current state of the Rocr computation.

    Returns
    -------
    RocrState
        The updated state after including the new value.
    """
    ...
