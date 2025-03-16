# test-app

OpenTestGen: Open-Source Code-to-Test Linker & Automated Test Generator
Description:
OpenTestGen is an open-source tool that tracks code-to-test relationships and automatically generates missing test cases for C++ (Google Test, Catch2) and Python (pytest). It integrates with Git to analyze changes, ensuring that modified functions always have corresponding tests.

Key Features:
✅ Code-to-Test Mapping: Links functions to their existing test cases.
✅ Git-Aware Test Selection: Detects code changes and reruns only relevant tests.
✅ Automated Test Case Generation: Writes missing test cases in Google Test, Catch2, and pytest.
✅ Static Analysis Integration: Ensures full test coverage by detecting untested functions.
✅ Cross-Language Support: Works with C++ and Python, expanding beyond Java/.NET solutions.

Why It’s Better?
🔹 No existing tool combines test tracking & test generation for C++ & Python.
🔹 Open-source & extensible, unlike closed-source alternatives.
🔹 Works with existing CI/CD pipelines (GitHub Actions, Jenkins).
