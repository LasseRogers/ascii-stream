use std::error::Error;
use std::io::{self, Read};
use std::process::{Child, ChildStdout, Command, Stdio};

#[derive(Debug)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub fps_num: u16,
    pub fps_den: u16,
}

pub fn probe(path: &str) -> Result<Info, Box<dyn Error>> {
    let out = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "v:0"])
        .args(["-show_entries", "stream=width,height,r_frame_rate"])
        .args(["-of", "csv=p=0", path])
        .output()?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into());
    }
    let text = String::from_utf8(out.stdout)?;
    let mut fields = text.trim().split([',', '/']);
    let mut next = || fields.next().ok_or("unexpected ffprobe output");
    Ok(Info {
        width: next()?.parse()?,
        height: next()?.parse()?,
        fps_num: next()?.parse()?,
        fps_den: next()?.parse()?,
    })
}

pub struct Frames {
    child: Child,
    stdout: ChildStdout,
}

pub fn frames(path: &str, cols: u16, rows: u16) -> io::Result<Frames> {
    let mut child = Command::new("ffmpeg")
        .args(["-v", "fatal", "-i", path])
        .args(["-vf", &format!("scale={cols}:{rows}")])
        .args(["-f", "rawvideo", "-pix_fmt", "rgb24", "-"])
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.take().expect("stdout is piped");
    Ok(Frames { child, stdout })
}

impl Frames {
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<bool> {
        match self.stdout.read_exact(buf) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(false),
            Err(e) => Err(e),
        }
    }
}

impl Drop for Frames {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
