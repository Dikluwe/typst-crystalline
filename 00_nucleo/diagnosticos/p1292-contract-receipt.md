# P1292 — recibo do contrato canônico v14

**Fase:** amendment-13, resselo exclusivamente de whitespace após normalização
pre-commit

**Papel:** autor contratual segregado; sem escrita em produto, testes, oracle,
ataques ou veredito

**Manifest corrente:**
`c3dbc43554474f81a8ce6b9b92557470eeb34cc76b33f6c1509f9e4195abe31f`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Proveniência:** working tree não commitada; auditoria iniciada em
`2026-09-01T13:04:46-03:00`; seal produzido em
`2026-09-01T13:06:28-03:00`.

Não há novo gate humano. A única entrada L0 alterada perdeu uma linha vazia no
EOF; não houve delta normativo, produtivo, público, de default, fase ou
compatibilidade. ADR-0127 permite fluxo contínuo.

## Bridge v13 → v14

- canônico v13:
  `61387d1be09b46740094112dbc2adea4f1df1491b4ca18fab1d4b90f9fc60010`;
- seal v13:
  `bff65c3e83f5dc84bcc8eaad03eb4463e7f0dd2bef3a1f194470f51a56496db4`;
- receipt v13:
  `7e780f05710975df53a9650c8940f008e91f11b73fa0a28475a74af17167d799`;
- lots v13:
  `edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf`;
- comparison v13:
  `71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46`.

## Prova de ausência de delta normativo

Dos 27 L0s, 26 coincidiram exatamente com v13. A única divergência foi:

```text
entities/elements/flush.md
v13:   7e1290bd948e4270bc380bf13fbf209819a679cfa87ad7d58280814e83cb9319
atual: 75967e0915aa211ef297c7f88059c3ce675c892de450930137760605df223c37
```

O arquivo atual contém 1.867 bytes e termina com uma única `LF`. Acrescentar
somente outra `LF` produz 1.868 bytes e exatamente o SHA v13
`7e1290bd...`. Assim, todos os bytes atuais e o texto normativo integral são
idênticos; o pre-commit removeu apenas a linha vazia adicional no EOF.

P1030, oracle, mutações, produto e expectativas públicas permanecem fechados e
inalterados.

A primeira tentativa de gate V5 rejeitou corretamente o header produtivo
antigo `7e1290bd`. O coordenador corrigiu exclusivamente
`01_core/src/entities/elements/flush.rs` para `@prompt-hash 75967e09`; SHA-256
bruto final do consumer:
`0208c6b3d86f3f64b7c84051dead1e5487b8d02ccf92dbf902257cafccc3cad1`.
O autor contratual não editou produto.

## Artefatos v14

- contrato canônico:
  `0715876251f1ae4c19b5bbd881b4f8af3f331378d3f83ca573d56ec7f8fd46df`;
- arquivo seal:
  `fb7e5271f68ba992cd358384bd39ac827e30c1e438500d721b8d77fa2f1aca6d`;
- amendment-13:
  `a5787306b4a13fb99f60d2c0c63fb54164ef1713ff1c9c5237aec2c7c30619a0`;
- lots preservado exatamente:
  `edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf`;
- comparison preservado exatamente:
  `71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46`.

## Gates v14

```text
canonical JSON + seal metadata
PASS — `0715876251f1ae4c19b5bbd881b4f8af3f331378d3f83ca573d56ec7f8fd46df`

27/27 hashes L0
PASS

crystalline-lint . --checks v5,v15,v26 --fail-on warning
PASS — No violations found

git diff --check e whitespace em arquivos tracked/untracked P1292
PASS — diff relevante vazio; cinco arquivos v14 sem trailing whitespace,
sem EOF ausente e sem linha vazia adicional no EOF
```

O regime permanece segregado por papel/capacidade, sem alegação de isolamento
técnico no filesystem compartilhado. **PARAGEM após os gates.**

---

## Histórico — recibo canônico v13

**Fase:** amendment-12, resselo exclusivamente de linhagem após
`crystalline-lint --fix-hashes .`

**Papel:** autor contratual segregado; sem escrita em produto, testes, oracle,
ataques ou veredito

**Manifest corrente:**
`1d9841747760d1dcb48522fe3108a4670c8096cb64aad436fe877e71e2f7c044`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Proveniência:** working tree não commitada; auditoria iniciada em
`2026-09-01T12:37:11-03:00`; seal produzido em
`2026-09-01T12:40:02-03:00`.

Não há novo gate humano. O delta comprovado é exclusivamente metadata
`Hash do Código` em dois L0s, mais uma correção anterior de observador
test-only P1030. Nenhuma obrigação, API, default, fase ou compatibilidade
mudou; ADR-0127 permite fluxo contínuo.

