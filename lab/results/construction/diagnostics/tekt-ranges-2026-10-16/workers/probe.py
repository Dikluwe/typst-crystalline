from pathlib import Path
import subprocess,json,datetime,gzip,hashlib
out=Path('00_nucleo/diagnosticos/tekt-ranges-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));source=before['files']['03_infra/src/watch.rs'];old=source[source.index('fn decode_url_path('):source.index('\nfn serve_connection(',source.index('fn decode_url_path('))]
old_digit='''let digit = |value: u8| match value {
                b'0'..=b'9' => Some(value - b'0'),
                b'a'..=b'f' => Some(value - b'a' + 10),
                _ => None,
            };'''
new_digit='let digit = |value: u8| char::from(value).to_digit(16).map(|digit| digit as u8);'
assert old.count(old_digit)==1
new=old.replace('fn decode_url_path(', 'fn candidate_decode_url_path(',1).replace(old_digit,new_digit)
main=r'''
fn main() {
    let original_digit = |value: u8| match value.to_ascii_lowercase() {
        b'0'..=b'9' => Some(value.to_ascii_lowercase() - b'0'),
        b'a'..=b'f' => Some(value.to_ascii_lowercase() - b'a' + 10),
        _ => None,
    };
    let candidate_digit = |value: u8| char::from(value.to_ascii_lowercase()).to_digit(16).map(|digit| digit as u8);
    let mut digits=0;
    for value in 0..=255u8 {assert_eq!(original_digit(value),candidate_digit(value));if original_digit(value).is_some(){digits+=1;}}
    let mut pairs=0;let mut accepted=0;
    for hi in 0..=255u8 {for lo in 0..=255u8 {
        let a=original_digit(hi).and_then(|h|original_digit(lo).map(|l|h*16+l));
        let b=candidate_digit(hi).and_then(|h|candidate_digit(lo).map(|l|h*16+l));
        assert_eq!(a,b);if a.is_some(){accepted+=1;}
        let path=format!("/a%{}{}z",char::from(hi),char::from(lo));
        assert_eq!(decode_url_path(&path),candidate_decode_url_path(&path));pairs+=1;
    }}
    let controls=[("/caf%C3%A9",Some("/café")),("/a%2Fb",Some("/a/b")),("/a/b/../c?d=1",Some("/a/c")),("/%2e%2e/private",None),("/x/%2E%2e/y",Some("/y")),("/%",None),("/%A",None),("/%GG",None),("/%00",None),("/%5c",None),("/%C0%AF",None),("/%FF",None),("/a/",Some("/a/"))];
    for (input,want) in controls {assert_eq!(decode_url_path(input).as_deref(),want);assert_eq!(candidate_decode_url_path(input).as_deref(),want);}
    assert_eq!(digits,22);assert_eq!(accepted,484);
    println!("{{\"bytes_compared\":256,\"accepted_bytes\":{digits},\"pairs_compared\":{pairs},\"accepted_pairs\":{accepted},\"route_controls\":{}}}",controls.len());
}
'''
code=old+'\n'+new+'\n'+main;(out/'probe.rs').write_text(code);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();start=datetime.datetime.now(datetime.timezone.utc).isoformat();cmd=['rustc','--edition','2021','-O',str(out/'probe.rs'),'-o',str(out/'probe')];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr;x=subprocess.run([str(out/'probe')],capture_output=True,text=True);assert x.returncode==0,x.stderr
record=dict(start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,rustc=subprocess.check_output(['rustc','-Vv'],text=True),source_sha256=sha(out/'probe.rs'),binary_sha256=sha(out/'probe'),original_function_sha256=hashlib.sha256(old.encode()).hexdigest(),old_digit=old_digit,new_digit=new_digit,stdout=x.stdout,stderr=x.stderr,exit=x.returncode,measurements=json.loads(x.stdout),sources_unchanged=all(sha(p)==h for p,h in before['hashes'].items()))
(out/'probe-details.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record['measurements']),flush=True)
