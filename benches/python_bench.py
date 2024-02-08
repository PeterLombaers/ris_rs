import rispy
import ris


def load_file_rispy(filepath):
    with open(filepath) as f:
        rispy.load(f)


def load_file_rust(filepath):
    ris.parse(filepath)


def test_appenzeller_herzog_python(benchmark):
    benchmark(load_file_rispy, "benches/files/Appenzeller-Herzog_2019.ris")


# def test_kwok_python(benchmark):
#     benchmark(load_file_rispy, "benches/files/Kwok_2020.ris")


def test_appenzeller_herzog_rust(benchmark):
    benchmark(load_file_rust, "benches/files/Appenzeller-Herzog_2019.ris")


def test_kwok_rust(benchmark):
    benchmark(load_file_rust, "benches/files/Kwok_2020.ris")