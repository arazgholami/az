use crate::{BLUE, CYAN, COMMENT, GREEN, MAGENTA, ORANGE, PURPLE, RED, Segment, CompletionItem};
use super::{after_nonspace_is, comp, find_literals, find_words, scan_ranges, string_ranges, CompletionContext};

pub(crate) fn segments(line: &str) -> Vec<Segment> {
    let mut s = Vec::new();
    for (a, b) in string_ranges(line) { s.push(Segment { start: a, end: b, color: GREEN }); }
    if let Some(pos) = line.find("//") { s.push(Segment { start: pos, end: line.len(), color: COMMENT }); }
    if let Some(pos) = line.find('#') { if !line[..pos].contains("<?") { s.push(Segment { start: pos, end: line.len(), color: COMMENT }); } }
    for (a, b) in find_words(line) {
        let w = &line[a..b];
        let color = if keywords().contains(&w) { PURPLE }
            else if w.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false) { CYAN }
            else if after_nonspace_is(line, b, '(') { BLUE }
            else { continue; };
        s.push(Segment { start: a, end: b, color });
    }
    for (a, b) in scan_ranges(line, |c| c.is_ascii_digit()) { s.push(Segment { start: a, end: b, color: ORANGE }); }
    for (a, b) in variable_ranges(line) { s.push(Segment { start: a, end: b, color: RED }); }
    for (a, b) in find_literals(line, &["<?php", "<?=", "?>"]) { s.push(Segment { start: a, end: b, color: MAGENTA }); }
    s
}

pub(crate) fn completion_context(before: &str, explicit: bool) -> Option<(String, String, usize)> {
    if let Some((prefix, start)) = variable_suffix(before) { return Some(("php:var".to_string(), prefix, start)); }
    if before.ends_with("->") || before.ends_with("::") { return Some(("php:member".to_string(), String::new(), before.len())); }
    if let Some((prefix, start)) = super::word_suffix(before) {
        if explicit || prefix.len() >= 2 { return Some(("php:word".to_string(), prefix, start)); }
    }
    None
}

pub(crate) fn completion_items(kind: &str, ctx: CompletionContext<'_>) -> Vec<CompletionItem> {
    match kind {
        "php:var" => variables(ctx.lines, ctx.scan_limit).iter().map(|s| comp(s, s, "variable")).collect(),
        "php:member" => members().iter().map(|s| comp(s, s, "member")).collect(),
        "php:word" => {
            let mut all = Vec::new();
            for s in keywords() { all.push(comp(s, s, "keyword")); }
            for s in functions() { all.push(comp(s, s, "function")); }
            for s in constants() { all.push(comp(s, s, "constant")); }
            for s in document_symbols(ctx.lines, ctx.scan_limit) { all.push(comp(&s, &s, "symbol")); }
            all
        }
        _ => Vec::new(),
    }
}

pub(crate) fn variables(lines: &[String], limit: usize) -> Vec<String> {
    let mut vars = vec!["$this".to_string(), "$_GET".to_string(), "$_POST".to_string(), "$_REQUEST".to_string(), "$_SERVER".to_string(), "$_SESSION".to_string(), "$_COOKIE".to_string(), "$_FILES".to_string(), "$_ENV".to_string()];
    for line in lines.iter().take(limit) {
        for token in variable_ranges(line) { vars.push(line[token.0..token.1].to_string()); }
    }
    vars.sort();
    vars.dedup();
    vars
}

pub(crate) fn document_symbols(lines: &[String], limit: usize) -> Vec<String> {
    let mut out = Vec::new();
    for line in lines.iter().take(limit) { out.extend(symbols(line)); }
    out.sort();
    out.dedup();
    out
}

pub(crate) fn symbols(line: &str) -> Vec<String> {
    let trimmed = line.trim_start();
    let mut out = Vec::new();
    for marker in ["function ", "class ", "trait ", "interface ", "enum "] {
        if let Some(pos) = trimmed.find(marker) {
            let after = &trimmed[pos + marker.len()..];
            let name: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if !name.is_empty() { out.push(format!("{} {}", marker.trim(), name)); }
        }
    }
    out
}

pub(crate) fn variable_ranges(line: &str) -> Vec<(usize, usize)> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            if i > start + 1 { out.push((start, i)); }
        } else { i += 1; }
    }
    out
}

pub(crate) fn variable_suffix(before: &str) -> Option<(String, usize)> {
    let idx = before.rfind('$')?;
    if before[idx + 1..].chars().all(|c| c.is_alphanumeric() || c == '_') {
        Some((before[idx..].to_string(), idx))
    } else { None }
}

fn keywords() -> Vec<&'static str> { vec!["abstract","array","as","break","callable","case","catch","class","clone","const","continue","declare","default","do","echo","else","elseif","empty","enum","extends","final","finally","fn","for","foreach","function","global","goto","if","implements","include","include_once","instanceof","interface","isset","list","match","namespace","new","null","print","private","protected","public","readonly","require","require_once","return","static","switch","throw","trait","try","unset","use","while","yield","true","false","void","self","parent","int","string","float","bool","mixed","never","object"] }
fn functions() -> Vec<&'static str> { vec!["array_column","array_filter","array_key_exists","array_keys","array_map","array_merge","array_pop","array_push","array_reduce","array_search","array_shift","array_slice","array_unique","array_values","basename","chr","count","date","dirname","empty","explode","file_exists","file_get_contents","file_put_contents","filter_input","filter_var","fopen","fwrite","getenv","html_entity_decode","htmlspecialchars","implode","in_array","is_array","is_bool","is_dir","is_file","is_int","is_null","is_numeric","is_object","is_string","json_decode","json_encode","ltrim","mb_strlen","mb_strtolower","mb_strtoupper","mb_substr","method_exists","number_format","pathinfo","preg_match","preg_match_all","preg_replace","print_r","realpath","round","rtrim","sprintf","str_contains","str_ends_with","str_replace","str_starts_with","strlen","strpos","strtolower","strtoupper","substr","trim","ucfirst","var_dump"] }
fn constants() -> Vec<&'static str> { vec!["true","false","null","PHP_EOL","PHP_VERSION","DIRECTORY_SEPARATOR","PATH_SEPARATOR","STDIN","STDOUT","STDERR","__DIR__","__FILE__","__LINE__","__CLASS__","__METHOD__","__FUNCTION__","__NAMESPACE__"] }
fn members() -> Vec<&'static str> { vec!["all","append","count","create","delete","find","first","get","has","insert","isEmpty","jsonSerialize","last","map","merge","pluck","push","save","set","toArray","toJson","update","where"] }

