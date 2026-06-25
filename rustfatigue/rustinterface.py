import numpy as np
from . import rustfatigue
from typing import List


def damage_equiv_load(
    signal: List[float], m: float, neq: int, half: bool = True, max_closed: bool = False
):
    """
    Calculate the Damage Equivalent Load (DEL) for a given signal.

    Parameters:
        signal (array-like): The time-series load signal.
        m (float): Wöhler exponent.
        neq (int): Number of equivalent load cycles.
        half (bool): Whether to count the residual cycles as half (True) or
                     whole (False) cycles. (default=True).
        max_closed (bool): Whether to begin the peak/trough search from the
                    largest peak (default=False).

    Returns:
        float: The damage equivalent load.
    """
    if max_closed:
        return rustfatigue.eq_load_max_half_cycle_closed_python(
            np.array(signal, dtype=np.float64), float(m), int(neq), half
        )
    else:
        return rustfatigue.eq_load_python(
            np.array(signal, dtype=np.float64), float(m), int(neq), half
        )


def rainflow_count(signal: List[float], half: bool = True):
    """
    Calculate half-cycles as (mean, range) tuples using the 4-point rainflow
    counting algorithm for a given signal.

    Parameters:
        signal (array-like): The time-series load signal.
        half (bool): Whether to count the residual cycles as half (True) or
                     whole (False) cycles. (default=True).

    Returns:
        list[tuple[float, float]]: Half-cycles represented as (mean, range).
    """
    return rustfatigue.halfcycles(np.array(signal, dtype=np.float64), half)


# Backward-compatible alias
eq_load = damage_equiv_load
