"""Language-level heading, link and bookmark checks for directed PDF edits."""
import re
import subprocess


def inspect(pdf, fingerprint):
    raw = subprocess.check_output(['pdftotext', '-raw', str(pdf), '-'], text=True)
    heading = re.compile(r'^(\d+)\. Section (\d+)[ \t]*$', re.M)
    titles = {}
    for page, page_text in enumerate(raw.split('\f'), 1):
        for title in heading.finditer(page_text):
            number, section = map(int, title.groups())
            if number == section + 1:
                titles[section] = page
    first = heading.search(raw)
    toc = [(int(number), int(section), int(page)) for number, section, page in re.findall(
        r'^(\d+)\. Section (\d+)\s+(?:\.\s*)+\u2060(\d+)[ \t]*$',
        raw[:first.start()] if first else '', re.M)]
    annotations = [annotation for page in fingerprint['links']
                   for annotation in sorted(page, key=lambda item: -item['/Rect'][3])]
    destinations = fingerprint['named_destinations']

    def arrival(annotation):
        destination = annotation.get('/Dest')
        if destination is None:
            destination = annotation.get('/A', {}).get('/D')
        if destination is None:
            return None
        resolved = destinations[destination] if isinstance(destination, str) else destination
        return {'name': destination, 'page': resolved[0][1]}

    links = [resolved for annotation in annotations if (resolved := arrival(annotation))]
    bookmarked = subprocess.check_output(['mutool', 'show', str(pdf), 'outline'], text=True)
    marks = [(int(section), int(page)) for section, page in re.findall(
        r'"(?:\d+\. )?Section (\d+)"\s+[^\n]*?#page=(\d+)', bookmarked)]
    toc_links = [link for link in links if isinstance(link['name'], str)
                 and link['name'].startswith('u:auto-toc-')]
    toc_links = toc_links[-len(toc):] if toc else []
    reference_links = [link for link in links if isinstance(link['name'], str)
                       and link['name'].startswith('u:s')]
    return {
        'heading_pages': titles,
        'toc_printed_pages': {section: page for _, section, page in toc},
        'toc_links': toc_links,
        'reference_links': reference_links,
        'bookmark_pages': dict(marks),
        'wrong_pages': {
            'toc_printed': [i for _, i, page in toc if titles.get(i) != page],
            'toc_links': [i for i, link in enumerate(toc_links) if titles.get(i) != link['page']],
            'bookmarks': [i for i, page in marks if titles.get(i) != page],
            'references': [i for link in reference_links
                           if isinstance(link['name'], str)
                           if link['name'].startswith('u:s')
                           for i in [int(link['name'][3:])]
                           if titles.get(i) != link['page']],
        },
    }
