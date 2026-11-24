from pathlib import Path
import hashlib
import json
import re
import struct
import subprocess
import sys
from xml.etree import ElementTree

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, cycle, STATES
sys.path.insert(0,str(D.parent/'references-500-visible-stable-cids-2026-11-21'))
from pdf_check import structure, text_layout

def sha(data):
    return hashlib.sha256(data).hexdigest()

def words_by_page(pdf):
    raw=subprocess.check_output(['pdftotext','-bbox-layout',str(pdf),'-'],timeout=30)
    root=ElementTree.fromstring(raw)
    pages=[]
    for page in root.findall('.//{*}page'):
        words=[(node.text,tuple(sorted(node.attrib.items()))) for node in page.findall('.//{*}word')]
        pages.append({'extent':page.attrib,'sha256':sha(json.dumps(words).encode()),
                      'first_words':[word for word,_ in words[:12]]})
    return pages

def source_proof(old,new):
    anchor=b'= Section 1 <s1>'
    before=old[old.index(anchor):]
    raw_after=new[new.index(anchor):]
    comment=b"\n// next comment; same visible source\n"
    assert new.endswith(comment)
    # The session already verified both comment-only publications as equal.
    after=raw_after.removesuffix(comment)
    config=lambda source: re.findall(rb'^#set (?:page|heading)\(.*\)$',source,re.M)
    refs=lambda source:re.findall(rb'^See @(s\d+)\.$',source,re.M)
    heads=lambda source:re.findall(rb'^= Section (\d+) <(s\d+)>$',source,re.M)
    return {'suffix_equal':before==after,'raw_suffix_equal':before==raw_after,
            'removed_known_terminal_comment':raw_after!=after,'old_suffix_sha256':sha(before),
            'new_suffix_sha256':sha(after),
            'configuration_equal':config(old)==config(new),
            'references_equal':refs(old)==refs(new),
            'headings_equal':heads(old)==heads(new),
            'source_file_old':sha(old),'source_file_new':sha(new),
            'source_anchor':anchor.decode()}

def scalar_label(value):
    return struct.unpack('<d',value.to_bytes(8,'little'))[0]

def snapshot(point):
    current=point['successor']
    assert all(n in current for n in ('cursor_x','cursor_y','region_width','region_height'))
    return {'closed_pages':current['closed_pages'],
            'cursor_pt':[scalar_label(current[k]) for k in ('cursor_x','cursor_y','line_start_x')],
            'paper_pt':[scalar_label(current[k]) for k in ('page_width','page_height')],
            'assumed_ascent_pt':scalar_label(current['ascent_payload']) if 'ascent_payload' in current else None,
            'line_commits_at_boundary':point['line_commits_after_transition'],
            'pending_selected_line':point.get('pending_selected_line'),
            'active_emitter':next((name for name,_ in point['stack'] if name and ('::paragraph_emit::' in name or '::word_layout::' in name)),None),
            'refusals':point['refusals']}

def cold(name,data):
    case=WORK/'cold-controls'/name
    case.mkdir(parents=True,exist_ok=True)
    source=case/'main.typ'
    pdf=case/'fresh.pdf'
    if source.exists() and pdf.exists() and source.read_bytes()==data:
        return pdf
    source.write_bytes(data)
    env=cycle.environment(case,'candidate',False)
    with (case/'fresh.log').open('wb') as log:
        subprocess.run(['taskset','-c','1',str(WORK/'control'),'compile',str(source),str(pdf),'--root',str(case),'--pdf-tagged=true'],
                       check=True,stdout=log,stderr=subprocess.STDOUT,env=env,timeout=60)
    return pdf

describe=cycle.cycle.original.compare.describe
results={}
for case in ('original','height','reference'):
    folder=WORK/f'checked-{case}'
    before=folder/'publication-0.pdf'; after=folder/'publication-3.pdf'
    edited=(folder/'edits/3.typ').read_bytes()
    proof=source_proof(STATES[0][1],edited)
    a,b=describe(before),describe(after)
    p,q=words_by_page(before),words_by_page(after)
    af,bf=cold(f'{case}-open',STATES[0][1]),cold(f'{case}-edit',edited)
    for fresh,warm in [(af,before),(bf,after)]:
        assert describe(fresh)==describe(warm)
        assert text_layout(fresh)==text_layout(warm)
        assert structure(fresh)==structure(warm)
    raw=json.loads((OUT/f'{case}-snapshots.json').read_text())
    for version in ('0','3'):
        for i,boundary in enumerate(raw[version],1):
            assert boundary['before']['closed_pages']==i-1
            assert boundary['successor']['closed_pages']==i
            assert not boundary['existing_checkpoint_admits']
    first, second=raw['0'][1],raw['3'][1]
    dest_a,dest_b=a['named_destinations'],b['named_destinations']
    changed=sorted(k for k in set(dest_a)|set(dest_b) if dest_a.get(k)!=dest_b.get(k))
    links=lambda x: [[item.get('/Dest') or item.get('/A',{}).get('/D') for item in page] for page in x['links']]
    admissions={
        'opening':snapshot(first),'edit':snapshot(second),
        'successor_scalars_equal_bits':first['successor']==second['successor'],
        'same_pending_line_operands':first.get('pending_selected_line')==second.get('pending_selected_line'),
        'source':proof,
        'continuation_owned': first['existing_checkpoint_admits'] and second['existing_checkpoint_admits'],
    }
    admissions['admit']=(admissions['continuation_owned'] and admissions['successor_scalars_equal_bits']
                         and all(proof[name] for name in ('suffix_equal','configuration_equal','references_equal','headings_equal')))
    evidence={'pages':[len(p),len(q)],'suffix_text_geometry_equal':p[2:]==q[2:],
              'first_words_page_3':[p[2]['first_words'],q[2]['first_words']],
              'links':[sum(map(len,a['links'])),sum(map(len,b['links']))],
              'link_targets_equal':links(a)==links(b),
              'link_annotations_equal':a['links']==b['links'],
              'named_destinations':[len(dest_a),len(dest_b)],'changed_destinations':changed,
              'bookmarks_equal':a['outline_sha256']==b['outline_sha256'],
              'matched_independent_cold':True}
    counts=json.loads((OUT/f'{case}-counts.json').read_text())
    available=counts['3:line_commit']-second['line_commits_after_transition']
    results[case]={'checkpoint':admissions,'output_oracle':evidence,'counts':counts,
                   'later_line_commits_in_full_layout':available,'line_commits_avoided':0}
    print(case,admissions,evidence,flush=True)
assert results['original']['checkpoint']['source']['suffix_equal']
assert results['original']['checkpoint']['successor_scalars_equal_bits']
assert results['original']['output_oracle']['suffix_text_geometry_equal']
assert results['original']['output_oracle']['link_targets_equal']
assert not results['height']['checkpoint']['source']['configuration_equal']
assert not results['height']['checkpoint']['successor_scalars_equal_bits']
assert not results['reference']['checkpoint']['source']['references_equal']
assert not results['reference']['output_oracle']['link_targets_equal']
(OUT/'admission.json').write_text(json.dumps(results,indent=2)+'\n')
