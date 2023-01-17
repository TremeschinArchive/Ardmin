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
    #[arg(short, long, help="(Global      ) Path to a Folder of Sessions")]
    path: String,

    // Exports
    #[arg(       long, help="(Global      ) Move existing exports files to other path", default_value_t=str!(""))]
    exports: String,

    // Apply all optimizations
    #[arg(short, long, help="(Global      ) Apply all optimizations")]
    all: bool,

    // Optimizations
    #[arg(short, long, help="(Optimization) Remove unused Source files (MIDI, WAV)")]
    unused: bool,

    #[arg(short, long, help="(Optimization) Remove old plugin states (5% chance of breaking session??)")]
    states: bool,

    #[arg(short, long, help="(Optimization) Remove backup (.bak) of sessions")]
    backup: bool,

    #[arg(long, help="(Optimization) Remove history (.history) of sessions")]
    history: bool,

    #[arg(short, long, help="(Optimization) Remove analysis, dead, peaks folders")]
    residuals: bool,
}

// -----------------------------------------------------------------------------------------------|

fn main() {
    Protostar::setupLog();
    let args = Args::parse();

    // Main "for each session"
    for session in Protostar::betterGlob(PathBuf::from(args.path).join("*")) {
        if session.is_file() {continue;}
        info!(":: Optimizing session [{}]", session.display());

        // Optimization: Remove analysis, dead, peaks
        if args.residuals || args.all {
            for folder in vec!("analysis", "dead", "peaks") {
                Protostar::remove(session.join(&folder));
            }
        }

        // Optimization: Remove .history or .backup or unused MIDI / WAV files
        let mut sources: Vec<String> = vec!();
        let sourcesExtensions = vec!(".mid", ".wav");

        for file in Protostar::betterGlob(session.join("*")) {
            if let Some(ext) = file.extension() {
                if (args.history || args.all) && (ext == "history") {Protostar::remove(file.clone())}
                if (args.backup  || args.all) && (ext == "bak"    ) {Protostar::remove(file.clone())}

                // Search for used MIDI sources
                if (args.unused  || args.all) && (ext == "ardour" ) {

                    // Iterate on .ardour session file lines
                    for line in BufReader::new(File::open(file).unwrap()).lines().map(Result::unwrap) {

                        // Optimization: Only match lines that contains sources extensions
                        if ! &sourcesExtensions.iter().any(|substring| line.contains(substring)) {continue;}

                        // Match for sources on Ardour session
                        for extension in &sourcesExtensions {
                            for capture in Regex::new(format!("name=\"(.*?){}\"", extension).as_str()).unwrap().captures_iter(&line) {
                                sources.push(format!("{}{}", &capture[1], extension));
                            }
                        }
                    }
                }
            }
        }

        // Recurse on interchange (sources) of session, remove files not listed in sources in any of .ardour sessions
        if args.unused || args.all {
            for source in Protostar::betterGlob(session.join("interchange").join("**").join("*")) {
                if source.is_file() && !sources.contains(&source.file_name().unwrap().to_str().unwrap().to_string()) {
                    Protostar::remove(source)
                }
            }
        }

        if args.states || args.all {
            // Optimization: Remove old plugin states
            for pluginFolder in Protostar::betterGlob(session.join("plugins").join("*")) {

                // Converts session/plugins/stateXYZ to i64 XYZ
                let getState = |x: &PathBuf| {
                    x.file_name().unwrap().to_str().unwrap().replace("state", "").parse().unwrap()
                };

                // The max allowed state
                let maxState: i64 = Protostar::betterGlob(pluginFolder.join("*")).iter().map(|x| getState(x)).max().unwrap();

                // Delete folders that lag behind the max state
                for stateFolder in Protostar::betterGlob(pluginFolder.join("*")) {
                    if getState(&stateFolder) < maxState {
                        Protostar::remove(stateFolder);
                    }
                }
            }
        }
    }
}
