use core::App;
use econ::EconPlugin;
use gov::GovPlugin;
use models::EconState;
use models::GovState;

mod util;
use util::fmt_currency;

fn main() {
    let mut app = App::new();

    app.add_plugin(EconPlugin);
    app.add_plugin(GovPlugin);

    app.run(12);

    app.summarize_state();

    use std::any::type_name;
    let econ_any = app.state.as_raw_map().get("econ");
    if let Some(econ_any) = econ_any {
        println!("[debug] econ type: {:?}", econ_any.type_id());
        // Try to downcast and print type name
        if econ_any.downcast_ref::<EconState>().is_none() {
            println!(
                "[debug] econ is not EconState, but type_id: {:?}",
                econ_any.type_id()
            );
        }
    } else {
        println!("[debug] econ key not found in current map");
    }
    let econ = app.state.get::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));

    let gov_any = app.state.as_raw_map().get("gov");
    if let Some(gov_any) = gov_any {
        println!("[debug] gov type: {:?}", gov_any.type_id());
        if gov_any.downcast_ref::<GovState>().is_none() {
            println!(
                "[debug] gov is not GovState, but type_id: {:?}",
                gov_any.type_id()
            );
        }
    } else {
        println!("[debug] gov key not found in current map");
    }
    let gov = app.state.get::<GovState>("gov");
    println!("Final stability: {:.2}%", gov.stability * 100.0);
}
