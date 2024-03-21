
running 14 tests
test content_iter::tests::test_next ... ignored
test content_iter::tests::test_take_line ... ignored
test content_iter::tests::test_take_tag ... ignored
test parser::tests::test_eof ... ignored
test parser::tests::test_multiple_references ... ignored
test parser::tests::test_parse_reference ... ignored
test ref_iter::tests::test_after_end_tag ... ignored
test ref_iter::tests::test_before_start_tag ... ignored
test ref_iter::tests::test_bom ... ignored
test ref_iter::tests::test_double_byte_char ... ignored
test ref_iter::tests::test_empty_ref ... ignored
test ref_iter::tests::test_next ... ignored
test ref_iter::tests::test_take_line ... ignored
test ref_iter::tests::test_take_tag ... ignored

test result: ok. 0 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 0.00s

appenzeller_herzog_handwritten
                        time:   [6.3604 ms 6.3703 ms 6.3799 ms]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild

parse_reference         time:   [14.241 µs 14.301 µs 14.355 µs]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low severe
  3 (3.00%) high mild
  6 (6.00%) high severe

============================= test session starts ==============================
platform linux -- Python 3.11.8, pytest-8.1.1, pluggy-1.4.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=10 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /home/runner/work/ris_rs/ris_rs
configfile: pyproject.toml
plugins: benchmark-4.0.0
collected 5 items

benches/python_bench.py .F.FF                                            [100%]

=================================== FAILURES ===================================
_______________________________ test_kwok_python _______________________________

benchmark = <pytest_benchmark.fixture.BenchmarkFixture object at 0x7f25654c09d0>

    def test_kwok_python(benchmark):
>       benchmark(load_file_rispy, "benches/files/Kwok_2020.ris")

benches/python_bench.py:20: 
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:125: in __call__
    return self._raw(function_to_benchmark, *args, **kwargs)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:147: in _raw
    duration, iterations, loops_range = self._calibrate_timer(runner)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:275: in _calibrate_timer
    duration = runner(loops_range)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:90: in runner
    function_to_benchmark(*args, **kwargs)
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 

filepath = 'benches/files/Kwok_2020.ris'

    def load_file_rispy(filepath):
>       with open(filepath) as f:
E       FileNotFoundError: [Errno 2] No such file or directory: 'benches/files/Kwok_2020.ris'

benches/python_bench.py:6: FileNotFoundError
________________________________ test_kwok_rust ________________________________

benchmark = <pytest_benchmark.fixture.BenchmarkFixture object at 0x7f2565937c90>

    def test_kwok_rust(benchmark):
>       benchmark(load_file_rust, "benches/files/Kwok_2020.ris")

benches/python_bench.py:28: 
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:125: in __call__
    return self._raw(function_to_benchmark, *args, **kwargs)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:147: in _raw
    duration, iterations, loops_range = self._calibrate_timer(runner)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:275: in _calibrate_timer
    duration = runner(loops_range)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:90: in runner
    function_to_benchmark(*args, **kwargs)
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 

filepath = 'benches/files/Kwok_2020.ris'

    def load_file_rust(filepath):
>       with open(filepath, "rb") as f:
E       FileNotFoundError: [Errno 2] No such file or directory: 'benches/files/Kwok_2020.ris'

benches/python_bench.py:11: FileNotFoundError
_____________________ test_appenzeller_herzog_100_000_rust _____________________

benchmark = <pytest_benchmark.fixture.BenchmarkFixture object at 0x7f256580dfd0>

    def test_appenzeller_herzog_100_000_rust(benchmark):
>       benchmark(load_file_rust, "benches/files/AH_100_000.ris")

benches/python_bench.py:36: 
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:125: in __call__
    return self._raw(function_to_benchmark, *args, **kwargs)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:147: in _raw
    duration, iterations, loops_range = self._calibrate_timer(runner)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:275: in _calibrate_timer
    duration = runner(loops_range)
.venv/lib/python3.11/site-packages/pytest_benchmark/fixture.py:90: in runner
    function_to_benchmark(*args, **kwargs)
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 

filepath = 'benches/files/AH_100_000.ris'

    def load_file_rust(filepath):
>       with open(filepath, "rb") as f:
E       FileNotFoundError: [Errno 2] No such file or directory: 'benches/files/AH_100_000.ris'

benches/python_bench.py:11: FileNotFoundError

--------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in ms)                      Min                Max               Mean            StdDev             Median               IQR            Outliers      OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_appenzeller_herzog_rust       11.5947 (1.0)      13.1991 (1.0)      12.3766 (1.0)      0.4358 (1.0)      12.3500 (1.0)      0.7866 (1.0)          33;0  80.7977 (1.0)          74           1
test_appenzeller_herzog_python     40.8454 (3.52)     55.7211 (4.22)     46.7244 (3.78)     3.8589 (8.85)     47.3606 (3.83)     2.3165 (2.95)          7;7  21.4021 (0.26)         21           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
=========================== short test summary info ============================
FAILED benches/python_bench.py::test_kwok_python - FileNotFoundError: [Errno 2] No such file or directory: 'benches/files/Kwok_2020.ris'
FAILED benches/python_bench.py::test_kwok_rust - FileNotFoundError: [Errno 2] No such file or directory: 'benches/files/Kwok_2020.ris'
FAILED benches/python_bench.py::test_appenzeller_herzog_100_000_rust - FileNotFoundError: [Errno 2] No such file or directory: 'benches/files/AH_100_000.ris'
========================= 3 failed, 2 passed in 3.08s ==========================
