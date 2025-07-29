use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::find_comment_category::FindTypeOfComment;
use crate::utils::LineStats;

/// Count lines in files under `path`, optionally filtering by extension and excluding dirs.
pub fn count_lines(
    path: &Path,
    ext_filter: Option<&str>,
    exclude_dirs: &[PathBuf],
    comment_style_json: Option<PathBuf>,
) -> HashMap<PathBuf, LineStats> {
    // Initialize style finder from JSON or defaults
    let mut style_finder = FindTypeOfComment::new();
    if let Some(json_path) = comment_style_json {
        style_finder.extract_from_json(json_path);
    } else {
        style_finder.default_set_style();
    }

    // Prepare exclude patterns: absolute paths and directory names
    let exclude_abs: Vec<PathBuf> = exclude_dirs
        .iter()
        .filter(|d| d.is_absolute())
        .cloned()
        .collect();
    let exclude_names: Vec<_> = exclude_dirs
        .iter()
        .filter_map(|d| d.file_name().map(|n| n.to_owned()))
        .collect();

    // 1) Gather all eligible files
    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let p = entry.path();
            // skip if inside any excluded absolute directory
            if exclude_abs.iter().any(|excl| p.starts_with(excl)) {
                return false;
            }
            // skip if any component matches excluded directory names
            if exclude_names.iter().any(|name| p.ancestors().any(|anc| anc.file_name() == Some(name))) {
                return false;
            }
            // skip non-files
            if !p.is_file() {
                return false;
            }
            // extension filtering or detection
            match p.extension().and_then(|s| s.to_str()) {
                Some(ext) => {
                    if let Some(req) = ext_filter {
                        ext == req
                    } else {
                        style_finder.get_comment_style(ext).is_some()
                    }
                }
                None => false,
            }
        })
        .map(|e| e.into_path())
        .collect();

    // 2) Process each file in parallel
    let stats_vec: Vec<(PathBuf, LineStats)> = files
        .par_iter()
        .filter_map(|file| {
            let ext = file.extension()?.to_str()?;
            let style = style_finder.get_comment_style(ext)?;
            let f = File::open(file).ok()?;
            let reader = BufReader::new(f);

            let mut stats = LineStats::default();
            let mut in_block = false;
            let mut block_end: Option<String> = None;

            for line in reader.lines().flatten() {
                stats.total += 1;
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    stats.blanks += 1;
                    continue;
                }
                if in_block {
                    stats.comments += 1;
                    if let Some(ref end) = block_end {
                        if trimmed.contains(end) {
                            in_block = false;
                            block_end = None;
                        }
                    }
                    continue;
                }
                // line comment
                if trimmed.starts_with(&style.line) {
                    stats.comments += 1;
                    continue;
                }
                // block comment start
                if let (Some(start), Some(end)) = (&style.multiline_start, &style.multiline_end) {
                    if trimmed.starts_with(start) {
                        stats.comments += 1;
                        if !trimmed.contains(end) || trimmed.find(start) == trimmed.rfind(end) {
                            in_block = true;
                            block_end = Some(end.clone());
                        }
                        continue;
                    }
                }
                // alternate block comment (e.g., Python)
                if let Some((ref alt_start, ref alt_end)) = style.alt_multiline {
                    if trimmed.starts_with(alt_start) {
                        stats.comments += 1;
                        if !trimmed.contains(alt_end)
                            || trimmed.find(alt_start) == trimmed.rfind(alt_end)
                        {
                            in_block = true;
                            block_end = Some(alt_end.clone());
                        }
                        continue;
                    }
                }
                // code line
                stats.code += 1;
            }
            Some((file.clone(), stats))
        })
        .collect();

    // 3) Turn the Vec back into a HashMap
    stats_vec.into_iter().collect()
}
