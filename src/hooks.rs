#![allow(non_snake_case)]
use crate::imports::*;


//special thanks to Wuboy/Wubor Patch for porting HDR's VarModule
#[skyline::hook(offset = 0x3af300)]
pub unsafe fn battleobjectmoduleaccessor__initialize_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: *const u64) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    if object_id != 0x50000000 {
        CustomVarManager::reset_var_module(module_accessor, false);
    }
}
#[skyline::hook(offset = 0x3afa10)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    VarModule::start(module_accessor);
}
#[skyline::hook(offset = 0x3afe00)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    CustomVarManager::reset_var_module(module_accessor, true);
    original!()(module_accessor, param_1)
}
#[skyline::hook(offset = 0x3af720)]
pub unsafe fn battleobjectmoduleaccessor__finalize_modules(module_accessor: *mut BattleObjectModuleAccessor) {
    CustomVarManager::remove_var_module(module_accessor);
    original!()(module_accessor)
}


pub fn install() {
    skyline::install_hooks!(
        battleobjectmoduleaccessor__initialize_modules,
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules,
        battleobjectmoduleaccessor__finalize_modules
    );
}