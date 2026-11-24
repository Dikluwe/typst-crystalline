"""Enumerate legal stage schedules from the observed data dependencies."""
import itertools
import json
from pathlib import Path

nodes = ('discovery', 'discovery_index', 'replay', 'attempt_index', 'read_validation')
edges = (
    ('discovery', 'discovery_index'),
    ('discovery_index', 'replay'),
    ('replay', 'attempt_index'),
    ('attempt_index', 'read_validation'),
)
legal = [order for order in itertools.permutations(nodes)
         if all(order.index(before) < order.index(after) for before, after in edges)]
out = {'nodes': nodes, 'edges': edges, 'legal_orders': legal,
       'rejected_orders': 120 - len(legal)}
path = Path(__file__).resolve().parent / 'orders.json'
path.write_text(json.dumps(out, indent=2) + '\n')
print(path, len(legal), out['rejected_orders'])
