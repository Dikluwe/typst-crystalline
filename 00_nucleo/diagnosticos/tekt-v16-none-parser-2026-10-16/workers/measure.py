from pathlib import Path
import subprocess
import tempfile
import json
import datetime

out = Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
constructor = 'enum Mode { A, B }\nfn classify(v: Option<Mode>) -> u8 { match v { Some(Mode::A) => 0, Some(Mode::B) => 1, None => 2 } }\n'
binding = 'enum Mode { A, B, C }\nfn classify(v: Mode) -> u8 { match v { Mode::A => 0, Mode::B => 1, None => 2 } }\n'
cases = {
    'constructor': constructor,
    'raw_constructor': constructor.replace('None =>', 'r#None =>'),
    'disabled_root': '#![no_implicit_prelude]\n' + binding,
    'disabled_inner': 'mod inner { #![no_implicit_prelude]\n' + binding + '}\n',
    'disabled_outer': '#[no_implicit_prelude]\nmod inner {\n' + binding + '}\n',
    'conditional_disabled': '#![cfg_attr(disable, no_implicit_prelude)]\n' + binding,
    'other_uppercase_binding': binding.replace('None =>', 'Other =>'),
}
record = {'start': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'cases': {}}
cache = Path('/tmp/tekt-linter-update-target/debug/deps')
with tempfile.TemporaryDirectory(prefix='tekt-none-syntax-') as temp:
    root = Path(temp)
    binary = root/'syntax'
    command = ['rustc', '--edition=2021', '-L', 'dependency='+str(cache)]
    for name in ['tree_sitter','tree_sitter_rust','serde_json']:
        libs=list(cache.glob('lib'+name+'-*.rlib'));assert len(libs)==1
        command += ['--extern',name+'='+str(libs[0])]
    command += [str(out/'workers/syntax.rs'),'-o',str(binary)]
    subprocess.run(command,check=True,capture_output=True,text=True)
    for name, code in cases.items():
        path=root/(name+'.rs');path.write_text(code)
        command=['rustc','--edition=2021','--crate-type','lib',str(path),'-o',str(root/(name+'.rlib'))]
        if name=='conditional_disabled':command += ['--cfg','disable']
        result=subprocess.run(command,capture_output=True,text=True)
        assert result.returncode==0,(name,result.stderr)
        parsed=subprocess.run([str(binary),str(path)],capture_output=True,text=True,check=True)
        record['cases'][name]={'source':code,'rustc_command':command,'rustc_exit':result.returncode,
            'rustc_stderr':result.stderr,'ast':json.loads(parsed.stdout)['ast']}
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat()
(out/'syntax-measurement.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'valid_cases':list(cases),'all_rustc_success':True}))
