pub trait Day {
    // Used only for benchmarking purposes as a wrapper around an actual parse method
    // (that returns a domain-specific type)
    fn parse_bench(&self, input: &str) -> ();
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;
}
