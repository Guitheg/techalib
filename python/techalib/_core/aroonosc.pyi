
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class AroonoscState:
    """State for the Aroonosc computation"""
    prev_aroonosc: float
    prev_high_window: List[float]
    prev_low_window: List[float]
    period: int
    ...

class AroonoscResult(NamedTuple):
    """Result of the Aroonosc computation"""
    aroonosc: NDArray
    state: AroonoscState

def aroonosc(
    high: NDArray,
    low: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> AroonoscResult | Tuple[NDArray, AroonoscState]:
    """
    Aroonosc: Aroon Oscillator
    ----------

    Parameters
    ----------
    high : NDArray
        The high prices of the asset.
    low : NDArray
        The low prices of the asset.
    period : int, default 14
        The number of periods to consider for the Aroonosc calculation.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    AroonoscResult
        A named tuple containing the result of the Aroonosc computation.
        - aroonosc: NDArray
            The computed values.
        - state: `AroonoscState`
    """
    ...

def aroonosc_next(
    new_high: float,
    new_low: float,
    state: AroonoscState
) -> AroonoscState:
    """
    Update the Aroonosc state with the next data.

    Parameters
    ----------
    new_high : float
        The new high price to include in the Aroonosc computation.
    new_low : float
        The new low price to include in the Aroonosc computation.
    state : AroonoscState
        The current state of the Aroonosc computation.

    Returns
    -------
    AroonoscState
        The updated state after including the new value.
    """
    ...
