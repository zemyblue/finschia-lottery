#[cfg(test)]
mod tests {
    use crate::helpers::FsLotteryContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::queries::query,
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
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, FsLotteryContract) {
        let mut app = mock_app();
        let fs_lottery_id = app.store_code(contract_template());

        let msg = InstantiateMsg { 
            use_denom: "cony".to_string(),
            exchange_ratio: 10, 
            min_exchange_amount: 200000000u32, 
            first_winner_ratio: 60u8, 
            second_winner_ratio: 20u8, 
            owner_ratio: 2u8, 
            token_name: "lottery".to_string(), 
            token_symbol: "LTT".to_string(), 
            token_decimals: 6u8, 
        };
        let fs_lottery_contract_addr = app
            .instantiate_contract(
                fs_lottery_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = FsLotteryContract(fs_lottery_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // let msg = ExecuteMsg::Increment {};
            let msg = ExecuteMsg::Invest {};
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
