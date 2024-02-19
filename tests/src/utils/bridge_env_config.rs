use super::consts::{PRIMARY_VALIDATOR_PK, SECONDARY_VALIDATOR_PK};

pub struct BridgeEnvConfig {
    pub primary_validator_pk: String,
    pub secondary_validator_pk: String,

    /// default: `0`
    pub yaro_fee_share: f64,
    /// default: `0`
    pub yusd_fee_share: f64,

    pub yaro_admin_fee: f64,
    pub yusd_admin_fee: f64,

    /// default: `100_000.0`
    pub yaro_admin_deposit: f64,
    /// default: `100_000.0`
    pub yusd_admin_deposit: f64,
}

impl Default for BridgeEnvConfig {
    fn default() -> Self {
        BridgeEnvConfig {
            yaro_fee_share: 0.0,
            yusd_fee_share: 0.0,

            yaro_admin_fee: 0.0,
            yusd_admin_fee: 0.0,

            yaro_admin_deposit: 100_000.0,
            yusd_admin_deposit: 100_000.0,

            primary_validator_pk: PRIMARY_VALIDATOR_PK.to_owned(),
            secondary_validator_pk: SECONDARY_VALIDATOR_PK.to_owned(),
        }
    }
}

impl BridgeEnvConfig {
    pub fn with_yaro_fee_share(mut self, yaro_fee_share: f64) -> Self {
        assert!((0.0..100.0).contains(&yaro_fee_share));

        self.yaro_fee_share = yaro_fee_share;
        self
    }

    pub fn with_yusd_fee_share(mut self, yusd_fee_share: f64) -> Self {
        assert!((0.0..100.0).contains(&yusd_fee_share));

        self.yusd_fee_share = yusd_fee_share;
        self
    }

    pub fn with_yaro_admin_fee(mut self, yaro_admin_fee: f64) -> Self {
        assert!((0.0..100.0).contains(&yaro_admin_fee));

        self.yaro_admin_fee = yaro_admin_fee;
        self
    }

    pub fn with_yusd_admin_fee(mut self, yusd_admin_fee: f64) -> Self {
        assert!((0.0..100.0).contains(&yusd_admin_fee));

        self.yusd_admin_fee = yusd_admin_fee;
        self
    }

    pub fn with_yaro_admin_deposit(mut self, yaro_admin_deposit: f64) -> Self {
        self.yaro_admin_deposit = yaro_admin_deposit;
        self
    }

    pub fn with_yusd_admin_deposit(mut self, yusd_admin_deposit: f64) -> Self {
        self.yusd_admin_deposit = yusd_admin_deposit;
        self
    }
}
