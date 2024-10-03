use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{clock::Clock, Sysvar},
    program::{invoke, invoke_signed},
    program_pack::Pack,
};

const ACTION_PLANT_PICTURE: u8 = 0;
const ACTION_WATERING: u8 = 1;
const ACTION_HARVEST: u8 = 2;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;

    // Validate that the user_account is owned by the program
    if user_account.owner != program_id {
        msg!("Error: Account is not owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse the instruction data (e.g., action type, subscription period, and geolocation)
    let (action, subscription_period, geolocation) = parse_instruction(instruction_data)?;

    // Get the current timestamp from the Solana Clock Sysvar
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    // Process actions such as subscribing to premium plans or gardening activities
    match action {
        ACTION_PLANT_PICTURE => log_action_with_metadata(user_account, "planted a picture", timestamp, &geolocation),
        ACTION_WATERING => log_action_with_metadata(user_account, "watered a plant", timestamp, &geolocation),
        ACTION_HARVEST => log_action_with_metadata(user_account, "harvested a plant", timestamp, &geolocation),
        3 => process_premium_subscription(user_account, subscription_period),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Log action with metadata
fn log_action_with_metadata(user_account: &AccountInfo, action_description: &str, timestamp: i64, geolocation: &str) -> ProgramResult {
    let censored_geo = censor_geolocation(geolocation);
    msg!(
        "User {} has {} at timestamp {} with location {}",
        user_account.key,
        action_description,
        timestamp,
        censored_geo
    );
    Ok(())
}

// Censor geolocation string
fn censor_geolocation(geolocation: &str) -> String {
    let length = geolocation.len();
    if length < 4 {
        return geolocation.to_string(); // Don't censor if too short
    }
    let censor_length = 4.min(length); // Ensure at least 4 characters are censored
    let censored_part = "*".repeat(censor_length);
    format!("{}{}", &geolocation[..length - censor_length], censored_part)
}

// Parse instruction data with error handling
fn parse_instruction(instruction_data: &[u8]) -> Result<(u8, u8, String), ProgramError> {
    if instruction_data.len() < 3 {
        msg!("Error: Instruction data too short");
        return Err(ProgramError::InvalidInstructionData);
    }

    let action = instruction_data[0];
    let subscription_period = instruction_data[1];

    let geolocation = String::from_utf8(instruction_data[2..].to_vec()).map_err(|_| {
        msg!("Error: Invalid UTF-8 in geolocation data");
        ProgramError::InvalidInstructionData
    })?;

    Ok((action, subscription_period, geolocation))
}

// Process premium subscription
fn process_premium_subscription(user_account: &AccountInfo, subscription_period: u8) -> ProgramResult {
    msg!("Processing premium subscription for user: {}", user_account.key);
    let expiry_time = calculate_expiry_time(subscription_period);
    msg!("Subscription will expire at: {}", expiry_time);
    Ok(())
}

// Calculate subscription expiry time
fn calculate_expiry_time(subscription_period: u8) -> u64 {
    const SECONDS_PER_DAY: u64 = 86400;
    let days = subscription_period as u64 * 30;
    Clock::get().unwrap().unix_timestamp as u64 + days * SECONDS_PER_DAY
}
