use crate::color::ShellColor;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;
use terminal_size;

pub struct Download {
    url: String,
    des: Option<String>,
    passed_dir: Option<String>,
    overwrite: bool,
    continue_download: bool,
    echo: bool,
    quiet: bool,
    batch: bool,
    done_icon: String,
    left_icon: String,
    current_icon: String,
    border_left: char,
    border_right: char,
    color_done: String,
    color_left: String,
    color_current: String,
    color_engine: ShellColor,
    f_size: Option<u64>,
    destination: String,
}

impl Download {
    pub fn new(
        url: String,
        des: Option<String>,
        overwrite: bool,
        continue_download: bool,
        echo: bool,
        quiet: bool,
        batch: bool,
        done_icon: String,
        left_icon: String,
        current_icon: String,
        icon_border: String,
        color_done: String,
        color_left: String,
        color_current: String,
        color_engine: ShellColor,
    ) -> Self {
        let (border_left, border_right) = Self::extract_border_icon(&icon_border);

        let done_icon = if done_icon.len() < 2 {
            done_icon
        } else {
            "▓".to_string()
        };
        let left_icon = if left_icon.len() < 2 {
            left_icon
        } else {
            "░".to_string()
        };
        let current_icon = if current_icon.len() < 2 {
            current_icon
        } else {
            "▓".to_string()
        };

        Self {
            url,
            des,
            passed_dir: None,
            overwrite,
            continue_download,
            echo,
            quiet,
            batch,
            done_icon,
            left_icon,
            current_icon,
            border_left,
            border_right,
            color_done,
            color_left,
            color_current,
            color_engine,
            f_size: None,
            destination: String::new(),
        }
    }

    fn extract_border_icon(passed_icon: &str) -> (char, char) {
        let mut chars = passed_icon.chars();
        match (chars.next(), chars.next()) {
            (Some(l), Some(r)) => (l, r),
            (Some(l), None) => (l, l),
            _ => ('|', '|'),
        }
    }

    fn get_name(&self) -> String {
        let temp_url = &self.url;
        if let Some(last) = temp_url.rsplit('/').next() {
            last.split('?').next().unwrap_or("temp").to_string()
        } else {
            "temp".to_string()
        }
    }

    fn is_valid_url(&self) -> bool {
        let re = Regex::new(r"^https?://|^file://").unwrap();
        re.is_match(&self.url)
    }

    fn parse_url(&self) -> Vec<String> {
        if self.is_valid_url() {
            return vec![self.url.clone()];
        }

        if !self.batch {
            eprintln!("{}: not a valid URL. Pass -b if it is a file containing various URL's and you want bulk download.", self.url);
            std::process::exit(0);
        }

        let rel_path = shellexpand::tilde(&self.url).to_string();

        if !Path::new(&rel_path).is_file() {
            eprintln!("{}: not a valid name or is a directory", rel_path);
            std::process::exit(-1);
        }

        let content = fs::read_to_string(&rel_path).unwrap_or_default();
        content.lines().map(|s| s.to_string()).collect()
    }

    fn parse_destination(&mut self) {
        if let Some(ref des) = self.des {
            if Path::new(des).is_dir() {
                self.passed_dir = Some(des.clone());
                self.destination = Path::new(des)
                    .join(self.get_name())
                    .to_string_lossy()
                    .to_string();
            } else {
                self.destination = des.clone();
            }
        } else {
            self.destination = self.get_name();
        }

        if Path::new(&self.destination).exists() {
            self.parse_exists();
        }
    }

    fn parse_exists(&mut self) {
        if self.overwrite {
            return;
        }

        if self.continue_download {
            if let Ok(metadata) = fs::metadata(&self.destination) {
                let cur_size = metadata.len();

                if let Ok(response) = reqwest::blocking::get(&self.url) {
                    if let Some(size) = response.content_length() {
                        if cur_size < size {
                            self.build_headers(cur_size);
                            return;
                        }
                    } else {
                        eprintln!("WARNING: Could not perform sanity check on partial download.");
                        self.build_headers(cur_size);
                        return;
                    }
                }
            }
        }

        eprintln!("ERROR: File exists. See 'dw --help' for solutions.");
        std::process::exit(-1);
    }

