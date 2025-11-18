"""
Pytest configuration for FMU integration tests
"""

import pytest
from pathlib import Path


def pytest_configure(config):
    """Register custom markers"""
    config.addinivalue_line(
        "markers", "slow: marks tests as slow (deselect with '-m \"not slow\"')"
    )
    config.addinivalue_line(
        "markers", "fmu_required: marks tests that require FMU files to be built"
    )


def pytest_collection_modifyitems(config, items):
    """Automatically mark FMU tests"""
    for item in items:
        # Mark all tests as requiring FMU
        if "fmu" in str(item.fspath).lower():
            item.add_marker(pytest.mark.fmu_required)


@pytest.fixture(scope="session")
def fmu_directory():
    """Get the FMU directory path"""
    return Path(__file__).parent.parent.parent / "fmus"


@pytest.fixture(scope="session", autouse=True)
def check_fmu_directory(fmu_directory):
    """Check if FMU directory exists and warn if empty"""
    if not fmu_directory.exists():
        pytest.skip(
            f"FMU directory not found: {fmu_directory}\n"
            "Build FMUs first using: ./scripts/build-fmu.sh"
        )

    fmu_files = list(fmu_directory.glob("*.fmu"))
    if not fmu_files:
        pytest.skip(
            f"No FMU files found in {fmu_directory}\n"
            "Build FMUs first using: ./scripts/build-fmu.sh"
        )
