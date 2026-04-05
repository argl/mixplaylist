use flate2::read::GzDecoder;
use quick_xml::{Reader, events::Event};
use std::{env, fs::File, io::BufReader};

struct Clip {
    name: String,
    time_beats: f64,
}

struct Project {
    tempo_bpm: f64,
    clips: Vec<Clip>,
}

fn parse_project(path: &str) -> Result<Project, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let gz = GzDecoder::new(BufReader::new(file));
    let mut reader = Reader::from_reader(BufReader::new(gz));
    reader.config_mut().trim_text(true);

    let mut clips: Vec<Clip> = Vec::new();
    let mut buf = Vec::new();

    let mut depth: usize = 0;
    let mut clip_depth: Option<usize> = None;
    let mut clip_time: Option<f64> = None;

    // Tempo: we're inside <Tempo> when this is Some, and we want the <Manual> child
    let mut in_tempo = false;
    let mut tempo_depth: Option<usize> = None;
    let mut tempo_bpm: f64 = 120.0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => {
                let tag = e.local_name();
                let tag = tag.as_ref();

                if tag == b"Tempo" && tempo_depth.is_none() {
                    in_tempo = true;
                    tempo_depth = Some(depth);
                }

                if clip_depth.is_none() && (tag == b"AudioClip" || tag == b"MidiClip") {
                    let time = e
                        .attributes()
                        .flatten()
                        .find(|a| a.key.local_name().as_ref() == b"Time")
                        .and_then(|a| {
                            std::str::from_utf8(&a.value)
                                .ok()
                                .and_then(|v| v.parse().ok())
                        });
                    if let Some(t) = time {
                        clip_time = Some(t);
                        clip_depth = Some(depth);
                    }
                }

                depth += 1;
            }

            Event::Empty(ref e) => {
                let tag = e.local_name();
                let tag = tag.as_ref();

                if tag == b"Manual"
                    && in_tempo
                    && let Some(td) = tempo_depth
                    && depth == td + 1
                    && let Some(bpm) = e
                        .attributes()
                        .flatten()
                        .find(|a| a.key.local_name().as_ref() == b"Value")
                        .and_then(|a| {
                            std::str::from_utf8(&a.value)
                                .ok()
                                .and_then(|v| v.parse().ok())
                        })
                {
                    tempo_bpm = bpm;
                    in_tempo = false;
                    tempo_depth = None;
                }

                if tag == b"Name"
                    && let (Some(t), Some(cd)) = (clip_time, clip_depth)
                    && depth == cd + 1
                {
                    let name = e
                        .attributes()
                        .flatten()
                        .find(|a| a.key.local_name().as_ref() == b"Value")
                        .map(|a| String::from_utf8_lossy(&a.value).into_owned());
                    if let Some(n) = name {
                        clips.push(Clip {
                            name: n,
                            time_beats: t,
                        });
                    }
                    clip_time = None;
                    clip_depth = None;
                }
            }

            Event::End(_) => {
                depth -= 1;
                if clip_depth == Some(depth) {
                    clip_time = None;
                    clip_depth = None;
                }
                if tempo_depth == Some(depth) {
                    in_tempo = false;
                    tempo_depth = None;
                }
            }

            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    clips.sort_by(|a, b| a.time_beats.partial_cmp(&b.time_beats).unwrap());
    Ok(Project { tempo_bpm, clips })
}

fn beats_to_mmss(beats: f64, bpm: f64) -> String {
    let total_seconds = (beats / bpm * 60.0).round() as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.als>", args[0]);
        std::process::exit(1);
    }

    match parse_project(&args[1]) {
        Ok(project) => {
            for clip in &project.clips {
                println!(
                    "[{}] {}",
                    beats_to_mmss(clip.time_beats, project.tempo_bpm),
                    clip.name
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
