import numpy as np
from . import rustfatigue
from typing import List


def damage_equiv_load(signal: List[float], m: float, neq: int, half: bool = True):
    """
    Calculate the Damage Equivalent Load (DEL) for a given signal.

    Parameters:
        signal (array-like): The time-series load signal.
        m (float): Wöhler exponent.
        neq (int): Number of equivalent load cycles.
        half (bool): Whether to count the residual cycles as half (True) or
                     whole (False) cycles. (default=True).

    Returns:
        float: The damage equivalent load.
    """
    return rustfatigue.eq_load_python(
        np.array(signal, dtype=np.float64), float(m), int(neq), half
    )


def rainflow_count(signal: List[float], half: bool = True):
    """
    Calculate a list of half-cycle ranges using the 4-point rainflow counting
    algorithm for a given signal.

    Parameters:
        signal (array-like): The time-series load signal.
        half (bool): Whether to count the residual cycles as half (True) or
                     whole (False) cycles. (default=True).

    Returns:
        list[float]: The damage equivalent load.
    """
    return rustfatigue.halfcycles(np.array(signal, dtype=np.float64), half)


# Backward-compatible alias
eq_load = damage_equiv_load
