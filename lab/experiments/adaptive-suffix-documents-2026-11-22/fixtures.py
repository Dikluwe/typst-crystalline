"""Documents with different headings, dimensions, numbering and loop sizes."""

def document(width, height, numbering, outline, headings, count, lines, edit, extra=""):
    header = f'#set page(width:{width}pt,height:{height}pt)\n#set heading(numbering:"{numbering}")\n{extra}{outline}\n'
    sections = []
    for i, (title, line) in enumerate(zip(headings, lines, strict=True)):
        sections.append(f'= {title} <chapter-{i}>\nSee @chapter-{i}.\n'
                        f'#for item in range({count}) [{line} #item with a record for the next team to inspect.\n\n]\n')
    old = header + ''.join(sections)
    revised = sections.copy()
    revised[edit] = sections[edit].replace(lines[edit], 'Final ' + lines[edit].split(' ', 1)[1], 1)
    new = header + ''.join(revised)
    return tuple((name, data.encode(), None) for name, data in (
        ('startup', old), ('comment', old + '\n// first inert note\n'),
        ('comment-2', old + '\n// second inert note\n'), ('visible', new)
    ))


manual_titles = ('Arrival', 'Inventory', 'Console', 'Network', 'Alerts', 'Recovery', 'Audit', 'Handoff')
manual = document(420, 560, '1.', '#outline()', manual_titles, 19,
                  tuple(f'{title} checklist entry' for title in manual_titles), 0)
release_titles = ('Summary', 'Editor', 'Parser', 'Tables', 'Shapes', 'Images',
                  'Metadata', 'Search', 'Navigation', 'Exports', 'Upgrade')
release = document(455, 505, 'A.', '#outline(title: none)', release_titles, 16,
                   tuple(f'{title} verification case' for title in release_titles), 3)
styled_titles = ('Introduction', 'Sampling', 'Results', 'Review', 'Appendix')
styled = document(400, 510, '1.', '#outline()', styled_titles, 20,
                  tuple(f'{title} note' for title in styled_titles), 1,
                  extra='#set text(size: 10pt)\n')
DOCUMENTS = {'manual': manual, 'release-notes': release, 'styled-report': styled}

# Changes to non-masked syntax are valid inputs, but invalidate the paired certificate.
title_edit = manual[1][1].replace(b'= Network ', b'= Networking ', 1)
assert title_edit != manual[1][1]
DIRECTED = {**DOCUMENTS, 'heading-change': (*manual[:3], ('visible', title_edit, None))}
