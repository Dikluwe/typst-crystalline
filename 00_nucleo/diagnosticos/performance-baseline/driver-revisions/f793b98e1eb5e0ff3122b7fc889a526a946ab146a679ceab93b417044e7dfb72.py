"""Sequential admission of unchanged short targets and existing Criterion Content groups."""
import json, pathlib, re, sys, time
import run as c
SELF=pathlib.Path(__file__).resolve()
ORIGINAL=c.monitored

def monitored(cmd,label,*args,**kwargs):
    before=c.sha(SELF);error=None;result=None
    try:result=ORIGINAL(cmd,label,*args,**kwargs)
    except BaseException as caught:error=caught
    after=c.sha(SELF);receipt=c.OUT/'receipts'/(label+'.json')
    if receipt.exists():
        value=json.loads(receipt.read_text());value['additional_driver_input']={'path':str(SELF),'sha256_before':before,'sha256_after':after};value['valid']=value['valid'] and before==after
        receipt.write_text(json.dumps(value,indent=2)+'\n')
    if error:raise error
    if before!=after:raise RuntimeError('Diagnostic driver changed; block discarded')
    return result
c.monitored=monitored

def retry(fn):
    while True:
        try:fn();return
        except RuntimeError as error:
            if not any(x in str(error) for x in ['No stable/idle','Concurrent workload before','Invalid result; raw log retained: micro']):raise
            print('Isolation rejected; resume after 30 s:',error,flush=True);time.sleep(30)

# Ordering changes invocation scheduling only; all binaries, CLI options, fixtures
# and Criterion loops remain those of the recorded build and existing suite.
c.TARGETS=['observation_bench','stdlib_bench','scanner_bench','layout_bench','pdf_builder_bench','stream_bench']
retry(lambda:c.micro([1,2,3]))
artifacts=c.identity_check();plans={}
for target in ['pipeline_bench','content_bench']:
    exe=artifacts[target]['path']
    monitored([exe,'--bench','--list'],'build-inventory-'+target,False)
    value=(c.OUT/('logs/build-inventory-'+target+'.log')).read_text()
    cases=sorted(set(row.removesuffix(': benchmark').strip() for row in value.splitlines() if row.endswith(': benchmark')))
    assert cases,'Unexpected empty Criterion inventory: '+target
    groups=sorted(set('/'.join(x.split('/')[:2]) for x in cases))
    plan={'utc':c.utc(),'target':target,'executable_sha256':artifacts[target]['sha256'],'driver_sha256':c.sha(SELF),'cases':cases,'groups':groups}
    c.save(pathlib.Path(target.removesuffix('_bench')+'-filter-plan.json'),plan);plans[target]=plan

def filtered():
    c.quiet();c.identity_check()
    for target,plan in plans.items():
        exe=artifacts[target]['path']
        for run in [1,2,3]:
            for group in plan['groups']:
                slug=group.replace('/','_');label=f'micro-r{run}-{target}-{slug}'
                destination=c.OUT/'criterion'/f'r{run}'/(target+'_filtered')/slug
                if c.resume_or_retire(label,destination):continue
                destination.mkdir(parents=True,exist_ok=False)
                cmd=[exe,'--bench','--sample-size','30','--warm-up-time','0.5','--measurement-time','1.5','--nresamples','10000','--noplot','^'+re.escape(group)+r'(?:/|$)']
                monitored(cmd,label,True,{'CRITERION_HOME':str(destination)})
                c.identity_check()
retry(filtered)
print('All eight existing targets and every inventoried filtered case admitted in three rounds',flush=True)
