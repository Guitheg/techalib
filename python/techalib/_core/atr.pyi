
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class AtrState:
    """State for the Atr computation"""
    atr: float
    prev_close: float
    period: int
    ...

class AtrResult(NamedTuple):
    """Result of the Atr computation"""
    atr: NDArray
    state: AtrState

def atr(
    high: NDArray,
    low: NDArray,
    close: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> AtrResult | Tuple[NDArray, AtrState]:
    """
    Atr: Average True Range indicator.
    ----------

    Parameters
    ----------
    high : NDArray
        High prices.
    low : NDArray
        Low prices.
    close : NDArray
        Close prices.
    period : int, default 14
        The number of periods to use for the ATR calculation.
        Must be greater than 0.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    AtrResult
        A named tuple containing the result of the Atr computation.
        - atr: NDArray
            The computed values.
        - state: `AtrState`
    """
    ...

def atr_next(
    high: float,
    low: float,
    close: float,
    state: AtrState
) -> AtrState:
    """
    Update the Atr state with the next data.

    Parameters
    ----------
    high : float
        The current high price.
    low : float
        The current low price.
    close : float
        The current close price

    state : AtrState
        The current state of the Atr computation.

    Returns
    -------
    AtrState
        The updated state after including the new value.
    """
    ...
