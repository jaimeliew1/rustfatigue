import numpy as np
from . import rustfatigue


def eq_load(signal, m, neq):
    return rustfatigue.eq_load_python(
        np.array(signal, dtype=np.float64), float(m), int(neq)
    )
