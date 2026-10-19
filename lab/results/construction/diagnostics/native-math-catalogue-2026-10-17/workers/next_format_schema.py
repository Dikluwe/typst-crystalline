from pathlib import Path
import json,gzip,subprocess,re,hashlib,datetime
out=Path(__file__).resolve().parents[1];pin='9f2b6e8715237cb086899a42873660fe744622e8';cases=[('svg','typst-svg/src/format.rs','SvgFormat'),('png','typst-render/src/format.rs','PngFormat'),('pdf','typst-pdf/src/format.rs','PdfFormat'),('html','typst-html/src/format.rs','HtmlFormat'),('bundle','typst-bundle/src/format.rs','BundleFormat'),('attach','typst-pdf/src/format.rs','AttachElem'),('artifact','typst-library/src/model/accessibility.rs','ArtifactElem'),('elem','typst-html/src/format.rs','HtmlElem'),('frame','typst-html/src/format.rs','FrameElem')];rows=[]
for name,part,struct in cases:
 path='crates/'+part;raw=subprocess.check_output(['git','show',pin+':'+path],text=True);start=raw.index('pub struct '+struct+' {');closing=raw.index('}',start);end=closing+1 if raw[raw.index('{',start)+1:closing].strip()=='' else raw.index('\n}',start)+2;attr=raw.rfind('#[elem',0,start);block=raw[start:end];fields=[];excluded=[];cursor=0
 for match in re.finditer(r'^    pub (\w+):',block,re.M):
  props=block[cursor:match.start()];(excluded if re.search(r'^    #\[(internal|external)\]',props,re.M) else fields).append(match[1].replace('_','-'));cursor=match.end()
 rows.append(dict(name=name,source=path,line=raw[:start].count('\n')+1,blob_sha256=hashlib.sha256(raw.encode()).hexdigest(),attributes=raw[attr:start].strip(),block=block,fields=fields,excluded=excluded,locatable='Locatable' in raw[attr:start]))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pin=pin,kind='Primary-source audit only; not an L0 or architecture/implementation decision',cases=rows)
with gzip.open(out/'next-format-pinned-schema.json.gz','wt') as f:json.dump(record,f)
for r in rows:print(r['name'],r['fields'],'excluded',r['excluded'],'locatable',r['locatable'])