    fn build_headers(&self, resume: u64) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Range", format!("bytes={}-", resume).parse().unwrap());
        headers
    }

    fn format_size(&self, size: u64) -> (f64, String) {
        let units = ["bytes", "KB", "MB", "GB"];
        let mut formatted_size = size as f64;
        let mut unit_idx = 0;

        while formatted_size > 1024.0 && unit_idx < units.len() - 1 {
            formatted_size /= 1024.0;
            unit_idx += 1;
        }

        (formatted_size, units[unit_idx].to_string())
    }

    fn format_time(&self, time_left: f64) -> (f64, String) {
        let units = ["s", "m", "h", "d"];
        let mut time = time_left;
        let mut unit_idx = 0;

        while time > 60.0 && unit_idx < units.len() - 1 {
            time /= 60.0;
            unit_idx += 1;
        }

        (time, units[unit_idx].to_string())
    }

    fn format_speed(&self, speed: f64) -> (f64, String) {
        let units = ["Kb/s", "Mb/s", "Gb/s"];
        let mut sp = speed;
        let mut unit_idx = 0;

        while sp > 1000.0 && unit_idx < units.len() - 1 {
            sp /= 1000.0;
            unit_idx += 1;
        }

        (sp, units[unit_idx].to_string())
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    pub fn download(&mut self) -> bool {
        let urls = self.parse_url();

        for url in urls {
            self.url = url;
            if !self.download_single() {
                return false;
            }
            if let Some(ref dir) = self.passed_dir {
                self.des = Some(dir.clone());
            }
        }
        true
    }

    fn download_single(&mut self) -> bool {
        self.parse_destination();

        let client = Client::new();

        let mut headers = HeaderMap::new();
        if self.continue_download && Path::new(&self.destination).exists() {
            if let Ok(metadata) = fs::metadata(&self.destination) {
                let cur_size = metadata.len();
                eprintln!("Trying to resume download at: {} bytes", cur_size);
                headers.insert("Range", format!("bytes={}-", cur_size).parse().unwrap());
            }
        }

        let mut response = client.get(&self.url).headers(headers).send();

        let mut response = match response {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                return false;
            }
        };

        self.f_size = response.content_length();

        let file_size = self.f_size;

        if !self.quiet {
            if let Some(size) = file_size {
                let (formatted, unit) = self.format_size(size);
                eprintln!("Size: {} {}", formatted.round(), unit);
            }
        }

        let file_exists = Path::new(&self.destination).exists();
        let msg = if file_exists && self.overwrite {
            format!("Overwriting: {}", self.destination)
        } else {
            format!("Saving as: {}", self.destination)
        };

        if self.quiet {
            eprint!("{}...", msg);
        } else {
            eprintln!("{}", msg);
        }

        let mut file = match OpenOptions::new()
            .create(true)
            .append(self.continue_download && file_exists)
            .open(&self.destination)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                return false;
            }
        };

        let beg_time = Instant::now();
        let mut downloaded: u64 = 0;
        let block_size = 8192;
        let mut buffer = vec![0u8; block_size];

        loop {
            let bytes_read = match response.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("\nERROR: {}", e);
                    return false;
                }
            };

            if let Err(e) = file.write_all(&buffer[..bytes_read]) {
                eprintln!("\nERROR: {}", e);
                return false;
            }

            downloaded += bytes_read as u64;

            if !self.quiet {
                let elapsed = beg_time.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let speed = (downloaded as f64 / 1024.0) / elapsed;
                    let (speed_val, speed_unit) = self.format_speed(speed);

                    let term_width = terminal_size::terminal_size()
                        .map(|(w, _)| w.0 as usize)
                        .unwrap_or(80);

                    let status = if let Some(size) = file_size {
                        let percent = (downloaded as f64 / size as f64) * 100.0;
                        let remaining = size - downloaded;
                        let time_left = if speed > 0.0 {
                            (remaining as f64 / 1024.0) / speed
                        } else {
                            0.0
                        };
                        let (time_val, time_unit) = self.format_time(time_left);

                        let bar_len = 40.min(term_width.saturating_sub(50));
                        let done = ((percent / 100.0) * bar_len as f64) as usize;
                        let left = bar_len - done;

                        let done_str = self
                            .color_engine
                            .wrap_in_color(&self.done_icon.repeat(done), &self.color_done);
                        let left_str = self
                            .color_engine
                            .wrap_in_color(&self.left_icon.repeat(left), &self.color_left);

                        let percent_str = format!("{:>4.0}%", percent);
                        format!(
                            "{:>7.1} {} | {:>3.0} {} || ETA: {:>4.1} {} |{}{}{}| {}",
                            downloaded as f64 / 1024.0,
                            "KB",
                            speed_val.round(),
                            speed_unit,
                            time_val.round(),
                            time_unit,
                            self.border_left,
                            done_str,
                            left_str,
                            self.border_right
                        ) + &percent_str
                    } else {
                        format!(
                            "{:>7.1} {} | {:>3.0} {} ||",
                            downloaded as f64 / 1024.0,
                            "KB",
                            speed_val.round(),
                            speed_unit
                        )
                    };

                    print!("\r{}", status);
                    std::io::stdout().flush().ok();
                }
            }
        }

        if !self.quiet {
            println!();
        } else {
            eprintln!("...success");
        }

        true
    }
}

struct OpenOptions {
    file: Option<std::fs::File>,
    create: bool,
    append: bool,
    read: bool,
    write: bool,
}

impl OpenOptions {
    fn new() -> Self {
        Self {
            file: None,
            create: false,
            append: false,
            read: false,
            write: true,
        }
    }

    fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }

    fn append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }

    fn open(self, path: &str) -> std::io::Result<std::fs::File> {
        use std::fs::OpenOptions as StdOpenOptions;
        let mut opts = StdOpenOptions::new();
        opts.write(true)
            .create(self.create)
            .append(self.append)
            .open(path)
    }
}
