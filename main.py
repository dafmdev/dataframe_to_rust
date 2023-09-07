import pandas as pd
import numpy as np
import polars as pl
from time import time
import counts

def is_prime(n):
    if n <= 1:
        return False
    for a in range(2, n):
        if n % a == 0:
            return False
    return True

@pd.api.extensions.register_series_accessor("counters")
class CountersAccesor:
    def __init__(self, series):
        self._series = series
        self._series_list = series.to_list()

    def is_prime_python(self):
        return self._series.apply(is_prime)

    def is_prime_rust(self):
        return counts.is_prime(self._series_list)

data = pd.Series(np.random.randint(0, 999, size=100_000_000))

start = time()
data.counters.is_prime_python()
end = time()

print(f"Time Python: {end-start}")

start = time()
data.counters.is_prime_rust()
end = time()

print(f"Time Rust: {end-start}")