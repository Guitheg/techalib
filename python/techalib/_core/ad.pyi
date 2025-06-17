
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class AdState:
    """State for the Ad computation"""
    ad: float
    ...

class AdResult(NamedTuple):
    """Result of the Ad computation"""
    ad: NDArray
    state: AdState

def ad(
    high: NDArray,
    low: NDArray,
    close: NDArray,
    volume: NDArray,
    release_gil: bool = False
) -> AdResult | Tuple[NDArray, AdState]:
    """
    AD: Chaikin Accumulation/Distribution Line
    ----------

    Parameters
    ----------
    high : NDArray
        High prices for the period.
    low : NDArray
        Low prices for the period.
    close : NDArray
        Close prices for the period.
    volume : NDArray
        Volume for the period.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    AdResult
        A named tuple containing the result of the Ad computation.
        - ad: NDArray
            The computed values.
        - state: `AdState`
    """
    ...

def ad_next(
    high: float,
    low: float,
    close: float,
    volume: float,
    state: AdState
) -> AdState:
    """
    Update the Ad state with the next data.

    Parameters
    ----------
    high : float
        The current high price.
    low : float
        The current low price.
    close : float
        The current close price.
    volume : float
        The current volume.
    state : AdState
        The current state of the Ad computation.

    Returns
    -------
    AdState
        The updated state after including the new value.
    """
    ...
