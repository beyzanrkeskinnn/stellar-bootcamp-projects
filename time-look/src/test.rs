#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_deposit_and_claim() {
        let env = Env::default();

        let contract_id = env.register_contract(None, ClaimableBalanceContract);
        let client = ClaimableBalanceContractClient::new(&env, &contract_id);

        let user1 = Address::random(&env);
        let user2 = Address::random(&env);
        let token = Address::random(&env);

        // Mock token transfer için yetkilendirme
        user1.require_auth();
        user2.require_auth();

        // Set up time bound
        let time_bound = TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: env.ledger().timestamp() + 1000, // Gelecekte
        };

        // Deposit işlemi
        client.deposit(&user1, &token, &1000, &vec![&env, user2.clone()], &time_bound);

        // Claim öncesi (doğru zaman)
        client.claim(&user2);

        // Claim sonrası tekrar claim yaparsan hata beklenir
        // assert panik testine girilebilir
    }
}