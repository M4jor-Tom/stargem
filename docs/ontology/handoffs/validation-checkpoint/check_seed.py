import json
import math
import re
from pathlib import Path

root = Path('/home/theta/repos/stargem.nix/ontology/instances')
records = numbers = keys = 0

def unique_object(pairs):
    global keys
    out = {}
    for k, v in pairs:
        assert k not in out, ('duplicate JSON key', k)
        keys += 1
        out[k] = v
    return out

def walk(value):
    global numbers
    if isinstance(value, dict):
        for v in value.values(): walk(v)
    elif isinstance(value, list):
        for v in value: walk(v)
    elif isinstance(value, (int, float)) and not isinstance(value, bool):
        assert math.isfinite(value) and abs(value) <= 3.4028234663852886e38, value
        numbers += 1

for path in sorted(root.glob('*.json')):
    data = json.loads(path.read_text(), object_pairs_hook=unique_object)
    ids = [r['id'] for r in data]
    assert len(ids) == len(set(ids)), path
    assert all(re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', i) for i in ids), path
    walk(data)
    records += len(data)
    print(f'{path.name}: {len(data)} unique kebab-case IDs')
assert records == 46
print(f'PASS: 8 files, {records} records, {numbers} finite f32-representable numeric inputs, {keys} unambiguous object keys')
