#[derive(PartialEq)]
pub enum ResultStatus {
    /// This is a correct match
    Success,
    /// This line is found in expected, but wasn't found in actual (too few lines)
    Missing,
    /// This line is found in actual, but wasn't found in expected (too many lines)
    Additional,
    /// The error is correct, but is throwed on an incorrect position
    IncorrectPosition,
    // The tree line or message mismatches between expected and actual
    Mismatch,
}

/// The result for a single line in the tree
pub struct TreeLineResult {
    /// Index (line number) inside the generated tree
    pub index: usize,
    /// Actual found line
    pub actual: String,
    /// Expected line
    pub expected: String,
    /// Result
    pub result: ResultStatus,
}

/// Defines a message that is generated on a certain line and column
pub struct ErrorMessagePosition {
    /// The error message
    pub message: String,
    /// The line on which the error was thrown by the parser
    pub line: usize,
    /// The column on which the error was thrown by the parser
    pub col: usize,
}

/// The result for a single error
pub struct ErrorResult {
    /// The actual error found during testing
    pub actual: ErrorMessagePosition,
    /// The expected error that should have been found
    pub expected: ErrorMessagePosition,
    /// Index of the error (0 based)
    pub index: usize,
    /// The actual result
    pub result: ResultStatus,
}

/// A combined result from a single test with all the result from the tree and parser errors
#[derive(Default)]
pub struct TestResult {
    /// Results of each line in the node tree (as generated by the parser)
    pub tree_results: Vec<TreeLineResult>,
    /// Results of each error generated by the parser
    pub error_results: Vec<ErrorResult>,
}

impl TestResult {
    /// Returns true when both results and error_results are either empty or have a result that is not Success
    pub fn is_success(&self) -> bool {
        self.tree_results
            .iter()
            .all(|r| r.result == ResultStatus::Success)
            && self
                .error_results
                .iter()
                .all(|r| r.result == ResultStatus::Success)
    }
}
