from provenance_common import *
x=state(__file__);before=json.load(gzip.open(D/'before.json.gz','rt'));cutoff=int(datetime.datetime.fromisoformat(before['utc']).timestamp()*1e9)
paths=set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines())
roots=[Path(p)for p in ['/usr/share/fonts','/usr/local/share/fonts',str(Path.home()/'.fonts'),str(Path.home()/'.local/share/fonts'),'/usr/share/texmf/fonts',str(FONT)]]
for root in roots:
 if root.exists():paths.update(str(p)for p in root.rglob('*')if p.is_file()and p.suffix.lower()in ['.ttf','.otf','.ttc','.otc'])
configs=set(str(p)for p in Path('/etc/fonts').rglob('*')if p.is_file()and p.suffix=='.conf')
for p in [Path.home()/'.fonts.conf',Path.home()/'.config/fontconfig/fonts.conf']:
 if p.is_file():configs.add(str(p))
x['fonts_and_configs']={};recent=[]
for p in sorted(paths|configs):
 path=Path(p)
 if not path.is_file():continue
 st=path.stat();lst=path.lstat();h=sha(path);after=path.stat();assert(st.st_ino,st.st_size,st.st_mtime_ns,st.st_ctime_ns)==(after.st_ino,after.st_size,after.st_mtime_ns,after.st_ctime_ns)
 v={'sha256':h,'dev':st.st_dev,'ino':st.st_ino,'size':st.st_size,'mtime_ns':st.st_mtime_ns,'ctime_ns':st.st_ctime_ns,'symlink':path.is_symlink(),'link_ctime_ns':lst.st_ctime_ns,'resolved':str(path.resolve()),'kind':'config'if p in configs else'font'};x['fonts_and_configs'][p]=v
 if max(st.st_mtime_ns,st.st_ctime_ns,lst.st_ctime_ns)>cutoff:recent.append(p)
x['relevant_environment']={k:os.environ.get(k)for k in ['HOME','XDG_DATA_HOME','XDG_CONFIG_HOME','FONTCONFIG_FILE','FONTCONFIG_PATH']};x['roots']=[str(p)for p in roots];x['measurement_start_utc']=before['utc'];x['recent_paths']=recent
prior_file=R/'00_nucleo/diagnosticos/face-parse-reuse-2026-10-10/final.json.gz';prior=json.load(gzip.open(prior_file,'rt'));x['prior_receipt_sha256']=sha(prior_file);prior_fonts=prior['fonts'];intersect={str(Path(p).resolve()):h for p,h in prior_fonts.items()};compared=0
for p,v in x['fonts_and_configs'].items():
 if v['resolved']in intersect:assert v['sha256']==intersect[v['resolved']],p;compared+=1
x['prior_assets_compared']=compared;x['stable_guard']=not recent;assert not recent,recent;save('font-provenance.json.gz',x);print('assets',len(x['fonts_and_configs']),'prior matches',compared,'all timestamps precede measurement')
