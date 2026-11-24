"""Count exact grouping and StyleChain calls separately from CPU timing."""
import hashlib
import json
import os
import subprocess

from bench import ARMS, PROTECT
from watch import D, STATES, WORK, cycle, golden

SUFFIXES = {
    "wait":"::watch::wait_for_change_since",
    "eval":"::eval_to_module_in_session",
    "group":"::TextualSnapshot::same_group",
    "collapse":"::style_chain::StyleChain::collapse",
}


def measure(arm):
    exe = WORK/arm
    content = subprocess.check_output(["nm","-S","-n","-C",str(exe)],text=True)
    addresses = {k:[] for k in SUFFIXES}
    for line in content.splitlines():
        fields = line.split(maxsplit=3)
        if len(fields) != 4 or fields[2] not in ("t","T"):
            continue
        name = fields[3].split(".llvm.",1)[0]
        for kind, suffix in SUFFIXES.items():
            if name.endswith(suffix):
                addresses[kind].append(int(fields[0],16))
    assert all(addresses.values()), (arm,addresses)
    # Exact call instructions verified by objdump and the GOT relocations.
    expected, calls = {
        "candidate": ("c2262d461c8ff488f1bc10feed4baffcfa0107b4cdeb54c39cb6d2568b7ff564",
                      [0x2035aa6,0x2035ab7]),
        "control": ("c059a3dfa68b84b7801074fb7cc8c06aca75dc5100dd1c8fd7781615b3d75f0d",
                    [0x203490d,0x2034920]),
    }[arm]
    with exe.open("rb") as f:
        assert hashlib.file_digest(f,"sha256").hexdigest()==expected
    addresses["group_collapse"] = calls
    case = WORK/"census"/arm
    (case/"edits").mkdir(parents=True)
    (case/"addresses.json").write_text(json.dumps(addresses,indent=2)+"\n")
    for i,(_,source,leaf) in enumerate(STATES):
        assert leaf is None
        (case/("main.typ" if i==0 else f"edits/{i}.typ")).write_bytes(source)
    env = cycle.environment(case,"candidate",False)
    env.update(PROTECT)
    args = ["taskset","-c","1","gdb","-q","-nx","-batch","-x",str(D/"census.gdb"),
            "--args",str(exe),"watch","main.typ","out.pdf","--root",str(case),"--pdf-tagged=true"]
    with (case/"gdb.log").open("wb") as log:
        subprocess.run(args,cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=60,check=True)
    for i,(name,_,_) in enumerate(STATES):
        assert cycle.cycle.key("candidate",case/f"pub-{i}.pdf")==golden("candidate",name)
    counts = json.loads((case/"census.json").read_text())
    assert counts["eval_collapses"]>0 and counts["comparisons"]>0 and not counts.get("unwound"), counts
    return counts


os.sched_setaffinity(0,{2})
result = {arm:measure(arm) for arm in ARMS[:2]}
assert result["candidate"]["comparisons"]==result["control"]["comparisons"], result
assert result["candidate"]["group_collapses"]<result["control"]["group_collapses"], result
(WORK/"census-summary.json").write_text(json.dumps(result,indent=2)+"\n")
print(result,flush=True)
