use crate::imports::*;


////motion
//appeal-hi
// added wind-box
unsafe extern "C" fn appeal_hi_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 100, 22, 0, 14.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(-15.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn appeal_hi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 3, 110, 2, 1, 0, 15, 30, 30, 50);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////motion
    //up-taunt
    agent.game_acmd("game_appealhil", appeal_hi_game, Priority::High);
    agent.game_acmd("game_appealhir", appeal_hi_game, Priority::High);
    agent.expression_acmd("expression_appealhil", appeal_hi_exp, Priority::High);
    agent.expression_acmd("expression_appealhir", appeal_hi_exp, Priority::High);
}