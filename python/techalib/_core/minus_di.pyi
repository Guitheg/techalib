
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class MinusDiState:
    """State for the MinusDi computation"""
    prev_minus_di: float
    prev_minus_dm: float
    prev_true_range: float
    prev_high: float
    prev_low: float
    prev_close: float
    period: int
    ...

class MinusDiResult(NamedTuple):
    """Result of the MinusDi computation"""
    minus_di: NDArray
    state: MinusDiState

def minus_di(
    high: NDArray,
    low: NDArray,
    close: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> MinusDiResult | Tuple[NDArray, MinusDiState]:
    """
    MinusDi: Minus Directional Indicator
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
        The period over which to calculate the Minus Directional Indicator.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    MinusDiResult
        A named tuple containing the result of the MinusDi computation.
        - minus_di: NDArray
            The computed values.
        - state: `MinusDiState`
    """
    ...

def minus_di_next(
    new_high: float,
    new_low: float,
    new_close: float,
    state: MinusDiState
) -> MinusDiState:
    """
    Update the MinusDi state with the next data.

    Parameters
    ----------
    new_high : float
        The new high price to include in the computation.
    new_low : float
        The new low price to include in the computation.
    new_close : float
        The new close price to include in the computation.
    state : MinusDiState
        The current state of the MinusDi computation.

    Returns
    -------
    MinusDiState
        The updated state after including the new value.
    """
    ...
