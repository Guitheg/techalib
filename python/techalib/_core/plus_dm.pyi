
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class PlusDmState:
    """State for the PlusDm computation"""
    prev_plus_dm: float
    prev_high: float
    prev_low: float
    period: int
    ...

class PlusDmResult(NamedTuple):
    """Result of the PlusDm computation"""
    plus_dm: NDArray
    state: PlusDmState

def plus_dm(
    high: NDArray,
    low: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> PlusDmResult | Tuple[NDArray, PlusDmState]:
    """
    PlusDm: Plus Directional Movement (PlusDM) Indicator
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
    PlusDmResult
        A named tuple containing the result of the PlusDm computation.
        - plus_dm: NDArray
            The computed values.
        - state: `PlusDmState`
    """
    ...

def plus_dm_next(
    high: float,
    low: float,
    state: PlusDmState
) -> PlusDmState:
    """
    Update the PlusDm state with the next data.

    Parameters
    ----------
    high : float
        The next high price.
    low : float
        The next low price.
    state : PlusDmState
        The current state of the PlusDm computation.

    Returns
    -------
    PlusDmState
        The updated state after including the new value.
    """
    ...
