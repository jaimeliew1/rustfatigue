# RustFatigue
RustFatigue is a lightweight Python package designed to calculate only one thing -  the Damage Equivalent Load (DEL) from time series load data. DEL is an important metric in fatigue analysis, frequently used in the wind turbine industry and other engineering fields to estimate cumulative damage under variable loading conditions.

The core computation is implemented in Rust for high performance, wrapped with a Python interface for seamless integration into Python workflows.

# Installation

```bash
pip install rust-fatigue
```
# Usage
This package provides a single function which returns the DEL of a signal:
```python
damage_equiv_load(signal: list[float], m: float, neq: int, half: bool=True) -> float
```
- `signal`: List or NumPy array of load values.
- `m`: Wöhler (fatigue) exponent.
- `neq`: Number of equivalent full-cycles.
- `half`: Whether to count residual cycles as half-cycles (True, default) or full cycles (False).

# Example
```python
from rustfatigue import damage_equiv_load

# Example load signal
signal = [1, 2, 3, 4, 5, 6]
wohler = 4
Ncycles = 20

# Calculate Damage Equivalent Load
DEL = damage_equiv_load(signal, wohler, Ncycles)

print(f"Damage Equivalent Load: {DEL}")
# Damage Equivalent Load: 2.224235024089319
```

By default, residual cycles are counted as half-cycles (which is standard practice).
If you prefer a more conservative estimate by counting residuals as full cycles, set `half=False`:


```
DEL = damage_equiv_load(signal, wohler, Ncycles, half=False)
```

# Theory

The calculation of Damage Equivalent Load (DEL) in this package involves three main steps:

### 1) Peak and Trough Finding
The input time series load signal is scanned to identify local maxima (peaks) and minima (troughs). This results in a compressed signal that contains only these reversal points, significantly reducing the data size while retaining the essential fatigue-driving features.

### 2) Rainflow Counting 
The sequence of peaks and troughs is processed using a [rainflow counting algorithm](https://en.wikipedia.org/wiki/Rainflow-counting_algorithm). This algorithm decomposes the load history into discrete load cycles by matching turning points. The result is a set of half-cycle ranges — each representing a peak-to-peak stress difference. 
In this package, Rainflow counting is performed in two steps.
1) Full cycle ranges (i.e. two matching half-cycles) are identified using the four point method.
2) Any remaining unpaired turning points are counted as half cycles - or optionally as full-cycles for a more conservative estimate.

### 3) Damage Equivalent Load Calculation
From the rainflow output, the DEL is computed using the Wöhler (S–N) exponent $m$ and a target number of equivalent cycles $N_{\text{eq}}$. The calculation assumes that fatigue damage follows a power law, and each cycle contributes damage proportional to its range raised to the power $m$. The DEL $L_{\text{eq}}$ is the constant-amplitude load that would cause the same cumulative damage over $N_{\text{eq}}$ cycles:

$$
L_{\text{eq}} = \left( \frac{1}{2}\frac{\sum_{i} L_i^m}{N_{\text{eq}}} \right)^{\frac{1}{m}}
$$

Where
- $L_i$ = range (peak-to-peak) of the $i$-th **half-cycle**
- $N_{\text{eq}}$ = number of equivalent full cycles
- $m$ = Wöhler (fatigue) exponent
- The factor $1/2$ accounts for the conversion from half-cycles to full cycles
  
In many textbooks and industry tools, cycle ranges are binned, and there is an additional weighting on $L_i$ to distinguish between half and full cycles. By contrast, this implementation uses no binning and operates directly on half-cycles, where a full cycle is simply represented by two matching half-cycle. This is mathematically equivalent to using an infinite number of bins and avoids the loss of precision introduced by histogram-based methods.
