use crate::contract::{instantiate, reward_users};
use crate::msg::{InstantiateMsg, RewardUserRequest, RewardUserResponse};
use crate::state::{Campaign, CAMPAIGN, USER_REWARDS};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Addr, StdError, Uint128};
use std::vec;

#[test]
fn test_reward_users_fully_allocated() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    let resp = reward_users(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &[]),
        vec![
            RewardUserRequest {
                user_address: Addr::unchecked("user1"),
                amount: Uint128::new(750),
            },
            RewardUserRequest {
                user_address: Addr::unchecked("user2"),
                amount: Uint128::new(250),
            },
        ],
    )
    .unwrap();

    // assert response
    let user_responses: Vec<RewardUserResponse> =
        from_json(resp.data.unwrap()).unwrap();
    assert_eq!(
        user_responses,
        vec![
            RewardUserResponse {
                user_address: Addr::unchecked("user1"),
                success: true,
                error_msg: "".to_string(),
            },
            RewardUserResponse {
                user_address: Addr::unchecked("user2"),
                success: true,
                error_msg: "".to_string(),
            },
        ]
    );

    // assert inner state of the contract
    let campaign = CAMPAIGN.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        campaign,
        Campaign {
            owner: Addr::unchecked("owner"),
            unallocated_amount: Uint128::zero(),
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2")
            ],
        }
    );

    assert_eq!(
        USER_REWARDS
            .load(deps.as_ref().storage, Addr::unchecked("user1"))
            .unwrap(),
        Uint128::new(750)
    );

    assert_eq!(
        USER_REWARDS
            .load(deps.as_ref().storage, Addr::unchecked("user2"))
            .unwrap(),
        Uint128::new(250)
    );
}

#[test]
fn test_reward_users_as_manager() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    let resp = reward_users(
        deps.as_mut(),
        env.clone(),
        mock_info("manager1", &[]),
        vec![
            RewardUserRequest {
                user_address: Addr::unchecked("user1"),
                amount: Uint128::new(750),
            },
            RewardUserRequest {
                user_address: Addr::unchecked("user2"),
                amount: Uint128::new(250),
            },
        ],
    )
    .unwrap();

    // assert response
    let user_responses: Vec<RewardUserResponse> =
        from_json(resp.data.unwrap()).unwrap();
    assert_eq!(
        user_responses,
        vec![
            RewardUserResponse {
                user_address: Addr::unchecked("user1"),
                success: true,
                error_msg: "".to_string(),
            },
            RewardUserResponse {
                user_address: Addr::unchecked("user2"),
                success: true,
                error_msg: "".to_string(),
            },
        ]
    );

    // assert inner state of the contract
    let campaign = CAMPAIGN.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        campaign,
        Campaign {
            owner: Addr::unchecked("owner"),
            unallocated_amount: Uint128::zero(),
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2")
            ],
        }
    );

    assert_eq!(
        USER_REWARDS
            .load(deps.as_ref().storage, Addr::unchecked("user1"))
            .unwrap(),
        Uint128::new(750)
    );

    assert_eq!(
        USER_REWARDS
            .load(deps.as_ref().storage, Addr::unchecked("user2"))
            .unwrap(),
        Uint128::new(250)
    );
}

#[test]
fn test_fails_when_we_try_to_allocate_more_than_available() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("owner", &coins(1000, "")),
        InstantiateMsg {
            campaign_id: "campaign_id".to_string(),
            campaign_name: "campaign_name".to_string(),
            campaign_description: "campaign_description".to_string(),
            managers: vec![
                Addr::unchecked("manager1"),
                Addr::unchecked("manager2"),
            ],
        },
    )
    .unwrap();

    let resp = reward_users(
        deps.as_mut(),
        env.clone(),
        mock_info("manager1", &[]),
        vec![
            RewardUserRequest {
                user_address: Addr::unchecked("user1"),
                amount: Uint128::new(750),
            },
            RewardUserRequest {
                user_address: Addr::unchecked("user2"),
                amount: Uint128::new(250),
            },
            RewardUserRequest {
                user_address: Addr::unchecked("user3"),
                amount: Uint128::new(251),
            },
        ],
    );

    assert_eq!(
        resp,
        Err(StdError::generic_err("Not enough funds in the campaign",))
    );
}
