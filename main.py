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

array = np.random.randint(0, 999, size=100_000_000)
data_pandas = pd.Series(array)
data_polars = pl.from_numpy(array)


start = time()
data_pandas.counters.is_prime_python()
end = time()

print(f"Time Python + Pandas: {end-start}")

start = time()
data_pandas.counters.is_prime_rust()
end = time()

print(f"Time Rust + Pandas: {end-start}")

start = time()
counts.is_prime_pl(data_polars, "column_0")
end = time()

print(f"Time Rust + Polars: {end-start}")