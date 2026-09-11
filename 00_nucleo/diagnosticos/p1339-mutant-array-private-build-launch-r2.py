"""Record inherited compiler environment before launching the frozen build runner."""
import datetime, hashlib, json, os, pathlib, subprocess
D=pathlib.Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def new(p,obj):
    with p.open('x') as f:json.dump(obj,f,indent=2);f.write('\n')
manifest=D/'p1339-mutant-array-private-build-manifest-r2.json'
m=json.loads(manifest.read_text())
envkeys={'PATH','HOME','LANG','LANGUAGE','LC_ALL','TZ','CARGO_HOME','RUSTUP_HOME','RUSTUP_TOOLCHAIN',
 'RUSTC','RUSTDOC','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTDOCFLAGS','CARGO_ENCODED_RUSTDOCFLAGS',
 'RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','CARGO_TARGET_DIR','CARGO_BUILD_TARGET','CARGO_BUILD_JOBS',
 'CARGO_INCREMENTAL','SOURCE_DATE_EPOCH','CC','CXX','AR','LD','CFLAGS','CXXFLAGS','LDFLAGS','NO_COLOR'}
relevant={k:v for k,v in os.environ.items() if k in envkeys or k.startswith(('LC_','TYPST_','CARGO_PROFILE_','CARGO_TARGET_'))}
before={'utc':utc(),'manifest_sha256':sha(manifest),'launcher_sha256':sha(__file__),
 'inherited_environment':relevant,'explicit_absent_keys':sorted(envkeys-set(os.environ)),
 'runner_delta':m['env_delta'],'effective_recorded_environment':{**relevant,**m['env_delta']},
 'scope':'compiler/provenance relevant variables; no token/credential variables collected',
 'pins_before':{name:sha(D/name) for name in m['pins']}}
new(D/'p1339-mutant-array-private-build-environment-before-r2.json',before)
argv=['python3',str(D/'p1339-mutant-array-private-build-r2.py'),'--go',str(D/'p1339-verifier-array-private-build-GO-r1.json')]
code=subprocess.call(argv,cwd=D.parents[1])
after={'utc':utc(),'launch_argv':argv,'exit':code,'pins_after':{name:sha(D/name) for name in m['pins']}}
after['pins_intact']=after['pins_after']==before['pins_before']
new(D/'p1339-mutant-array-private-build-environment-after-r2.json',after)
raise SystemExit(code)
