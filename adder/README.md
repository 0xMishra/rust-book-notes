## commands for handling how tests are executed

1. If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the **--test-threads** flag and the number of threads you want to use to the test binary.
   `$ cargo test -- --test-threads=1`

2. If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with **--show-output**
   `$ cargo test -- --show-output`

3. Sometimes, running a full test suite can take a long time. If you’re working on code in a particular area, you might want to run only the tests pertaining to that code. You can choose which tests to run by passing cargo test the name or names of the test(s) you want to run as an argument.

   -- for running single test : `cargo test <test_function_name> `

4. We can specify part of a test name, and any test whose name matches that value will be run. For example, if two of our tests’ names contain `add`, we can run those two by running `cargo test add`.
