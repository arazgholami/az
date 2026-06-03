use crate::{COMMENT, GREEN2, PURPLE, Segment, CompletionItem};
use super::{comp, find_between, find_prefixed_words, suffix_token, CompletionContext};

pub(crate) fn segments(line: &str) -> Vec<Segment> {
    let mut s = Vec::new();
    for (start, end) in find_prefixed_words(line, '@') { s.push(Segment { start, end, color: PURPLE }); }
    for (start, end) in find_between(line, "{{", "}}") { s.push(Segment { start, end, color: GREEN2 }); }
    for (start, end) in find_between(line, "{{--", "--}}") { s.push(Segment { start, end, color: COMMENT }); }
    s
}

pub(crate) fn completion_context(before: &str, _explicit: bool) -> Option<(String, String, usize)> {
    suffix_token(before, '@').map(|(prefix, start)| ("blade:directive".to_string(), prefix, start))
}

pub(crate) fn completion_items(kind: &str, _ctx: CompletionContext<'_>) -> Vec<CompletionItem> {
    match kind {
        "blade:directive" => directives().iter().map(|s| comp(s, s, "Blade")).collect(),
        _ => Vec::new(),
    }
}

pub(crate) fn symbols(line: &str) -> Vec<String> {
    let trimmed = line.trim_start();
    for dir in ["@section", "@if", "@foreach", "@forelse", "@while", "@switch", "@component"] {
        if trimmed.starts_with(dir) { return vec![trimmed.to_string()]; }
    }
    Vec::new()
}

fn directives() -> Vec<&'static str> { vec!["@auth","@aware","@break","@can","@cannot","@case","@choice","@class","@component","@continue","@csrf","@dd","@disabled","@each","@else","@elseif","@empty","@endauth","@endcan","@endcannot","@endcase","@endcomponent","@endempty","@endenv","@enderror","@endforeach","@endif","@endisset","@endonce","@endproduction","@endpush","@endsection","@endswitch","@endunless","@endwhile","@env","@error","@extends","@foreach","@forelse","@if","@include","@isset","@json","@lang","@method","@once","@php","@production","@props","@push","@section","@selected","@stack","@style","@switch","@unless","@vite","@while","@yield"] }
