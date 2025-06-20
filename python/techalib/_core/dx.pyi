
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
    period: int = 14,
    release_gil: bool = False
) -> DxResult | Tuple[NDArray, DxState]:
    """
    Dx: Directional Movement Index
    ----------

    Parameters
    ----------
    high : NDArray
        The high prices of the asset.
    low : NDArray
        The low prices of the asset.
    close : NDArray
        The close prices of the asset.
    period : int, default 14
        The period over which to compute the Dx.
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
    new_high : float
        The new high price.
    new_low : float
        The new low price.
    new_close : float
        The new close price.
    state : DxState
        The current state of the Dx computation.

    Returns
    -------
    DxState
        The updated state after including the new value.
    """
    ...
