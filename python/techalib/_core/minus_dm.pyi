
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class MinusDmState:
    """State for the MinusDm computation"""
    prev_minus_dm: float
    prev_high: float
    prev_low: float
    period: int
    ...

class MinusDmResult(NamedTuple):
    """Result of the MinusDm computation"""
    minus_dm: NDArray
    state: MinusDmState

def minus_dm(
    high: NDArray,
    low: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> MinusDmResult | Tuple[NDArray, MinusDmState]:
    """
    MinusDm: Minus Directional Movement (MinusDM) Indicator
    ----------

    Parameters
    ----------
    high : NDArray
        The high prices of the asset.
    low : NDArray
        The low prices of the asset.
    period : int, default 14
        The period used for the calculation.
        Must be greater than 0.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    MinusDmResult
        A named tuple containing the result of the MinusDm computation.
        - minus_dm: NDArray
            The computed values.
        - state: `MinusDmState`
    """
    ...

def minus_dm_next(
    high: float,
    low: float,
    state: MinusDmState
) -> MinusDmState:
    """
    Update the MinusDm state with the next data.

    Parameters
    ----------
    high : float
        The next high price.
    low : float
        The next low price.
    state : MinusDmState
        The current state of the MinusDm computation.

    Returns
    -------
    MinusDmState
        The updated state after including the new value.
    """
    ...
