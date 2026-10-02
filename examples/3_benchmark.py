"""
Benchmark: rainflow_count runtime across time series lengths, demonstrating
O(n) complexity.
"""

import time

import numpy as np

from rustfatigue.rustinterface import rainflow_count

N_SERIES_VALUES = 1000
N_SAMPLES_VALUES = [600, 6_000, 60_000]

rng = np.random.default_rng(seed=0)

print(f"{'N_SERIES_VALUES':>10} {'n_samples':>10} {'time (s)':>10}")
for n_samples in N_SAMPLES_VALUES:
    signals = rng.normal(size=(N_SERIES_VALUES, n_samples))

    start = time.perf_counter()
    for signal in signals:
        rainflow_count(signal)
    elapsed = time.perf_counter() - start

    print(f"{N_SERIES_VALUES:>10} {n_samples:>10} {elapsed:>10.4f}")
