use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use tokio::sync::mpsc;

pub struct PtyHandle {
    pub reader_rx: mpsc::Receiver<Vec<u8>>,
    pub writer_tx: mpsc::Sender<Vec<u8>>,
}

pub fn spawn_pty() -> Result<PtyHandle, Box<dyn std::error::Error + Send + Sync>> {
    let pty_system = native_pty_system();

    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Get the user's default shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    let mut cmd = CommandBuilder::new(&shell);
    cmd.env("TERM", "xterm-256color");

    let _child = pair.slave.spawn_command(cmd)?;

    // Get reader and writer before moving pair
    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;

    let (reader_tx, reader_rx) = mpsc::channel::<Vec<u8>>(100);
    let (writer_tx, mut writer_rx) = mpsc::channel::<Vec<u8>>(100);

    // Reader thread - reads from PTY and sends to channel
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if reader_tx.blocking_send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Writer thread - receives from channel and writes to PTY
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            while let Some(data) = writer_rx.recv().await {
                if writer.write_all(&data).is_err() {
                    break;
                }
                let _ = writer.flush();
            }
        });
    });

    Ok(PtyHandle {
        reader_rx,
        writer_tx,
    })
}
