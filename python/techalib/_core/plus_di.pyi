
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class PlusDiState:
    """State for the PlusDi computation"""
    prev_plus_di: float
    prev_plus_dm: float
    prev_true_range: float
    prev_high: float
    prev_low: float
    prev_close: float
    period: int
    ...

class PlusDiResult(NamedTuple):
    """Result of the PlusDi computation"""
    plus_di: NDArray
    state: PlusDiState

def plus_di(
    high: NDArray,
    low: NDArray,
    close: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> PlusDiResult | Tuple[NDArray, PlusDiState]:
    """
    PlusDi: Plus Directional Indicator (PlusDI)
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
        The period over which to compute the PlusDi.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    PlusDiResult
        A named tuple containing the result of the PlusDi computation.
        - plus_di: NDArray
            The computed values.
        - state: `PlusDiState`
    """
    ...

def plus_di_next(
    new_high: float,
    new_low: float,
    new_close: float,
    state: PlusDiState
) -> PlusDiState:
    """
    Update the PlusDi state with the next data.

    Parameters
    ----------
    new_high : float
        The new high price to include in the computation.
    new_low : float
        The new low price to include in the computation.
    new_close : float
        The new close price to include in the computation.
    state : PlusDiState
        The current state of the PlusDi computation.

    Returns
    -------
    PlusDiState
        The updated state after including the new value.
    """
    ...
