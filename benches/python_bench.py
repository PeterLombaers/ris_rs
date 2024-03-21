import rispy
import ris


def load_file_rispy(filepath):
    with open(filepath) as f:
        rispy.load(f)


def load_file_rust(filepath):
    with open(filepath, "rb") as f:
        ris.parse(f.read())


def test_appenzeller_herzog_python(benchmark):
    benchmark(load_file_rispy, "benches/files/Appenzeller-Herzog_2019.ris")


def test_kwok_python(benchmark):
    benchmark(load_file_rispy, "benches/files/Brouwer_2019.ris")


def test_appenzeller_herzog_rust(benchmark):
    benchmark(load_file_rust, "benches/files/Appenzeller-Herzog_2019.ris")


def test_kwok_rust(benchmark):
    benchmark(load_file_rust, "benches/files/Brouwer_2019.ris")


# def test_appenzeller_herzog_100_000_python(benchmark):
#     benchmark(load_file_rispy, "benches/files/AH_100_000.ris")


# def test_appenzeller_herzog_100_000_rust(benchmark):
#     benchmark(load_file_rust, "benches/files/AH_100_000.ris")
