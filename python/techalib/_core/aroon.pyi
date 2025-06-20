
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple, List

from numpy.typing import NDArray

@dataclass(frozen=True)
class AroonState:
    """State for the Aroon computation"""
    prev_aroon_down: float
    prev_aroon_up: float
    prev_high_window: List[float]
    prev_low_window: List[float]
    period: int
    ...

class AroonResult(NamedTuple):
    """Result of the Aroon computation"""
    aroon_down: NDArray
    aroon_up: NDArray
    state: AroonState

def aroon(
    high: NDArray,
    low: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> AroonResult | Tuple[NDArray, NDArray, AroonState]:
    """
    Aroon: Aroon Indicator
    ----------

    Parameters
    ----------
    high : NDArray
        The high prices of the asset.
    low : NDArray
        The low prices of the asset.
    period : int, default 14
        The number of periods to consider for the Aroon calculation.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    AroonResult
        A named tuple containing the result of the Aroon computation.
        - aroon: NDArray
            The computed values.
        - state: `AroonState`
    """
    ...

def aroon_next(
    new_high: float,
    new_low: float,
    state: AroonState
) -> AroonState:
    """
    Update the Aroon state with the next data.

    Parameters
    ----------
    new_high : float
        The new high price to include in the Aroon calculation.
    new_low : float
        The new low price to include in the Aroon calculation.
    state : AroonState
        The current state of the Aroon computation.

    Returns
    -------
    AroonState
        The updated state after including the new value.
    """
    ...