## Bridge v12 → v13

- canônico v12:
  `493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef`;
- seal v12:
  `d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`;
- receipt v12:
  `8ea9dfe983d00c6939d29736443b3cdbaacb7f9c970e46fe62b9a85daa2009a8`;
- lots v12:
  `edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf`;
- comparison v12:
  `71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46`.

## Prova de ausência de delta normativo

O seal v12 divergiu em exatamente dois dos 27 L0s. A reconstrução substituiu
somente a primeira linha de metadata atual:

```text
compiler/math/layout/cancel.md
81399128 → b804d54b
SHA reconstruído: 84b2c85216b568155318284d6a4579a8c13f232ad32dd9472e2207a0ef394ffd

entities/elements/math_cancel.md
2bb6fdd1 → e3993092
SHA reconstruído: cc164630f81c98c08951bae556cf9d12dbe094649f0e5ecb645bb4cbb825e3cc
```

Ambos os resultados são exatamente os hashes protegidos pelo v12. Todos os
bytes normativos restantes são idênticos. Os hashes atuais ressellados são:

```text
9bf37e4d2f8a5e6517564ace60c99cb76a9899e12399812d1c2e955b148b82cd  compiler/math/layout/cancel.md
320008c4e4915de15c8e85338943ea6383d833bb8d09f5cb8083ac94dd6a0d38  entities/elements/math_cancel.md
```

## Correção test-only P1030 preservada

O helper legado observava `MathMatrix`, mas não o `MathVec` owner-correct já
materializado. A correção apenas em `compiler/eval/tests.rs` distinguiu os dois
variants sem relaxar expectativas ou alterar produto:

- tests.rs SHA-256:
  `7e5c362b1854214aaedb90e1e01949a78f812164da8fcf9f6a2cdcaa4b052017`;
- receipt:
  `edabc574c2b8748e7b114230348aac3312b4acb66e70e5c587b04b6ca2ca5a61`;
- P1030 7/7 GREEN; P1292 core 17/17 GREEN; workspace GREEN;
- oracle protegido 11/11 e byte-idêntico
  `fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`;
- `Unknown=0`.

P1030 fecha causalidade de verificação, mas não altera lots, comparison ou
qualquer expectativa pública do contrato.

## Artefatos v13

- contrato canônico:
  `61387d1be09b46740094112dbc2adea4f1df1491b4ca18fab1d4b90f9fc60010`;
- arquivo seal:
  `bff65c3e83f5dc84bcc8eaad03eb4463e7f0dd2bef3a1f194470f51a56496db4`;
- amendment-12:
  `6a18472b8423e0cc7d1cedc219420972d947b1d37347e72e4170afcb890c70f9`;
- lots preservado exatamente:
  `edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf`;
- comparison preservado exatamente:
  `71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46`.

## Gates v13

```text
canonical JSON + seal metadata
PASS — `61387d1be09b46740094112dbc2adea4f1df1491b4ca18fab1d4b90f9fc60010`

27/27 hashes L0
PASS

crystalline-lint . --checks v5,v15,v26 --fail-on warning
PASS — No violations found

git diff --check e whitespace
PASS — saída vazia
```

O regime permanece segregado por papel/capacidade, sem alegação de isolamento
técnico no filesystem compartilhado. **PARAGEM após os gates.**

---

## Histórico — recibo canônico v12

**Fase:** amendment-11, flow ordinário sem Flush e correção de ownership
Block/Place

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado/final:**
`d4b5908c44ac4ef536e474efb67969e89ae52ed6a2431ea86614060c5c032839`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Proveniência:** working tree não commitada; medição em
`2026-09-01T10:52:16-03:00`; fechamento em
`2026-09-01T11:00:39-03:00`; `git diff HEAD --stat` registrou 45 arquivos,
3.053 inserções e 497 remoções antes dos artefatos documentais v12.

Não há novo gate humano. O amendment corrige fórmulas internas de paridade e
realinha ownership dentro da fase de layout existente, sem assinatura pública,
default de produto, fase ou quebra de compatibilidade. Segue fluxo contínuo
ADR-0127. A confirmação anterior do default relativo `1.5em` permanece
vigente e inalterada.

## Bridge v11 → v12

- canônico v11:
  `671fd53142624d9cceaa28d4ebaf5676eb07f4bfa5d3432d1fd2e030ad29d9f2`;
- seal v11:
  `256f5cbee59ae11c7bf2a1150865a95694753a1dfd24197de837a4f464f52daa`;
- receipt v11:
  `4cd26f7bd20a4e343f4f59231584454202975d0da9fea9c7eb85a173af7e6148`;
