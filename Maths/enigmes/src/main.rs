
mod give_change;

use crate::give_change::*;


fn main() {
    // Money change algo.
    println!("# Give change.");
    let return_money: u32 = 127;
    println!("To return money = {}", return_money);

    println!("European system = {:?}", SYSTEM_UE);
    let is_ue_canonic: bool = canonic_system(SYSTEM_UE);
    println!("- Is EU System canonic: {}", is_ue_canonic);
    let returns_ue: [u32; give_change::SYSTEM_UE_LEN] = give_change::greedy(return_money, give_change::SYSTEM_UE);
    let returns_ue_quantity: u32 = array_sum(returns_ue);
	println!("- {:?}, q={}", returns_ue, returns_ue_quantity);

    println!("Plutonic system = {:?}", SYSTEM_PLT);
    let is_plt_canonic: bool = canonic_system(SYSTEM_PLT);
    println!("- Is PLT System canonic: {}", is_plt_canonic);
    let returns_plt: [u32; give_change::SYSTEM_PLT_LEN] = give_change::greedy(return_money, give_change::SYSTEM_PLT);
    let returns_plt_quantity: u32 = array_sum(returns_plt);
	println!("- {:?}, q={}", returns_plt, returns_plt_quantity);
}

