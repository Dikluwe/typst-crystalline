"""Record absolute call counts and identify where the retained 490 calls originate."""
from bisect import bisect_right
from collections import Counter
import hashlib
import json
import os
import subprocess
import sys

from bench import ARMS, PROTECT
from watch import D, STATES, WORK, cycle, golden
from symbols import COLLAPSES, ELF_SHA, MARKUP_FIND, NATIVE, TABLES, TEKT


def measure(arm):
    exe = WORK/arm
    with exe.open("rb") as f:
        assert hashlib.file_digest(f,"sha256").hexdigest()==ELF_SHA[arm]
    suffixes = NATIVE if arm=="vanilla" else TEKT
    entries = {}
    addresses = {key:[] for key in suffixes}
    for line in TABLES[arm].read_text().splitlines():
        fields = line.split(maxsplit=3)
        if len(fields)!=4 or fields[2] not in ("t","T"):
            continue
        address, size = (int(x,16) for x in fields[:2])
        function = fields[3].split(".llvm.",1)[0]
        entries[address] = (size,function)
        for kind, choices in suffixes.items():
            if function.endswith(choices):
                addresses[kind].append(address)
    assert all(addresses.values()), (arm,addresses)
    if len(sys.argv)>1:
        assert sys.argv[1]=="--native-add" and arm=="vanilla"
        addresses.update(native_lhs_sequence=[0x2bba1c7], native_rhs_sequence_only=[0x2bba283],
                         native_new_pair=[0x2bba1bd], native_clone_lhs=[0x2bba1ea,0x2bba364],
                         native_clone_rhs=[0x2bba298], native_grow_lhs=[0x2bba245,0x2bba476])
    starts = sorted(entries)
    case = WORK/"census"/arm
    (case/"edits").mkdir(parents=True)
    (case/"addresses.json").write_text(json.dumps(addresses,indent=2)+"\n")
    for i,(_,source,leaf) in enumerate(STATES):
        assert leaf is None
        (case/("main.typ" if i==0 else f"edits/{i}.typ")).write_bytes(source)
    variant = "vanilla" if arm=="vanilla" else "candidate"
    env = cycle.environment(case,variant,False)
    if arm!="vanilla":
        env.update(PROTECT)
    args = ["taskset","-c","1","gdb","-q","-nx","-batch","-x",str(D/"census.gdb"),
            "--args",str(exe),"watch","main.typ","out.pdf","--root",str(case),"--pdf-tagged=true"]
    with (case/"gdb.log").open("wb") as log:
        subprocess.run(args,cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=85,check=True)
    for i,(name,_,_) in enumerate(STATES):
        assert cycle.cycle.key(variant,case/f"pub-{i}.pdf")==golden(variant,name)
    raw = json.loads((case/"counts.json").read_text())
    totals = {kind:sum(raw.get(kind,{}).values()) for kind in addresses if kind!="watch_wait"}
    assert totals["eval"]==1
    callers, buckets = {}, {}
    for kind, records in raw.items():
        if kind=="eval":
            continue
        names, classified = Counter(), Counter()
        for ret, hits in records.items():
            ret = int(ret)
            loc = ret-1
            index = bisect_right(starts,loc)-1
            name = "[unresolved]"
            if index >= 0:
                address = starts[index]
                size, symbol = entries[address]
                if loc < address+size:
                    name = symbol
            names[name] += hits
            if kind=="collapse":
                classified[COLLAPSES[arm].get(ret,"other")] += hits
            if kind=="find":
                if "::LinkedNode::find" in name:
                    classified["recursive"] += hits
                elif arm!="vanilla" and ret==MARKUP_FIND[arm]:
                    classified["markup_origin"] += hits
                elif "::control_flow::run_for_loop" in name:
                    classified["memo_key"] += hits
                else:
                    classified["other_root"] += hits
        callers[kind], buckets[kind] = names, classified
    return dict(totals=totals,callers=callers,buckets=buckets)


os.sched_setaffinity(0,{2})
result = {}
for arm in (("vanilla",) if len(sys.argv)>1 else ARMS):
    result[arm] = measure(arm)
    print(arm,result[arm]["totals"],result[arm]["buckets"],flush=True)
for arm,n in (() if len(sys.argv)>1 else (("candidate",517),("control",7106))):
    assert result[arm]["totals"]["collapse"]==n
    assert sum(v for k,v in result[arm]["buckets"]["collapse"].items() if k!="group")==490
if len(sys.argv)>1:
    count = result["vanilla"]["totals"]
    assert count["concat"]==sum(count[k] for k in (
        "native_lhs_sequence","native_rhs_sequence_only","native_new_pair"))
(WORK/("native-add.json" if len(sys.argv)>1 else "census-summary.json")).write_text(json.dumps(result,indent=2)+"\n")
