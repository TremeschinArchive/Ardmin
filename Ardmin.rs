// | (c) Tremeschin, MIT License | Ardmin Project | //
#![allow(non_snake_case)]
extern crate Protostar;
use Protostar::*;

// -----------------------------------------------------------------------------------------------|

const ABOUT: &str = "
Ardmin, an Ardour Session Minimizer.

(c) 2023 Tremeschin, MIT License.";

// CLI Arguments
#[derive(Parser, Debug)]
#[command(author="Tremeschin", version, about=ABOUT)]
struct Args {
    #[arg(short, long, help="· (Global      ) Path to a <Session Folder> or <Folder of Sessions> (depth=1)")]
    path: String,

    // Exports
    #[arg(long, default_value_t=str!(""), help="· (Global      ) Move existing exports files to other path")]
    exports: String,

    // Apply all optimizations
    #[arg(short, long, help="· (Global      ) Apply all optimizations")]
    all: bool,

    // Optimizations
    #[arg(short, long, help="· (Optimization) Remove unused MIDI files")]
    midi: bool,

    #[arg(short, long, help="· (Optimization) Remove old plugin states")]
    states: bool,

    #[arg(short, long, help="· (Optimization) Remove backup (.bak) of sessions")]
    backup: bool,

    #[arg(long, help="· (Optimization) Remove history (.history) of sessions")]
    history: bool,

    #[arg(short, long, help="· (Optimization) Remove analysis, dead, peaks folders")]
    residuals: bool,
}

// -----------------------------------------------------------------------------------------------|


// fn treatSession(sessionPath: String) {

// }

fn main() {
    Protostar::setupLog();
    let args = Args::parse();
    dbg!(&args);

    for _sessionPath in glob(format!("{}/*", args.path).as_str()).expect("Path Exists") {
        // infof!("Dealing with session [{_sessionPath}]");
    }

}
