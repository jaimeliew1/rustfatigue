from rustfatigue import damage_equiv_load
import numpy as np

from rustfatigue.rustinterface import rainflow_count

test_vec = [
    40.0,
    15.0,
    72.0,
    22.0,
    43.0,
    82.0,
    75.0,
    7.0,
    34.0,
    49.0,
    95.0,
    75.0,
    85.0,
    47.0,
    63.0,
    31.0,
    90.0,
    20.0,
    37.0,
    39.0,
]


def test_damage_equiv_load():
    np.testing.assert_almost_equal(
        damage_equiv_load(test_vec, 4, 20), 46.10943506509813
    )


def test_whole_cycle_residuals():
    np.testing.assert_almost_equal(
        damage_equiv_load(test_vec, 4, 20, half=False), 53.37846821582299
    )


def test_rainflow_count():
    np.testing.assert_almost_equal(
        rainflow_count(test_vec, half=True),
        [
            50.0,
            50.0,
            10.0,
            10.0,
            16.0,
            16.0,
            59.0,
            59.0,
            25.0,
            67.0,
            75.0,
            88.0,
            75.0,
            19.0,
        ],
    )

def test_rainflow_count_whole_cycles():
    np.testing.assert_almost_equal(
        rainflow_count(test_vec, half=False),
        [
            50.0,
            50.0,
            10.0,
            10.0,
            16.0,
            16.0,
            59.0,
            59.0,
            25.0,
            25.0,
            67.0,
            67.0,
            75.0,
            75.0,
            88.0,
            88.0,
            75.0,
            75.0,
            19.0,
            19.0,
        ],
    )
