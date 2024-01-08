#[cfg(test)]
mod tests {
    use crate::helpers::FlipCoinContract;
    use crate::msg::{InstantiateMsg, ExecuteMsg, BetSide};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_flip_coin() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, FlipCoinContract) {
        let mut app = mock_app();
        let flip_coin_id = app.store_code(contract_flip_coin());

        let msg = InstantiateMsg {};
        let flip_coin_contract_addr = app
            .instantiate_contract(
                flip_coin_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "Flip Coin",
                None,
            )
            .unwrap();

        let flip_coin_contract = FlipCoinContract(flip_coin_contract_addr);

        (app, flip_coin_contract)
    }

    mod flip_coin_tests {
        use super::*;
        
        #[test]
        fn place_bet_and_resolve() {
            let (mut app, flip_coin_contract) = proper_instantiate();

            // User places a bet on heads
            let bet_msg = ExecuteMsg::PlaceBet { side: BetSide::Heads, amount: 100 };
            let cosmos_msg = flip_coin_contract.call(bet_msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // Resolve the game
            let resolve_msg = ExecuteMsg::Resolve {};
            let cosmos_msg = flip_coin_contract.call(resolve_msg).unwrap();
            app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();

            // Additional assertions can be added here to check the state after the game is resolved
        }
    }
}
