#[macro_use]
extern crate cpp;

#[link(name="JSBSim")]
extern {}

cpp! {{
    #include <FGFDMExec.h>
    #include <models/FGAircraft.h>
}}
fn main() {
    unsafe {
        cpp!([] {
            JSBSim::FGFDMExec fdmex = JSBSim::FGFDMExec();
            fdmex.SetSystemsPath("systems/");
            fdmex.SetAircraftPath("aircraft/");
            fdmex.SetEnginePath("engine/");
            fdmex.LoadScript("simulation/run.xml");
            int loop = 1;
            while (loop) {
                
                loop = fdmex.Run();
                cout << "----------DATA----------" << endl;
                cout <<fdmex.GetAircraft()->GetForces().Dump(",") << endl;
                cout <<fdmex.GetAircraft()->GetXYZep().Dump(",") << endl;
                cout <<fdmex.GetAircraft()->GetXYZrp().Dump(",") << endl;
                cout <<fdmex.GetAircraft()->GetXYZvrp().Dump(",") << endl;
                cout << "---------ENDDATA--------" << endl;
            }
        });
    }
}
