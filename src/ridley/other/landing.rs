use crate::imports::*;


////motion
//landing-heavy
// added screen shake
unsafe extern "C" fn landing_heavy_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    agent.expression_acmd("expression_landingheavy", landing_heavy_exp, Priority::High);
}