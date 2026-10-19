import subprocess,sys
commands=[['cargo','fmt','--all'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v15,v26','--fail-on','warning'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','--fix-hashes','.'],['/tmp/tekt-linter-none-target/debug/crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning']]
for command in commands:
 print(command,flush=True)
 r=subprocess.run(command)
 if r.returncode:sys.exit(r.returncode)
