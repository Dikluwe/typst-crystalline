const fs = require('fs');
const cp = require('child_process');
const crypto = require('crypto');
const path = require('path');
const root = '/repos/Antigravity/typst-crystalline';
const dir = '/tmp/p1321-ab.VBRiJC';
const out = path.join(root, '00_nucleo/diagnosticos');
const sha = x => crypto.createHash('sha256').update(x).digest('hex');
const baseline = '/tmp/p1319-target.VqXtmj/release/typst';
const vanilla = '/usr/local/bin/typst';
const cases = [];
function add(id, expr, policy='vanilla', setup='') {
  cases.push({id, source: setup + '#metadata(' + expr + ') <result>\n', policy});
}
add('missing', 'csv()');
add('missing-option', 'csv(delimiter: 2, weird: true)');
add('missing-named-source', 'csv(weird: true, source: 42, delimiter: 2)');
add('named-source-with', 'f()', 'vanilla', '#let f = csv.with(source: bytes("a,b"))\n');
add('missing-with', 'f()', 'vanilla', '#let f = csv.with(delimiter: ";")\n');
add('source-before-unknown', 'csv(42, weird: true, delimiter: 2)');
add('source-after-unknown', 'csv(weird: true, 42)');
add('source-before-option', 'csv(delimiter: 2, false, row-type: 3)');
add('source-args-map', 'csv(..a)', 'vanilla', '#let a = arguments(42, weird: true).map(v => v)\n');
add('delimiter-before-unknown', 'csv(bytes("a,b"), weird: true, delimiter: "xx")');
add('delimiter-before-row', 'csv(bytes("a,b"), row-type: 3, delimiter: "é", weird: true)');
add('row-before-unknown', 'csv(bytes("a,b"), weird: true, row-type: str)');
add('delimiter-before-io', 'csv("missing.csv", delimiter: "xx", 8)');
add('row-before-excess', 'csv(bytes("a,b\\n1"), 8, row-type: 4)');
add('excess-valid', 'csv(bytes("a,b"), 7)');
add('excess-malformed', 'csv(bytes("a,b\\n1"), 7)');
add('excess-invalid-utf8', 'csv(bytes((255,)), 7)');
add('excess-missing-path', 'csv("missing.csv", 7)');
add('excess-text-path', 'csv("short.csv", 7)');
add('excess-binary-path', 'csv("invalid.csv", 7)');
add('unknown-before-excess', 'csv(bytes("a,b\\n1"), zeta: 1, 7, alpha: 2)');
add('excess-before-unknown', 'csv(bytes("a,b\\n1"), 7, zeta: 1)');
add('unknown-order-z-a', 'csv(bytes("a,b"), zeta: 1, alpha: 2)');
add('unknown-order-a-z', 'csv(bytes("a,b"), alpha: 1, zeta: 2)');
add('named-source-after-pos', 'csv(bytes("a,b"), source: bytes("c,d"))');
add('unknown-before-source', 'csv(zeta: 1, bytes("a,b"), delimiter: ",")');
add('with-unknown-first', 'f(8)', 'vanilla', '#let f = csv.with(bytes("a,b"), zeta: 4)\n');
add('with-excess-first', 'f(zeta: 4)', 'vanilla', '#let f = csv.with(bytes("a,b"), 8)\n');
add('args-excess', 'csv(..a)', 'vanilla', '#let a = arguments(bytes("a,b"), 8)\n');
add('args-map-excess', 'csv(..a)', 'vanilla', '#let a = arguments(bytes("a,b"), 8).map(v => v)\n');
add('args-filter-unknown', 'csv(..a)', 'vanilla', '#let a = arguments(bytes("a,b"), zeta: 8, alpha: 1).filter(v => true)\n');
add('sink-unknown', 'f(bytes("a,b"), zeta: 4, 8)', 'vanilla', '#let f(..a) = csv(..a)\n');
add('spread-array-excess', 'csv(..(bytes("a,b"), 8))');
add('spread-dict-unknown', 'csv(bytes("a,b"), ..(zeta: 1, alpha: 2))');
add('duplicate-option-with', 'f(delimiter: ",", zeta: 8)', 'vanilla', '#let f = csv.with(bytes("a,b"), delimiter: "xx")\n');
add('duplicate-option-spread', 'csv(..a, delimiter: ",", zeta: 8)', 'vanilla', '#let a = arguments(bytes("a,b"), delimiter: "xx")\n');
add('map-delimiter-detached', 'csv(..a)', 'vanilla', '#let a = arguments(bytes("a,b"), delimiter: "xx", zeta: 8).map(v => v)\n');
add('map-path-with-excess', 'csv(..a)', 'vanilla', '#let a = arguments("good.csv", 8).map(v => v)\n');
add('default-bytes', 'csv(bytes("a,b\\n1,2"))');
add('default-empty', 'csv(bytes(""))');
add('dictionary-value', 'csv(bytes("a;b\\n1;2"), delimiter: ";", row-type: dictionary)');
add('override-valid', 'f(delimiter: ";")', 'vanilla', '#let f = csv.with(bytes("a;b"), delimiter: ",")\n');
add('valid-path', 'csv("good.csv")');
add('parse-bytes', 'csv(bytes("a,b\\n1"))');
add('parse-binary-path', 'csv("invalid.csv")');
add('parse-text-path-preserve', 'csv("short.csv")', 'baseline');
add('missing-path-preserve', 'csv("missing.csv")', 'baseline');
add('map-path-preserve', 'csv(..a)', 'baseline', '#let a = arguments("good.csv").map(v => v)\n');
add('source-symbol-preserve', 'csv(sym.alpha)', 'baseline');
add('delimiter-symbol-preserve', 'csv(bytes("a,b"), delimiter: sym.comma)', 'baseline');
add('read-control', 'read(42, zeta: 1)', 'baseline');
add('json-control', 'json(bytes("{}"), zeta: 1)', 'baseline');
add('yaml-control', 'yaml(bytes("a: 1"), 8)', 'baseline');
add('toml-control', 'toml(bytes("a=1"))', 'baseline');
add('cbor-control', 'cbor(cbor.encode((a: 1)))', 'baseline');
add('xml-control', 'xml(bytes("<a/>"))', 'baseline');
add('json-encoder-control', 'json.encode((a: 1), pretty: false)', 'baseline');
function git(args) { return cp.execFileSync('git', args, {cwd:root,encoding:'utf8'}); }
function state() { return {utc:new Date().toISOString(),head:git(['rev-parse','HEAD']).trim(),diff_stat:git(['diff','HEAD','--stat'])}; }
function save(name,x) {fs.writeFileSync(path.join(out,name),JSON.stringify(x,null,2)+'\n');}
function run(bin,c) {
 const argv = ['--color=never','query',path.join(dir,c.id+'.typ'),'<result>','--field','value'];
 const started = new Date().toISOString();
 const r = cp.spawnSync(bin,argv,{cwd:dir,encoding:'utf8',timeout:20000});
 return {argv,cwd:dir,utc:started,status:r.status,signal:r.signal,stdout:r.stdout,stderr:r.stderr,error:r.error?String(r.error):null};
}
function observable(r) {return JSON.stringify([r.status,r.signal,r.stdout,r.stderr,r.error]);}
const mode = process.argv[2];
if(mode==='baseline') {
 fs.writeFileSync(path.join(dir,'good.csv'),'a,b\n1,2');
 fs.writeFileSync(path.join(dir,'short.csv'),'a,b\n1');
 fs.writeFileSync(path.join(dir,'invalid.csv'),Buffer.from([97,44,98,10,255]));
 for(const c of cases) fs.writeFileSync(path.join(dir,c.id+'.typ'),c.source);
 save('p1321-ab-cases.json',cases);
 const r = {schema:'p1321-ab-baseline/v1',before:state(),binaries:{baseline:{path:baseline,sha256:sha(fs.readFileSync(baseline))},vanilla:{path:vanilla,sha256:sha(fs.readFileSync(vanilla))}},cases:[]};
 for(const c of cases) r.cases.push({...c,baseline:run(baseline,c),vanilla:run(vanilla,c)});
 r.after=state(); save('p1321-ab-baseline-runs.json',r);
 for(const c of r.cases) console.log(c.id,c.policy,observable(c.baseline)===observable(c.vanilla)?'equal':'DIFF',c.baseline.stderr.split('\n')[0],'=>',c.vanilla.stderr.split('\n')[0]);
} else if(mode==='freeze') {
 const b=JSON.parse(fs.readFileSync(path.join(out,'p1321-ab-baseline-runs.json')));
 const expected=b.cases.map(c=>({id:c.id,policy:c.policy,observable:c[c.policy]}));
 save('p1321-ab-expectations.json',expected);
 const files=['p1321-ab-cases.json','p1321-ab-baseline-runs.json','p1321-ab-expectations.json','p1321-ab-runner.cjs'];
 const f={schema:'p1321-ab-freeze/v1',regime:'A/B sem atestação de isolamento',state:state(),prompt:{path:'00_nucleo/prompts/compiler/stdlib/loading.md',sha256:sha(fs.readFileSync(path.join(root,'00_nucleo/prompts/compiler/stdlib/loading.md')))},binaries:b.binaries,artifacts:Object.fromEntries(files.map(x=>[x,sha(fs.readFileSync(path.join(out,x)))])),fixtures:Object.fromEntries(fs.readdirSync(dir).map(x=>[x,sha(fs.readFileSync(path.join(dir,x)))])),budget:'One baseline full, up to two focal calibrations with gain; one candidate normal/repeat/reverse; no repetition of green corpus',unknown:'Required unknown blocks closure',limitations:['No technical access isolation attestation','Accidental p1320 before.diff output exposed P1319 baseline diff; no P1321 candidate code read'],allowed_reads:['L0','ADRs','vanilla source','binary outputs','git HEAD/status/diffstat'],write_scope:['00_nucleo/diagnosticos/p1321-ab-*',dir]};
 save('p1321-ab-freeze.json',f);console.log(JSON.stringify(f,null,2));
} else if(mode==='candidate') {
 const f=JSON.parse(fs.readFileSync(path.join(out,'p1321-ab-freeze.json')));
 for(const [p,h] of Object.entries(f.artifacts)) if(sha(fs.readFileSync(path.join(out,p)))!==h) throw Error('changed artifact '+p);
 for(const [p,h] of Object.entries(f.fixtures)) if(sha(fs.readFileSync(path.join(dir,p)))!==h) throw Error('changed fixture '+p);
 const ex=JSON.parse(fs.readFileSync(path.join(out,'p1321-ab-expectations.json')));
 const bin=process.argv[3];
 const r={schema:'p1321-ab-candidate/v1',freeze_sha256:sha(fs.readFileSync(path.join(out,'p1321-ab-freeze.json'))),before:state(),binary:{path:bin,sha256:sha(fs.readFileSync(bin))},runs:[]};
 for(const order of ['normal','repeat','reverse']) {
  for(const c of order==='reverse'?[...cases].reverse():cases) {
   const actual=run(bin,c); const e=ex.find(x=>x.id===c.id);
   r.runs.push({id:c.id,order,policy:e.policy,actual,match:observable(actual)===observable(e.observable)});
  }
 }
 r.after=state();save('p1321-ab-candidate-runs.json',r);
 console.log(JSON.stringify({runs:r.runs.length,failures:r.runs.filter(x=>!x.match)},null,2));
}
