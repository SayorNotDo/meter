use crate::context::seeder::SeedDbTestContext;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_login(_ctx: &mut SeedDbTestContext) {}
