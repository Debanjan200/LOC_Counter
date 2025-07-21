use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::find_comment_category::FindTypeOfComment;
use crate::utils::LineStats;

pub fn count_lines(
    path: &Path,
    ext_filter: Option<&str>,
    exclude_dirs: &[PathBuf],
) -> HashMap<PathBuf, LineStats> {
    let mut results = HashMap::new();
    let style_finder = FindTypeOfComment::new();

    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();

            for excl in exclude_dirs {
                if path.starts_with(excl) || path.components().any(|c| excl.ends_with(c)) {
                    return false;
                }
            }

            if !path.is_file() {
                return false;
            }

            let ext_opt = path.extension().and_then(|s| s.to_str());

            match ext_filter {
                Some(requested_ext) => ext_opt.map_or(false, |ext| ext == requested_ext),
                None => ext_opt.map_or(false, |ext| style_finder.get_comment_style(ext).is_some()),
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    for file in files {
        let ext = file.extension().and_then(|s| s.to_str());

        if let Some(ext) = ext {
            if let Some(style) = style_finder.get_comment_style(ext) {
                if let Ok(f) = File::open(&file) {
                    let reader = BufReader::new(f);
                    let mut stats = LineStats::default();

                    let mut in_multiline = false;
                    let mut multiline_delim: Option<&str> = None;

                    for line in reader.lines().flatten() {
                        stats.total += 1;
                        let trimmed = line.trim();

                        if trimmed.is_empty() {
                            stats.blanks += 1;
                            continue;
                        }

                        if in_multiline {
                            stats.comments += 1;
                            if let Some(end) = multiline_delim {
                                if trimmed.contains(end) {
                                    in_multiline = false;
                                    multiline_delim = None;
                                }
                            }
                            continue;
                        }

                        if trimmed.starts_with(style.line) {
                            stats.comments += 1;
                            continue;
                        }

                        // Primary multiline
                        if let Some(start) = style.multiline_start {
                            if trimmed.starts_with(start) {
                                stats.comments += 1;
                                if let Some(end) = style.multiline_end {
                                    if !(trimmed.contains(end) && trimmed.find(start) != trimmed.rfind(end)) {
                                        in_multiline = true;
                                        multiline_delim = Some(end);
                                    }
                                }
                                continue;
                            }
                        }

                        // Optional alt multiline (Python """ or ''')
                        if let Some((alt_start, alt_end)) = style.alt_multiline {
                            if trimmed.starts_with(alt_start) {
                                stats.comments += 1;
                                if !(trimmed.contains(alt_end) && trimmed.find(alt_start) != trimmed.rfind(alt_end)) {
                                    in_multiline = true;
                                    multiline_delim = Some(alt_end);
                                }
                                continue;
                            }
                        }

                        stats.code += 1;
                    }

                    results.insert(file, stats);
                }
            }
        }
    }

    results
}
