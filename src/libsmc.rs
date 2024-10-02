use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

const ACTION_PLANT_PICTURE: u8 = 0;
const ACTION_WATERING: u8 = 1;
const ACTION_HARVEST: u8 = 2;

// Token rewards for different actions (Just for reference, not used)
const REWARD_PLANT: u64 = 10 * 1_000_000;    // 10 tokens for planting
const REWARD_WATERING: u64 = 15 * 1_000_000; // 15 tokens for watering
const REWARD_HARVEST: u64 = 25 * 1_000_000;  // 25 tokens for harvesting

// Premium subscription costs
const PREMIUM_COST_3_MONTHS: u64 = 300_000;  // Cost in token units
const PREMIUM_COST_7_MONTHS: u64 = 500_000;  // Cost in token units
const PREMIUM_COST_1_YEAR: u64 = 1_000_000;   // Cost in token units

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;

    // Parse the instruction data (e.g., action type, subscription period)
    let (action, subscription_period) = parse_instruction(instruction_data)?;

    // Process actions such as subscribing to premium plans or gardening activities
    match action {
        ACTION_PLANT_PICTURE => log_action(user_account, "planted a picture"),
        ACTION_WATERING => log_action(user_account, "watered a plant"),
        ACTION_HARVEST => log_action(user_account, "harvested a plant"),
        3 => process_premium_subscription(user_account, subscription_period),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn log_action(user_account: &AccountInfo, action_description: &str) -> ProgramResult {
    msg!(
        "User {} has {}.",
        user_account.key,
        action_description
    );

    Ok(())
}

fn parse_instruction(instruction_data: &[u8]) -> Result<(u8, u8), ProgramError> {
    if instruction_data.len() < 2 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let action = instruction_data[0];
    let subscription_period = instruction_data[1];

    Ok((action, subscription_period))
}

fn process_premium_subscription(user_account: &AccountInfo, subscription_period: u8) -> ProgramResult {
    let payment_amount = match subscription_period {
        0 => PREMIUM_COST_3_MONTHS,
        1 => PREMIUM_COST_7_MONTHS,
        2 => PREMIUM_COST_1_YEAR,
        _ => return Err(ProgramError::InvalidInstructionData),
    };

    msg!(
        "User {} paid {} for premium subscription (period: {} months)",
        user_account.key,
        payment_amount,
        match subscription_period {
            0 => 3,
            1 => 7,
            2 => 12,
            _ => 0,
        }
    );

    let expiry_time = calculate_expiry_time(subscription_period);

    msg!(
        "Premium subscription activated until timestamp {} for user {}",
        expiry_time,
        user_account.key
    );

    Ok(())
}

// Function to calculate expiry time based on subscription period
fn calculate_expiry_time(subscription_period: u8) -> u64 {
    let current_timestamp = 0;
    let additional_time = match subscription_period {
        0 => 90 * 24 * 60 * 60,  // 3 months in seconds
        1 => 210 * 24 * 60 * 60, // 7 months in seconds
        2 => 365 * 24 * 60 * 60, // 1 year in seconds
        _ => 0,
    };
    current_timestamp + additional_time
}
