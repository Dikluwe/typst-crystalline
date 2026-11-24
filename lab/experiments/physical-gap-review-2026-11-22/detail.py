"""Inclusive hot subscopes within the independently sampled phase profile."""
import json
from pathlib import Path
from statistics import mean
import tarfile

import analyze

OUT = analyze.OUT
rows = [json.loads(row) for row in (OUT / 'profile.jsonl').read_text().splitlines()]
tables = {arm: analyze.decoder.symbols(OUT / (arm + '.symbols'))
          for arm in ('tekt', 'vanilla')}
patterns = {
    'tekt': {
        'physical': {
            'planned_layout': ('::compiler::layout::convergence::run_planned',),
            'font_metrics': ('::font_metrics::fallback::',),
            'paragraph_pagination': ('::paragraph_pagination::',),
            'line_commit': ('::flush_line_with_outer_singleton',),
            'memo_validation': ('::physical_memo::frame_charge', '::physical_memo::style_charge'),
            'link_resolution': ('::layout::link_capture::',),
            'position_markers': ('::layout::position_marker::',),
            'bidi_origin': ('::layout::bidi_origin::',),
        },
        'pdf': {
            'zlib': ('::images::compress_zlib', 'miniz_oxide::deflate::'),
            'stream_text': ('::export::stream::text::',),
            'resource_walk': ('::export::tiling::walk_resources', '::pipeline::resources::'),
            'page_stream': ('::export::stream::page::',),
        },
        'index': {
            'page_running': ('::introspect::page_running::introspect_with_page_running',),
            'walk': ('::introspect::walk::',),
        },
    },
    'vanilla': {
        'physical': {'layout_body': ('typst_layout::pages::layout_document_common',)},
        'pdf': {'pdf_export': ('typst_pdf::pdf',)},
    },
}
result = {}
with tarfile.open(OUT / 'profiles.tar.gz') as raw:
    for arm, table in tables.items():
        sessions = []
        for row in (r for r in rows if r['arm'] == arm):
            stacks, timing = analyze.decoder.stacks(raw, row, tables)
            scale = timing['main_ns'] / 1e6 / timing['samples']
            scopes = {}
            for phase, subs in patterns[arm].items():
                for name, fragments in subs.items():
                    n = sum(count for stack, count in stacks.items()
                            if analyze.phase(arm, stack) == phase
                            and any(fragment in frame for fragment in fragments for frame in stack))
                    scopes[f'{phase}/{name}'] = n * scale
            sessions.append(scopes)
        result[arm] = {name: mean(s[name] for s in sessions) for name in sessions[0]}
(OUT / 'hot-subscopes.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
