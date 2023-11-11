from setuptools import dist, setup

dist.Distribution().fetch_build_eggs(["setuptools_rust"])
from setuptools_rust import Binding, RustExtension  # noqa: E402

if __name__ == "__main__":
    setup(
        name="hal",
        rust_extensions=[
            RustExtension(
                ".hal_rs",
                path="Cargo.toml",
                binding=Binding.PyO3,
                native=True,
                debug=False,
            )
        ],
    )
