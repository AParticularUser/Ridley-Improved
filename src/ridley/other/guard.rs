use crate::imports::*;
use crate::ridley::consts::vars::*;


////status
//shield
// skewer input
unsafe extern "C" fn guard_on_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
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
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_GUARD_ON)(agent)
}
unsafe extern "C" fn guard_on_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
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
        0.into()
    }else {
        smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_GUARD_ON)(agent)
    }
}
unsafe extern "C" fn guard_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
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
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_GUARD)(agent)
}
unsafe extern "C" fn guard_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
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
        0.into()
    }else {
        smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_GUARD)(agent)
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //shield
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_status_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_status_exec);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD, guard_status_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD, guard_status_exec);
}