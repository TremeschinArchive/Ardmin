// | (c) Tremeschin, AGPLv3-only License | Ardmin Project | //
#![allow(non_snake_case)]
use Protostar::*;

// -----------------------------------------------------------------------------------------------|

#[derive(Parser, Debug)]
#[command(author=Protostar::Constants::AUTHOR, about=Protostar::Constants::About::ARDMIN, version)]
struct Args {
    #[arg(short, long, help="(Global      ) Path to a Folder of Ardour Sessions")]
    path: String,
    #[arg(       long, help="(Global      ) Move existing exports files to other path", default_value_t=str!(""))]
    exports: String,
    #[arg(short, long, help="(Global      ) Apply all optimizations")]
    all: bool,
    #[arg(short, long, help="(Optimization) Remove unused Source files (MIDI, WAV)")]
    unused: bool,
    #[arg(short, long, help="(Optimization) Remove old plugin states (5% chance of breaking per-plugin??)")]
    states: bool,
    #[arg(short, long, help="(Optimization) Remove backup (.bak) of sessions")]
    backup: bool,
    #[arg(       long, help="(Optimization) Remove history (.history) of sessions")]
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

        // List of regex to apply searching for sources
        let mut regrets: HashMap<&str, Regex> = HashMap::new();
        let mut sources: Vec<String> = vec!();

        // Regex for different sources
        for extension in vec!(".mid", ".wav") {
            let regex = Regex::new(format!("name=\"(.*?){}\"", extension).as_str());
            regrets.insert(extension, regex.unwrap());
        }

        // Optimization: Remove .history or .backup or unused MIDI / WAV files
        for file in Protostar::betterGlob(session.join("*")) {
            if let Some(ext) = file.extension() {
                if (args.history || args.all) && (ext == "history") {Protostar::remove(file.clone())}
                if (args.backup  || args.all) && (ext == "bak"    ) {Protostar::remove(file.clone())}

                // Search for used MIDI sources
                if (args.unused  || args.all) && (ext == "ardour" ) {

                    // Iterate on .ardour session file lines
                    for line in BufReader::new(File::open(file).unwrap()).lines().map(Result::unwrap) {

                        // Code optimization: Will not find any more sources
                        if line.contains("</Sources>") {break}

                        // Match any regex for sources
                        for (extension, regex) in regrets.iter() {
                            for capture in regex.captures_iter(&line) {
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

        // Optimization: Remove old plugin states
        if args.states || args.all {
            for pluginFolder in Protostar::betterGlob(session.join("plugins").join("*")) {

                // Converts session/plugins/stateXYZ to <XYZ: i64>
                let getState = |x: &PathBuf| -> i64 {
                    x.file_name().unwrap().to_str().unwrap().replace("state", "").parse::<i64>().unwrap()
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

        // Optimization: Move exports to other folder
        if args.exports != str!("") {
            for export in Protostar::betterGlob(session.join("export").join("*")) {
                Protostar::moveFile(&export, &PathBuf::from(args.exports.clone()).join(export.file_name().unwrap()));
            }
        }
    }
}