- lots v11:
  `867b5fe0e61e3bb69a4e54aec4a98710c4acb12c2de03b2f5098d9c890aa79e0`;
- comparison v11:
  `cbcac2aa1efe454a20e9542515211f7fabba4048ee2824cec3d2c2ff37207c01`.

A–C, superfície, defaults, erros e o vetor transacional D permanecem. O v12
refina D somente para congelar os controles ordinários sem marker/float e
atribuir as fórmulas aos owners corretos.

## Artefatos v12

- contrato canônico:
  `493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef`;
- arquivo seal:
  `d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`;
- lots:
  `edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf`;
- comparison:
  `71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46`;
- amendment-11:
  `7f5ce09fbaf6ac871d1e4fb4acab4bf60ffd2722cfeb21f3da2a600d337d72cf`.

## Medição e owner

Dois controles bilaterais, um sem Flush e outro sem float nem Flush,
coincidiram dentro de cada renderer:

```text
candidato: AFTER_FLOW 27.404pt; AFTER_MARKER -9.834pt
vanilla:   AFTER_FLOW 40.604pt; AFTER_MARKER 47.842pt
```

O delta `13.2pt` de AFTER_FLOW é o default Block `1.2em` a 11pt, comprovado
em `layout/container.rs:347-360` e `flow/collect.rs:232-278`. Place não-float
no upstream usa o offset já avançado pelo frame in-flow em
`flow/distribute.rs:504-510,630-662`.

Portanto `compiler/layout/block.md` entra no grafo como owner 1:1 de
`layout/block.rs`; `compiler/layout/place.md` mantém a fórmula do Place
não-float; `compiler/layout.md` e `cursor.md` ficam limitados à mecânica
transacional e proíbem tradução global/hardcode. `flush.md` foi auditado e
permaneceu inalterado.

## L0s D v12

```text
compiler/layout/block.md
4d0158cdf9b53f080373457209c23162ef6f09df35cb0703519a2f156af01f02

compiler/layout.md
8b3248416ede53bcb836b219cab4de53129f597a154959a19c1d22d26a09e835

compiler/layout/cursor.md
c1fafbee057737ed44bdd7ac5bc2473537f8fa79aa47b76c662f0ff6a5806bd4

compiler/layout/place.md
53631639a6f417d449b9f2742fbbb75161ff9486481cef4e9344fdbf785814f6

compiler/layout/tests.md
deee32e8fadf311c9a8686004e27b098230b7ef348a4d8b09dd91f9f72d4f157

compiler/layout/flush.md (auditado, inalterado)
c105f59a3e6f3013ba76db514ec982c25826c8cd64f2b0e8b13a04815899ca1a
```

O set canônico passa de 26 para 27 L0s ao incluir o owner produtivo existente
de Block. `Hash do Código` não foi artificialmente atualizado e
`--fix-hashes` não foi usado.

## Aceitação e handoff

Os testes independentes devem tornar RED, separadamente:

1. Block→flow sem float/Flush:
   `AFTER_FLOW=40.604±0.002pt`, `AFTER_MARKER=47.842±0.002pt`;
2. o mesmo com float anterior e sem Flush;
3. variação relativa/absoluta de `block.spacing` ou `below`;
4. preservação do vetor transacional p3:
   `AFTER_FLOW=-2.596±0.002pt`, `AFTER_MARKER=4.642±0.002pt`.

O implementador recebe o seal somente depois do RED. A implementação pertence
a Block/Place; Cursor pode remover a compensação global e conservar apenas a
transação. Flush, testes protegidos, A–C, defaults e API permanecem congelados.

## Gates do contrato

```text
python3 -m json.tool p1292-manifest.json e p1292-contract-seal.json
PASS

validação canônica + 27 hashes L0
PASS 493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef

crystalline-lint . --checks v15,v26 --fail-on warning
PASS — No violations found

git diff --check
PASS — saída vazia

rg -n '[ \\t]+$' <L0s/artefatos v12>
PASS — saída vazia
```

O autor não leu/editou produto, testes ou oracle protegido. O filesystem é
compartilhado; o regime é segregado por papel/capacidade, sem alegação de
isolamento técnico. **PARAGEM após validação.**

---

## Histórico — recibo canônico v11

**Fase:** amendment-10, checkpoint atômico do sufixo e refutação da
expectativa P245 sobre clearance bottom

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`1f606660721641d536c26c063377963164ad77f0cbb15880a77f98865955057f`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Proveniência:** working tree não commitada; medição/decisão em
`2026-09-01T09:44:29-03:00`; fechamento em
`2026-09-01T09:47:14-03:00`; `git diff HEAD --stat` registrou 44 arquivos,
2.428 inserções e 450 remoções.

