import rispy


def load_file(filepath):
    with open(filepath) as f:
        rispy.load(f)


def test_appenzeller_herzog(benchmark):
    benchmark(load_file, "benches/files/Appenzeller-Herzog_2019.ris")


def test_kwok(benchmark):
    benchmark(load_file, "benches/files/Kwok_2020.ris")