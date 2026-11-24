import json
import os
import subprocess

from topology import OUT, STATES, WORK, cycle, describe, execute, options, structure, text_layout


def fresh(case, data):
    case.mkdir()
    (case / 'main.typ').write_bytes(data)
    env = cycle.environment(case, 'candidate', False)
    env.update(options)
    with (case / 'compile.log').open('wb') as log:
        subprocess.run(['taskset', '-c', '1', str(WORK / 'control'), 'compile',
                        'main.typ', 'out.pdf', '--root', str(case), '--pdf-tagged=true'],
                       cwd=case, env=env, stdout=log, stderr=subprocess.STDOUT,
                       timeout=120, check=True)
    return case / 'out.pdf'


def replacement(data, old, new):
    assert data.count(old) == 1, old
    return data.replace(old, new)


def check_pdf(current, expected):
    info = describe(current)
    assert info == describe(expected), current
    assert structure(current) == structure(expected), current
    assert text_layout(current) == text_layout(expected), current
    return info


def main():
    before = json.loads((OUT / 'topology-control.json').read_text())['counts']
    after = json.loads((OUT / 'topology-candidate.json').read_text())['counts']
    assert before['3:line_commit'] == after['3:line_commit'] == 57
    assert before['3:index'] == 3 and after['3:index'] == 3
    trials = json.loads((OUT / 'topology-candidate-trials.json').read_text())
    nominal = [t for t in trials if t['publication'] == 3]
    assert any(t.get('scope') == 'index_reuse' and t['result'] > 100_000 for t in nominal)
    blocks = [t for t in nominal if t.get('scope') == 'block' and t['reused_packets'] > 0]
    assert before['3:plan'] == after['3:plan'] == 4102
    assert before['3:reserve_visit'] == 6940
    (OUT / 'green.json').write_text(json.dumps(dict(control=before, candidate=after,
        trials=trials, accepted_blocks=blocks), indent=2) + '\n')
    assert len(blocks) == 1 and blocks[0]['reused_packets'] == 3258, (trials, after)
    assert after['3:reserve_visit'] < before['3:reserve_visit'], (trials, after)
    reused, = [t for t in nominal if t.get('scope') == 'reserve_reuse']
    assert reused['parts'] > 0 and reused['location_after'] > 0, reused

    oracle = WORK / 'whole-reservation-oracle'
    os.environ['TEKT_LAB_RESERVE_VERIFY'] = '1'
    try:
        oracle_counts, _ = execute('candidate', oracle, STATES)
    finally:
        del os.environ['TEKT_LAB_RESERVE_VERIFY']
    oracle_trials = json.loads((oracle / 'trials.json').read_text())
    full, = [t for t in oracle_trials if t['publication']==3 and t.get('scope')=='reserve_reuse']
    assert full['parts'] == 0 and oracle_counts['3:reserve_visit'] == before['3:reserve_visit']
    assert reused['context_floor'] == full['context_floor'], (reused, full)
    assert reused['location_after'] == full['location_after'], (reused, full)
    live = WORK / os.environ.get('TEKT_LAB_TOPOLOGY_CASE', 'topology-candidate')
    for index in range(4):
        check_pdf(oracle / f'publication-{index}.pdf', live / f'publication-{index}.pdf')
    (OUT / 'oracle.json').write_text(json.dumps(dict(full_counts=oracle_counts,
        full_reservation=full, retained_reservation=reused, matching_publications=4), indent=2) + '\n')
    (OUT / 'oracle.log').write_bytes((oracle / 'gdb.log').read_bytes())

    visible = STATES[-1][1]
    suffix = b'= Section 19 <s19>\nSee @s19.\n#for j in range(25) [Paragraph'
    cases = (
        ('visible', visible, True),
        ('height', replacement(visible, b'height:600pt', b'height:440pt'), False),
        ('prefix-reference', replacement(visible, b'See @s0.', b'See @s13.'), False),
        ('suffix-reference', replacement(visible, b'See @s19.', b'See @s13.'), False),
        ('suffix-heading', replacement(visible, b'= Section 13 <s13>', b'= Renamed section <s13>'), False),
        ('text-style', replacement(visible, b'#outline()', b'#set text(size:12pt)\n#outline()'), False),
        ('suffix-context', visible + b'\n#context [New suffix context with @s13.]\n', False),
        ('suffix-text', replacement(visible, suffix, suffix.removesuffix(b'Paragraph') + b'Updated paragraph'), False),
        ('prefix-wrap', replacement(visible, b'Visible paragraph',
            b'Many additional words make this descriptive visible paragraph take several extra lines'), True),
    )
    results = {}
    for name, data, inside_grammar in cases:
        case = WORK / os.environ.get('TEKT_LAB_DIRECTED_CASES', 'directed') / name
        counts, _ = execute('candidate', case, [*STATES[:3], (name, data, None)], reduced=True)
        active = [t for t in json.loads((case / 'trials.json').read_text()) if t['publication'] == 3]
        expected = fresh(case / 'fresh', data)
        pdf = case / 'publication-3.pdf'
        info = check_pdf(pdf, expected)
        if not inside_grammar:
            assert not counts.get('3:plan_reuse') and not counts.get('3:index_reuse'), (name, active)
            assert not any(t.get('reused_pages', 0) or t.get('reused_packets', 0)
                           or (t.get('scope') == 'reserve_reuse' and t['parts'])
                           for t in active), (name, active)
        if name == 'visible':
            assert info['page_count'] == 23 and sum(map(len, info['links'])) == 40
        if name == 'prefix-wrap':
            assert info['page_count'] == 24
            assert any(t.get('scope') == 'reserve_reuse' and t['parts'] == reused['parts']
                       for t in active), active
            assert not any(t.get('reused_pages', 0) or
                           (t.get('scope') == 'index_reuse' and t['result'] > 100_000)
                           for t in active), active
        links = info['links']
        outline = info['outline_sha256']
        if name == 'visible':
            nominal_links, nominal_outline = links, outline
            nominal_text = info['text_geometry_font_sha256']
        if name in ('prefix-reference', 'suffix-reference'):
            assert links != nominal_links and sum(map(len, links)) == sum(map(len, nominal_links)), name
        if name == 'text-style':
            assert info['text_geometry_font_sha256'] != nominal_text
        if name == 'suffix-heading':
            assert outline != nominal_outline
            assert b'Renamed section' in subprocess.check_output(
                ['pdftotext', '-f', '1', '-l', '1', str(pdf), '-'], timeout=20)
        if name == 'suffix-text':
            assert info['text_geometry_font_sha256'] != nominal_text
            assert b'Updated paragraph' in subprocess.check_output(
                ['pdftotext', str(pdf), '-'], timeout=20)
        if name == 'suffix-context':
            assert sum(map(len, links)) == 41
            assert b'New suffix context' in subprocess.check_output(
                ['pdftotext', str(pdf), '-'], timeout=20)
        if name == 'height':
            assert info['page_count'] != 23
        results[name] = dict(pages=info['page_count'], links=sum(map(len, links)),
                             bookmarks_sha256=outline, toc_text_verified=name == 'suffix-heading',
                             link_targets_changed=links != nominal_links, trials=active,
                             agrees_with_fresh=True)
        print(name, results[name], flush=True)
    (OUT / 'directed.json').write_text(json.dumps(results, indent=2) + '\n')


if __name__ == '__main__':
    main()