Não há novo gate humano: checkpoint interno e correção de expectativa de teste
refutada seguem fluxo contínuo ADR-0127. O gate do default `1.5em` permanece
vigente e inalterado.

## Bridge v10 → v11

- canônico v10:
  `3b1ead87735df2e4e7c72a84c899a45c61f45067bf75f808812e9a4f9056bd69`;
- seal v10:
  `f798bfc18ebdcd4b9bcc684f3b95d398fbb37b072f6e8ea89e0fb7676647d630`;
- receipt v10:
  `b79af4ec4c9a5bdc59a34b28168dfb5f78c93666f8b0cbc54557bc44df96baa7`;
- lots v10:
  `e096a694f5c6fad1fb5623ec2e3544ddc878fd407befde14bcf296459b87360f`;
- comparison v10:
  `513120408189be25876812533c2860a262edc9fd3e4f00c7db67add2464d0875`.

Superfície, defaults, erros, A-C e os resultados D anteriores permanecem. O
v11 refina D para exigir atomicidade de todos os efeitos pós-marker e separa
anchor físico de reserva/fitting nos controles de clearance.

## Artefatos v11

- contrato canônico:
  `671fd53142624d9cceaa28d4ebaf5676eb07f4bfa5d3432d1fd2e030ad29d9f2`;
- arquivo seal:
  `256f5cbee59ae11c7bf2a1150865a95694753a1dfd24197de837a4f464f52daa`;
- lots:
  `867b5fe0e61e3bb69a4e54aec4a98710c4acb12c2de03b2f5098d9c890aa79e0`;
- comparison:
  `cbcac2aa1efe454a20e9542515211f7fabba4048ee2824cec3d2c2ff37207c01`;
- amendment-10:
  `8aac16ad279e22495ed3ce805462a2ed36a44d30cef6df4b62a16b85f52c512f`.

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

O set canônico passa de 25 para 26 L0s ao incluir o owner test-only existente
`compiler/layout/tests.md`. Os 26 hashes conferem contra os bytes atuais.
`Hash do Código` não foi artificialmente atualizado e `--fix-hashes` não foi
usado.

## Mecanismo selado

O resultado v10 protegido tinha três páginas e anchor do prefixo corretos, mas
deixava AFTER_MARKER p2 enquanto o flow correspondente migrava a p3. A
reprodução independente mostrou a mesma cisão: Place não-float já anexado a
`current_items` escapava da migração isolada de `current_line`.

O Layouter agora possui contratualmente uma transação sobre todo o sufixo
pós-marker: cursor/região, linha e métricas, cauda de `current_items`, cauda do
frame/items ativo, geometria/deferred/floats do sufixo e ponto de replay por
ocorrência. Cursor commita a cauda inteira ou faz rollback atômico, avança e
reexecuta. Prefixo realizado, reservas e conteúdo anterior ficam fora do
rollback. Não há inspeção futura, Block especial, duplicação ou fase nova.

O caso focal exige PREFLOW p1, FLOAT_BEFORE p2, e AFTER_MARKER + AFTER_FLOW +
FLOAT_AFTER p3, todos exatamente uma vez.

## Obrigação test-only corrigida

P245 não pode exigir que bottom clearance mova o frame. Medição bilateral em
página 100pt mostrou ANCHOR `yMin=90.166` com `0pt` e `20pt`. Um controle
separado com 80pt de flow provou fitting: `0pt` mantém float p1; `20pt` move o
float para p2, ainda ancorado em `90.166`.

O testador independente deve substituir apenas a expectativa refutada por:

1. teste de anchor físico invariável;
2. teste separado de reserva/fitting;
3. regressão do checkpoint atômico com Place não-float pós-marker.

O autor de contrato não editou `tests.rs` ou oráculo protegido.

## Gates

```text
sha256sum 00_nucleo/diagnosticos/p1292-manifest.json
1f606660721641d536c26c063377963164ad77f0cbb15880a77f98865955057f

python3 -m json.tool 00_nucleo/diagnosticos/p1292-contract-seal.json
exit 0

validação canônico + 26 hashes L0
PASS 671fd53142624d9cceaa28d4ebaf5676eb07f4bfa5d3432d1fd2e030ad29d9f2

crystalline-lint . --checks v15,v26 --fail-on warning
✓ No violations found

git diff --check
exit 0

rg -n '[ \\t]+$' <três-L0s> <amendment-10> <seal> <receipt>
saída vazia
```

Foram escritos somente os três L0s autorizados, amendment-10, seal e receipt.
Nenhum produto, teste, oráculo, ataque ou veredito foi lido para adaptação ou
editado. O implementador e o testador recebem obrigações segregadas do v11.
**PARAGEM.**
