use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::{
    vars::*,
    *
};


////status
//air-dodge
// skewer input
unsafe extern "C" fn escape_air_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        VarModule::on_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER);
        agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return true.into()
    }
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_ESCAPE_AIR)(agent)
}
unsafe extern "C" fn escape_air_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) 
    && agent.global_table[global_table::STATUS_FRAME].get_i32() <= param::RIDLEY_INT_SKEWER_INPUT_FRAME 
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        VarModule::on_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER);
        agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    }
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //air-dodge
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_status_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_status_exec);
}