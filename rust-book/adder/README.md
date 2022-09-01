## commands for handling how tests are executed

1. If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the **--test-threads** flag and the number of threads you want to use to the test binary.
   `$ cargo test -- --test-threads=1`

2. If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with **--show-output**
   `$ cargo test -- --show-output`

3. Sometimes, running a full test suite can take a long time. If you’re working on code in a particular area, you might want to run only the tests pertaining to that code. You can choose which tests to run by passing cargo test the name or names of the test(s) you want to run as an argument.

   - for running single test : `cargo test <test_function_name> `

4. We can specify part of a test name, and any test whose name matches that value will be run. For example, if two of our tests’ names contain `add`, we can run those two by running `cargo test add`.

## types of tests

1. ### unit tests

   - The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.

   - You’ll put unit tests in the src directory in each file with the code that they’re testing.

   - The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

2. ### integration tests

   - In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API

   - Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well.

   - To create integration tests, you first need a `tests` directory in the root level of your project alongside`src` directory .
