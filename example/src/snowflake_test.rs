#[cfg(test)]
mod tests {
    use mybatis::snowflake::new_snowflake_id;

    #[test]
    fn test_new_block_id() {
        println!("{}", new_snowflake_id());
        println!("{}", new_snowflake_id());
    }

    //cargo test --release --package mybatis --test snowflake_test test::test_bench_new_block_id --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    fn test_bench_new_block_id() {}
}
