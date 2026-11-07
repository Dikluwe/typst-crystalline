#!/usr/bin/env python3
"""Validate aggregate catalog probe TSV without adding inclusive phase clocks.

Debug stack/path/thread fields are opaque. Textual backslash escapes are valid;
actual extra tabs create extra columns and are rejected. Missing files are valid
only in off mode. This consumer does not infer performance from query captures.
"""
import argparse
import json
from pathlib import Path
import re

_DECIMAL = re.compile(r'[0-9]+', re.ASCII)
_U64 = (1 << 64) - 1
_U128 = (1 << 128) - 1


def _decimal(value, *, positive, maximum=None):
    if not _DECIMAL.fullmatch(value):
        raise ValueError('Integer must contain decimal ASCII digits only')
    normalized = value.lstrip('0') or '0'
    if positive and normalized == '0':
        raise ValueError('Integer must be positive')
    if maximum is not None:
        bound = str(maximum)
        if len(normalized) > len(bound) or (len(normalized) == len(bound) and normalized > bound):
            raise ValueError('Integer exceeds protocol range')
        return int(normalized)
    # pid is a positive decimal identifier, not a bounded arithmetic value.
    return normalized


def parse_dump(path, mode, known):
    """Return complete validated blocks, or raise ValueError/OSError.

    `known` is the iterable of permitted exact scope labels. Debug values remain
    opaque strings. This function validates mode; CLI additionally validates the
    purpose (counts versus performance).
    """
    if mode not in ('timing', 'query', 'off'):
        raise ValueError('Unknown probe mode')
    path = Path(path)
    if mode == 'off':
        try:
            path.lstat()
        except FileNotFoundError:
            return []
        raise ValueError('Off mode requires an absent file')
    allowed = set(known)
    text = path.read_text(encoding='utf-8')
    if not text:
        raise ValueError('Active mode requires a nonempty capture')
    # Permit a single terminating LF. Internal blank lines and other record
    # separators are not protocol records. Preserve opaque debug characters.
    lines = text.split('\n')
    if lines[-1] == '':
        lines.pop()
    blocks = []
    current = None
    for number, line in enumerate(lines, 1):
        fields = line.split('\t')
        tag = fields[0]
        try:
            if tag == 'cycle':
                if len(fields) != 3 or current is not None:
                    raise ValueError('Cycle arity or interleaved/incomplete block')
                if not fields[1].startswith('pid=') or not fields[2].startswith('thread='):
                    raise ValueError('Cycle must contain pid then thread')
                pid = _decimal(fields[1][4:], positive=True)
                thread = fields[2][7:]
                if not thread:
                    raise ValueError('Thread debug identifier is empty')
                current = dict(pid=pid, thread=thread, scopes={}, queries=[])
            elif tag == 'scope':
                if len(fields) != 4 or current is None:
                    raise ValueError('Scope arity or missing cycle')
                label = fields[1]
                if label not in allowed:
                    raise ValueError('Unknown scope label')
                if label in current['scopes']:
                    raise ValueError('Duplicate scope in one block')
                count = _decimal(fields[2], positive=True, maximum=_U64)
                ns = _decimal(fields[3], positive=False, maximum=_U128)
                current['scopes'][label] = dict(count=count, ns=ns)
            elif tag == 'query':
                if len(fields) != 6 or current is None:
                    raise ValueError('Query arity or missing cycle')
                if any(not field for field in fields[1:5]):
                    raise ValueError('Query debug stack/operation/path/outcome must be nonempty')
                count = _decimal(fields[5], positive=True, maximum=_U64)
                current['queries'].append(dict(stack=fields[1], operation=fields[2],
                    path=fields[3], outcome=fields[4], count=count))
            elif tag == 'end':
                if len(fields) != 1 or current is None:
                    raise ValueError('End arity or missing cycle')
                if mode == 'timing' and not current['scopes']:
                    raise ValueError('Timing block requires scopes')
                if mode == 'query' and not current['queries']:
                    raise ValueError('Query block requires queries')
                blocks.append(current)
                current = None
            else:
                raise ValueError('Unknown record tag')
        except ValueError as error:
            raise ValueError(f'Line {number}: {error}') from error
    if current is not None:
        raise ValueError('Incomplete final block')
    if not blocks:
        raise ValueError('No complete blocks')
    return blocks


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', required=True)
    parser.add_argument('--mode', choices=('timing', 'query', 'off'), required=True)
    parser.add_argument('--purpose', choices=('counts', 'performance'), required=True)
    parser.add_argument('--known-scope', action='append', default=[])
    args = parser.parse_args()
    try:
        if args.purpose == 'performance' and args.mode != 'timing':
            raise ValueError('Performance evidence requires timing mode')
        blocks = parse_dump(args.input, args.mode, args.known_scope)
        if args.purpose == 'performance' and any(block['queries'] for block in blocks):
            raise ValueError('Query captures cannot be performance evidence')
    except (OSError, UnicodeError, ValueError) as error:
        parser.exit(1, f'Invalid capture: {error}\n')
    print(json.dumps(dict(accepted=True, blocks=len(blocks), mode=args.mode, purpose=args.purpose)))


if __name__ == '__main__':
    main()
