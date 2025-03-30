# test-app

OpenTestGen: Open-Source Code-to-Test Linker & Automated Test Generator
Description:
OpenTestGen is an open-source tool that tracks code-to-test relationships and automatically generates missing test cases for C++ (Google Test, Catch2) and Python (pytest). It integrates with Git to analyze changes, ensuring that modified functions always have corresponding tests.

Key Features:
-  parse codebase and list/group external depandcies, librraies, imports
-  gnerate template build scripts (cmake, setup, gradle ) based on code analysis
-  look for extranl depancides in package mangers and give recommendation to integate them 
-  fro the diffrent languages
-  write unitests cases for functions , classes, and data structs, 
-  write template testing piplines in 
-  gnerate a function-test linked test table to view the covergance of the test suite, 
-  identify lacks in function testing
-  
-  Code-to-Test Mapping: Links functions to their existing test cases.
-  Git-Aware Test Selection: Detects code changes and reruns only relevant tests.
-  Automated Test Case Generation: Writes missing test cases in Google Test, Catch2, and pytest.
-  Static Analysis Integration: Ensures full test coverage by detecting untested functions.
-  Cross-Language Support: Works with C++ and Python, expanding beyond Java/.NET solutions.
- 

Why It’s Better?
🔹 No existing tool combines test tracking & test generation for C++ & Python.
🔹 Open-source & extensible, unlike closed-source alternatives.
🔹 Works with existing CI/CD pipelines (GitHub Actions, Jenkins).
