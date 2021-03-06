use std::process;
use std::process::{Child, ChildStdin, ChildStdout, Stdio};
use std::io::{Result, LineWriter, BufReader, Write, BufRead};

use game::*;
use match_runner::*;

// A collection of running bots (i.e. process handles)
pub struct BotRunner {
    // Maps player ids to their process handles
    processes: PlayerMap<process::Child>,
}

impl BotRunner {
    pub fn run(players: &PlayerMap<PlayerConfig>) -> Self {
        BotRunner {
            processes: players.iter().map(|(&id, config)| {
                let process = process::Command::new(&config.command)
                    .args(&config.args)
                    .arg(format!("{}", config.name.as_str()))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect(&format!(
                        "\n[DRIVER] Failed to execute process: {} {:?}\n",
                        config.command,
                        config.args
                    ));
                return (id, process);
            }).collect()
        }
    }

    pub fn player_handles<'p>(&'p mut self) -> PlayerMap<PlayerHandle<'p>> {
        self.processes.iter_mut().map(|(&player_id, process)| {
            (player_id, PlayerHandle::new(process))
        }).collect()
    }

    pub fn kill(mut self) {
        for handle in self.processes.values_mut() {
            handle.kill().expect("Couldn't kill bot");
        }
    }
}


pub struct PlayerHandle<'p> {
    writer: LineWriter<&'p mut ChildStdin>,
    reader: BufReader<&'p mut ChildStdout>,
}

impl<'p> PlayerHandle<'p> {
    pub fn new(process: &'p mut Child) -> Self {
        PlayerHandle {
            writer: LineWriter::new(process.stdin.as_mut().unwrap()),
            reader: BufReader::new(process.stdout.as_mut().unwrap()),
        }
    }

    pub fn send_msg(&mut self, msg: &str) -> Result<()> {
        write!(&mut self.writer, "{}\n", msg)
    }

    pub fn recv_msg(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        return Ok(buf);
    }
}
