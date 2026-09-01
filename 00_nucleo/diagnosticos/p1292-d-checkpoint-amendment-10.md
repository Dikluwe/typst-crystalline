# P1292 — amendment-10: checkpoint atômico e regressão de clearance

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`1f606660721641d536c26c063377963164ad77f0cbb15880a77f98865955057f`

**Estado:** decisão interna em fase paginada existente; nenhum novo gate
humano. Contrato pronto para resselo e entrega causal.

## Proveniência

- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- working tree não commitada;
- medição/decisão em `2026-09-01T09:44:29-03:00`;
- `git diff HEAD --stat`: 44 arquivos, 2.428 inserções e 450 remoções;
- vanilla ratificado `/usr/local/bin/typst` SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- candidato `target/debug/typst` SHA-256
  `fe6b740b1f044bccc9cdca651a4cc971349bef90dce11626987fe5a916fbfd8f`.

Os PDFs foram compilados bilateralmente por CLI e lidos com `pdfinfo` e
`pdftotext -bbox`. Nenhum teste protegido ou código candidato foi lido para
adaptar o contrato.

## Refutação v10 e reprodução independente

O resultado protegido registrou 11 testes, 10 GREEN e um D RED: três páginas
e anchor FLOAT_BEFORE `17.404` já estavam corretos, mas AFTER_MARKER ficou p2
em vez de p3; Unknown=0. O finding atribui o escape a um Place não-float já
anexado a `current_items` antes da migração de `current_line`.

Uma fixture independente reproduziu a forma causal: página 100pt × 100pt,
PREFLOW, bottom float-prefixo de 80pt, Flush, Block posterior com texto
`AFTER_FLOW` e Place não-float `AFTER_MARKER`, seguido de FLOAT_AFTER. Ambos
os renderers produziram três páginas. No candidato refutado, AFTER_MARKER
ficou p2 (`yMin=-9.834`) enquanto AFTER_FLOW foi p3 (`-2.596`); no vanilla,
AFTER_MARKER e AFTER_FLOW ficaram p3 (`-2.596`), exatamente uma vez. O float
posterior também permaneceu p3.

**Classificação:** mover somente a linha é insuficiente. A unidade observável
é todo efeito regional posterior à fronteira, inclusive items já anexados por
variants descendentes. A hipótese Block-specific é refutada porque o escape é
um efeito de Place e a decisão de fitting pertence à transação regional.

## Decisão owner-correct

`compiler/layout.md`, owner de `Layouter`, define um checkpoint transacional
do sufixo. Depois que prefix floats e reservas estão estabilizados, estabelece
uma fronteira antes do flow posterior e captura cursor/região, buffers e
métricas de linha, cauda de `current_items`, cauda de items/frame ativo,
geometria/extensões pendentes, fronteiras deferred/floats do sufixo e ponto de
replay por ocorrência. Qualquer buffer regional futuro que possa tornar o
sufixo visível integra a mesma transação.

Se a unidade cabe, a cauda é commitada. Se a região efetiva rejeita, rollback
remove atomicamente toda a cauda, restaura o checkpoint, avança normalmente e
reexecuta as ocorrências na nova região. Prefixo realizado, reservas e items
anteriores permanecem imutáveis. Suffix float não é antecipado; cada efeito é
confirmado uma vez.

`compiler/layout/cursor.md` possui commit/rollback/replay/avanço e não pode
migrar `current_line` separadamente de `current_items`. Ele não inspeciona o
próximo Content nem especializa Block. `compiler/layout/place.md` e
`compiler/layout/flush.md` já fixam, respectivamente, inserções/reservas e a
política da sentinela; foram auditados e permanecem inalterados neste
amendment.

Não há nova fase, passagem, API, default ou compatibilidade. Refutam o
contrato: snapshot parcial; rollback do prefixo; duplicação; varredura futura;
Block especial; item posterior preso na região rejeitada; ou replay de float
posterior como prefixo.

## P245: anchor separado de reserva/fitting

O owner test-only `compiler/layout/tests.md` continha obrigação herdada cujo
teste P245 esperava que bottom clearance deslocasse o frame para cima. Dois
controles independentes refutaram essa leitura:

| Controle | `clearance: 0pt` | `clearance: 20pt` | Resultado bilateral |
|---|---|---|---|
| bottom float isolado | 1 página; ANCHOR `yMin=90.166` | 1 página; ANCHOR `yMin=90.166` | anchor físico invariável |
| 80pt flow antes do float | 1 página; ADJACENT `-2.596`, FLOAT `90.166` | 2 páginas; ADJACENT p1, FLOAT p2 `90.166` | clearance altera fitting/reserva, não anchor |

A obrigação nova substitui a expectativa refutada por dois testes separados:
anchor invariável e fitting adjacente distinto. Um terceiro teste cobre o
checkpoint, exigindo PREFLOW p1, FLOAT_BEFORE p2 e AFTER_MARKER + AFTER_FLOW +
FLOAT_AFTER p3, todos uma vez. O autor de contrato não edita `tests.rs`; o
testador independente materializa o delta somente depois do resselo.

## L0s e ownership

```text
compiler/layout.md
e3d1b88ced9ed38ebdb7f7187b1feb8374ccf6548f7f351b9e8820bd4de25cea

compiler/layout/cursor.md
a51acc36d05328576e020543f98cd20f72054fa25b786603a7cda646ebca9ebf

compiler/layout/tests.md
35f8db27e3594d5089099e72e6b3cac031da2926e7d48d68eb4c2bdb51bd32a6

compiler/layout/place.md (auditado, inalterado)
51b9fbd113f919bc7d3008b6abca4590eb985b1d91ae51075833ac0cd8ae4f97

compiler/layout/flush.md (auditado, inalterado)
c105f59a3e6f3013ba76db514ec982c25826c8cd64f2b0e8b13a04815899ca1a
```

`compiler/layout/tests.md` entra no grafo canônico como owner do consumer
test-only existente; o set passa de 25 para 26 L0s. `Hash do Código` não foi
artificialmente atualizado e `--fix-hashes` não foi usado.

## Hashes das fixtures independentes

```text
p1292-a10-after-place.typ      a4ebd412282e12385cd14ee3acc60c990e01f327196b51cba7ca69b11c56d4e2
p1292-a10-anchor-0.typ         a71a1aad63e3004b18a21acb17ac2a71deffb47459e001c433a98ced80913c54
p1292-a10-anchor-20.typ        ae683ad32477fa524f94f9864a3a4bdca43879ef581a12b389b7bf740972ab39
p1292-a10-reserve-after-0.typ  86ac5e7e357f45bcbfd70f5945935ddeabe167514da7972ff9dcacd45501e2d2
p1292-a10-reserve-after-20.typ 47279b314d94aedf6ea77e404633375ed46cc89542e521a1d9984154efeb9b5d
```

Contrato/receipt/gates finais são registrados após o resselo. **PARAGEM.**
