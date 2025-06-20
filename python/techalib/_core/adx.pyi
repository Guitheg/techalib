
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class AdxState:
    """State for the Adx computation"""
    prev_adx: float
    prev_true_range: float
    prev_plus_dm: float
    prev_minus_dm: float
    prev_high: float
    prev_low: float
    prev_close: float
    period: int
    ...

class AdxResult(NamedTuple):
    """Result of the Adx computation"""
    adx: NDArray
    state: AdxState

def adx(
    high: NDArray,
    low: NDArray,
    close: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> AdxResult | Tuple[NDArray, AdxState]:
    """
    Adx: Average Directional Index
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
        The period over which to compute the Adx.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    AdxResult
        A named tuple containing the result of the Adx computation.
        - adx: NDArray
            The computed values.
        - state: `AdxState`
    """
    ...

def adx_next(
    new_high: float,
    new_low: float,
    new_close: float,
    state: AdxState
) -> AdxState:
    """
    Update the Adx state with the next data.

    Parameters
    ----------
    new_high : float
        The new high price.
    new_low : float
        The new low price.
    new_close : float
        The new close price.
    state : AdxState
        The current state of the Adx computation.

    Returns
    -------
    AdxState
        The updated state after including the new value.
    """
    ...
