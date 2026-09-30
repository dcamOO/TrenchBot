// Non-creation instructions from pump-fun/pump-public-docs/idl/pump.json (2026-09-29).
// Unknown instructions fail closed until reviewed against a new IDL.
pub const NON_CREATION: &[[u8; 8]] = &[
    [228, 69, 165, 46, 81, 203, 154, 29], // Anchor EVENT_IX_TAG_LE (lang/src/event.rs)
    [2, 14, 61, 138, 170, 142, 14, 95],   // add_quote_control_mint
    [111, 121, 21, 56, 40, 24, 94, 209],  // add_quote_mint
    [125, 126, 214, 134, 77, 229, 188, 89], // admin_cto
    [8, 217, 96, 231, 144, 104, 192, 5],  // admin_set_idl_authority
    [209, 11, 115, 87, 213, 23, 124, 204], // admin_update_token_incentives
    [102, 6, 61, 18, 1, 218, 235, 234],   // buy
    [194, 171, 28, 70, 104, 77, 91, 47],  // buy_exact_quote_in_v2
    [56, 252, 116, 8, 158, 223, 205, 95], // buy_exact_sol_in
    [184, 23, 238, 97, 103, 197, 211, 61], // buy_v2
    [37, 58, 35, 126, 190, 53, 228, 197], // claim_cashback
    [122, 243, 204, 65, 94, 116, 29, 55], // claim_cashback_v2
    [16, 4, 71, 28, 204, 1, 40, 27],      // claim_token_incentives
    [249, 69, 164, 218, 150, 103, 84, 138], // close_user_volume_accumulator
    [20, 22, 86, 123, 198, 28, 219, 132], // collect_creator_fee
    [207, 17, 138, 242, 4, 34, 19, 56],   // collect_creator_fee_v2
    [165, 114, 103, 0, 121, 206, 247, 81], // distribute_creator_fees
    [255, 203, 19, 79, 244, 68, 8, 159],  // distribute_creator_fees_v2
    [98, 54, 145, 97, 2, 70, 173, 43],    // distribute_fee_to_holders
    [234, 102, 194, 203, 150, 72, 62, 229], // extend_account
    [117, 225, 127, 202, 134, 95, 68, 35], // get_minimum_distributable_fee
    [94, 6, 202, 115, 255, 96, 232, 183], // init_user_volume_accumulator
    [175, 175, 109, 31, 13, 152, 155, 237], // initialize
    [239, 73, 245, 173, 209, 177, 84, 66], // initialize_quote_control
    [155, 234, 231, 146, 236, 158, 162, 30], // migrate
    [87, 124, 52, 191, 52, 38, 214, 232], // migrate_bonding_curve_creator
    [187, 203, 18, 31, 206, 237, 254, 41], // migrate_v2
    [223, 7, 253, 26, 81, 165, 218, 166], // remove_quote_control_mint
    [177, 65, 223, 38, 88, 209, 158, 155], // remove_quote_mint
    [51, 230, 133, 164, 1, 127, 131, 173], // sell
    [93, 246, 130, 60, 231, 233, 64, 178], // sell_v2
    [254, 148, 255, 112, 207, 142, 170, 165], // set_creator
    [61, 169, 188, 191, 153, 149, 42, 97], // set_mayhem_virtual_params
    [138, 96, 174, 217, 48, 85, 197, 246], // set_metaplex_creator
    [27, 234, 178, 52, 147, 2, 187, 141], // set_params
    [62, 79, 161, 211, 165, 170, 214, 210], // set_quote_control_admin
    [111, 172, 162, 232, 114, 89, 213, 142], // set_reserved_fee_recipients
    [101, 135, 191, 104, 9, 88, 20, 96],  // set_virtual_quote_reserves
    [86, 31, 192, 87, 163, 87, 79, 238],  // sync_user_volume_accumulator
    [115, 103, 224, 255, 189, 89, 86, 195], // toggle_cashback_enabled
    [28, 255, 230, 240, 172, 107, 203, 171], // toggle_create_v2
    [1, 9, 111, 208, 100, 31, 255, 163],  // toggle_mayhem_mode
    [251, 224, 171, 146, 160, 26, 113, 233], // update_buyback_config
    [61, 175, 160, 249, 66, 66, 136, 175], // update_creator_fee_config
    [227, 181, 74, 196, 208, 21, 97, 213], // update_global_authority
    [225, 252, 66, 4, 199, 35, 236, 16],  // update_holder_reward_config
];
