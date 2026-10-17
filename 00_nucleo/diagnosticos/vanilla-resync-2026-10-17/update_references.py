from pathlib import Path
import json
out=Path(__file__).resolve().parent
receipt=json.loads((out/'oracle-build.json').read_text())
assert receipt['sys_version']=='"version(0, 15, 1)"',receipt['sys_version']
sha=receipt['binary_sha256'];old='9f2b6e8715237cb086899a42873660fe744622e8';new=receipt['pin']
p=Path('CLAUDE.md');s=p.read_text();start=s.index('## Referência de paridade —');end=s.index('\n---',start)
s=s[:start]+f'''## Referência de paridade — qual binário, qual fonte (novo pin 2026-10-17)

O alvo atual da busca de paridade é o vanilla upstream/main
**`{new}`**, hash **pinado**, árvore
`{receipt['upstream_tree']}`. O re-sync explícito autorizado em 2026-10-17
substitui o pin P1489 `{old}`.
Uma atualização futura exige novo passo explícito. Não usar `main` móvel ou
uma tag/versão textual como alvo. Medições, prompts e relatórios feitos contra
`9f2b6e871`, `586e1bd43` ou pins anteriores continuam históricos e válidos apenas
para aqueles snapshots; não certificam o alvo atual sem nova execução.
O pin `a51e02804` permanece histórico na retificação P990-P992.
A paridade segue ADR-0107/0108: semântica, sintaxe e morfologia medidas antes
da decisão; defeitos do vanilla não são copiados como requisito do cristalino.

Quatro armadilhas de proveniência:

1. **A string de versão só é evidência auxiliar.** `lab/typst-original` não tem
   `.git` próprio. O build ratificado recebeu `TYPST_COMMIT_SHA={new}`
   explicitamente, com Rust 1.92.0, lockfile, um job, teto de 12 GiB e swap zero.
   Sua fonte foi comparada por blob com a árvore acima, preservando somente o
   lockfile e o extrator local como overlay. O executável
   `lab/typst-original/target/release/typst` tem SHA-256
   `{sha}`. A receita, a verificação de fonte/árvore,
   recursos e recibo completo estão em
   `00_nucleo/diagnosticos/vanilla-resync-2026-10-17/relatorio.md` e seus anexos.
   O binário P1489 foi preservado localmente como
   `lab/typst-original/target/release/typst-9f2b6e871-p1489`;
   seus relatórios conservam o SHA original. A identidade exige commit, árvore,
   receita e SHA-256, nunca só `--version`.
2. **`./target/release/typst` na raiz é o cristalino.** `04_wiring` declara
   `[[bin]] name = "typst"`; a versão pública vem de `PARITY_VERSION`
   (`entities/version.rs`) e do nosso HEAD. Confundir os dois foi o erro P1024.
   O oráculo é o build comprovado do lab, ou o checkout independente do CI com
   recibo verificado. `/usr/local/bin/typst` só pode substituí-lo após prova de
   identidade de bytes com o build ratificado vigente.
3. **`sys.version` é superfície de linguagem.** A sonda direta do novo vanilla
   `bd5ebec2` devolveu `version(0, 15, 1)`. O cristalino conserva esse triplo;
   `PARITY_VERSION` não muda neste re-sync. Uma futura mudança exige o gate
   ADR-0127 em `00_nucleo/prompts/entities/version.md`.
4. **`--version` do cristalino pode ficar preso a um HEAD antigo.** Antes do P1033,
   `02_shell/build.rs` só declarava `rerun-if-env-changed=TYPST_COMMIT_SHA`.
   Resolvido em P1033 com `cargo:rerun-if-changed=../.git/HEAD` e
   `../.git/refs`. A proveniência continua a registrar HEAD e estado da árvore;
   uma string de versão isolada não prova a fonte usada na medição.

Nunca citar "0.15.0"/"0.15.1" como alvo sem confirmar o hash pinado primeiro.
''' +s[end:];p.write_text(s)
p=Path('README.md');p.write_text(p.read_text().replace(old,new))
p=Path('lab/parity/matrix/README.md');s=p.read_text().replace(old,new,1)
s=s.replace('As medições P1487 com `586e1bd43` permanecem históricas;', 'O recibo vigente está em `00_nucleo/diagnosticos/vanilla-resync-2026-10-17/`. As medições anteriores com `9f2b6e871` ou `586e1bd43` permanecem históricas;')
p.write_text(s)
p=Path('tools/ci/README.md');s=p.read_text().replace(old,new).replace('Para o binário local ratificado em P1489:', 'Para o binário local ratificado no re-sync de 2026-10-17:').replace('7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea',sha).replace('Compila com\nlockfile e Rust do workspace,', 'Compila com\nlockfile, um job e Rust do workspace,').replace('local P1489;', 'local ratificado;')
s=s.replace('receita e SHA-256 do executável.', 'receita e SHA-256 do executável. O relatório local e o recibo completos estão em\n`00_nucleo/diagnosticos/vanilla-resync-2026-10-17/relatorio.md`.')
p.write_text(s)
for name in ['lab/parity/tools/p1495_symbols_probe.py','lab/parity/tools/p1497_language_probes.py']:
 p=Path(name);s=p.read_text().replace('7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea',sha).replace('O oráculo é o binário vanilla P1489 pinado.', 'O oráculo é o binário vanilla bd5ebec2 ratificado no re-sync 2026-10-17.')
 p.write_text(s)
print('Current reference documentation and probe SHA updated:',new,sha)
