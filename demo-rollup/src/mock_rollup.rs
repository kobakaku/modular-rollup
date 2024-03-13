use async_trait::async_trait;
use sov_modules_rollup_blueprint::RollupBlueprint;

pub struct MockRollup {}

#[async_trait]
impl RollupBlueprint for MockRollup {}
