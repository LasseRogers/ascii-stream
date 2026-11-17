use std::error::Error;
use std::process::Command;

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
