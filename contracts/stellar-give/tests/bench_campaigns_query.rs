use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, Address, String};

mod helpers;
use helpers::{register_and_setup, set_timestamp, single_ben};

// A campaign creator may own at most MAX_CAMPAIGNS_PER_CREATOR (10) campaigns,
// so benchmarks that populate the store with more than 10 campaigns must spread
// them across several creators. This many campaigns fit under one creator.
const PER_CREATOR: u32 = 10;

#[test]
fn bench_get_campaigns_paged_100_campaigns() {
    let (env, client, _creator, beneficiary, _donor, _admin, token_client, token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let target_amount = 10_000_000_i128;
    let deadline = 2_000_u64;

    env.budget().reset_default();

    // Rotate to a fresh, funded creator every PER_CREATOR campaigns.
    let mut creator = Address::generate(&env);
    token_admin_client.mint(&creator, &1_000_000_000_000);
    for i in 0..100u32 {
        if i > 0 && i % PER_CREATOR == 0 {
            creator = Address::generate(&env);
            token_admin_client.mint(&creator, &1_000_000_000_000);
        }
        let title = String::from_str(&env, &format!("Campaign {}", i));
        let metadata = String::from_str(&env, &format!("https://example.com/meta/{}", i));

        client.create_campaign(
            &creator,
            &bens,
            &title,
            &String::from_str(&env, "A test campaign description."),
            &metadata,
            &symbol_short!("relief"),
            &target_amount,
            &deadline,
            &token_client.address,
            &None,
        );
    }

    let cpu_before = env.budget().cpu_instruction_cost();
    let mem_before = env.budget().memory_bytes_cost();

    let campaigns = client.get_campaigns_paged(&0, &10);

    let cpu_used = env
        .budget()
        .cpu_instruction_cost()
        .saturating_sub(cpu_before);
    let mem_used = env
        .budget()
        .memory_bytes_cost()
        .saturating_sub(mem_before);

    println!("CPU instructions used: {}", cpu_used);
    println!("Memory bytes used: {}", mem_used);
    println!("Campaigns returned: {}", campaigns.len());

    assert_eq!(campaigns.len(), 10);
    assert!(cpu_used < 5_000_000, "CPU usage exceeded limit: {}", cpu_used);
}

#[test]
fn bench_get_campaigns_paged_with_limit_20() {
    let (env, client, _creator, beneficiary, _donor, _admin, token_client, token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let target_amount = 10_000_000_i128;
    let deadline = 2_000_u64;

    let mut creator = Address::generate(&env);
    token_admin_client.mint(&creator, &1_000_000_000_000);
    for i in 0..100u32 {
        if i > 0 && i % PER_CREATOR == 0 {
            creator = Address::generate(&env);
            token_admin_client.mint(&creator, &1_000_000_000_000);
        }
        let title = String::from_str(&env, &format!("Campaign {}", i));
        let metadata = String::from_str(&env, &format!("https://example.com/meta/{}", i));

        client.create_campaign(
            &creator,
            &bens,
            &title,
            &String::from_str(&env, "A test campaign description."),
            &metadata,
            &symbol_short!("relief"),
            &target_amount,
            &deadline,
            &token_client.address,
            &None,
        );
    }

    env.budget().reset_default();
    let cpu_before = env.budget().cpu_instruction_cost();
    let mem_before = env.budget().memory_bytes_cost();

    let campaigns = client.get_campaigns_paged(&0, &20);

    let cpu_used = env
        .budget()
        .cpu_instruction_cost()
        .saturating_sub(cpu_before);
    let mem_used = env
        .budget()
        .memory_bytes_cost()
        .saturating_sub(mem_before);

    println!("CPU instructions for 20-item query: {}", cpu_used);
    println!("Memory bytes used: {}", mem_used);
    println!("Campaigns returned: {}", campaigns.len());

    assert_eq!(campaigns.len(), 20);
    assert!(
        cpu_used < 8_000_000,
        "CPU usage exceeded limit for 20-item query: {}",
        cpu_used
    );
}

#[test]
fn bench_get_campaigns_by_id_batch() {
    let (env, client, _creator, beneficiary, _donor, _admin, token_client, token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let target_amount = 10_000_000_i128;
    let deadline = 2_000_u64;

    let mut campaign_ids = soroban_sdk::Vec::new(&env);

    let mut creator = Address::generate(&env);
    token_admin_client.mint(&creator, &1_000_000_000_000);
    for i in 0..50u32 {
        if i > 0 && i % PER_CREATOR == 0 {
            creator = Address::generate(&env);
            token_admin_client.mint(&creator, &1_000_000_000_000);
        }
        let title = String::from_str(&env, &format!("Campaign {}", i));
        let metadata = String::from_str(&env, &format!("https://example.com/meta/{}", i));

        let id = client.create_campaign(
            &creator,
            &bens,
            &title,
            &String::from_str(&env, "A test campaign description."),
            &metadata,
            &symbol_short!("relief"),
            &target_amount,
            &deadline,
            &token_client.address,
            &None,
        );

        campaign_ids.push_back(id);
    }

    env.budget().reset_default();
    let cpu_before = env.budget().cpu_instruction_cost();
    let mem_before = env.budget().memory_bytes_cost();

    let campaigns = client.get_campaigns(&campaign_ids);

    let cpu_used = env
        .budget()
        .cpu_instruction_cost()
        .saturating_sub(cpu_before);
    let mem_used = env
        .budget()
        .memory_bytes_cost()
        .saturating_sub(mem_before);

    println!("CPU instructions for batch query of 50 campaigns: {}", cpu_used);
    println!("Memory bytes used: {}", mem_used);

    assert_eq!(campaigns.len(), 50);
    assert!(
        cpu_used < 10_000_000,
        "CPU usage exceeded limit for batch query: {}",
        cpu_used
    );
}

#[test]
fn bench_get_campaigns_by_creator() {
    let (env, client, creator, beneficiary, _donor, _admin, token_client, _token_admin_client) =
        register_and_setup();
    set_timestamp(&env, 1_000);

    let bens = single_ben(&env, &beneficiary);
    let target_amount = 10_000_000_i128;
    let deadline = 2_000_u64;

    // A single creator can own at most PER_CREATOR campaigns, which is the
    // realistic upper bound for a by-creator query.
    for i in 0..PER_CREATOR {
        let title = String::from_str(&env, &format!("Campaign {}", i));
        let metadata = String::from_str(&env, &format!("https://example.com/meta/{}", i));

        client.create_campaign(
            &creator,
            &bens,
            &title,
            &String::from_str(&env, "A test campaign description."),
            &metadata,
            &symbol_short!("relief"),
            &target_amount,
            &deadline,
            &token_client.address,
            &None,
        );
    }

    env.budget().reset_default();
    let cpu_before = env.budget().cpu_instruction_cost();
    let mem_before = env.budget().memory_bytes_cost();

    let campaigns = client.get_campaigns_by_creator(&creator);

    let cpu_used = env
        .budget()
        .cpu_instruction_cost()
        .saturating_sub(cpu_before);
    let mem_used = env
        .budget()
        .memory_bytes_cost()
        .saturating_sub(mem_before);

    println!(
        "CPU instructions for get_campaigns_by_creator ({} campaigns): {}",
        PER_CREATOR, cpu_used
    );
    println!("Memory bytes used: {}", mem_used);
    println!("Campaigns returned: {}", campaigns.len());

    assert_eq!(campaigns.len(), PER_CREATOR);
    assert!(
        cpu_used < 15_000_000,
        "CPU usage exceeded limit for creator query: {}",
        cpu_used
    );
}
