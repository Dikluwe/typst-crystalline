# Relatório — Passo 995: reconciliação `(n \ k)` secção 7 (medição fresca)

**Tipo**: verificação/reconciliação — sem fix (nenhuma regressão encontrada;
nenhum bisect necessário).
**Estado**: HEAD `f59278441` (confirmado por `git status`/`git log` antes de
começar — bate com a pré-condição do passo).

## Passo 1 — dados da medição paralela (fornecidos pelo dono)

- `.typ`: o canónico, na pasta `.typ` (o mesmo de P991) ✓.
- PDF medido: **o oráculo** (`--oracle-pdf`), "porque tem os operadores de
  PDF mais parecidos com o vanilla".
- Comando: não especificado (assumido `pdftotext -bbox`, conforme o passo).
- Commit/binário da medição paralela: não fornecido.

**Verificação da premissa "oráculo vs normal"**: compilei o canónico com
`--oracle-pdf` a partir do HEAD isolado — as coordenadas de `(n \ k)` são
**idênticas ao PDF normal ao 4º decimal** (n: 237.1975/243.7975; k:
237.5495/243.2805 em ambos). Para este construto, medir o oráculo ou o
normal é indiferente.

## Passo 2 — medição fresca com proveniência completa

- **Cristalino**: worktree isolado `/tmp/p995-cristalino` em `f59278441`,
  build release novo — sha256
  `fde659b81eef2b69a9b411bfa959def6a3d118c47431d02e1c7060f777f387bd`,
  string `typst 0.15.0 (f5927844)` (distintiva ✓, lição P934).
- **Vanilla**: `/usr/local/bin/typst`, sha256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  string `typst 0.15.1 (e0e8ca4d)` — **registado: parece 0.15.1 mas é
  upstream/main `a51e02804` (main+93) com hash do repo local**, baseline
  ratificado em `typst-retificacao-p990-p992-lab-sync.md` (equivalência a
  0.15.1 medida: delta zero no canónico).

**Medição** (`pdftotext -bbox`, secção 7, linha `(n \ k)`, PDF normal do
HEAD; idêntica no oráculo):

| elemento | HEAD fresco | P991 "depois" | P991 "antes" |
|---|---|---|---|
| n: folga esq/dir | **0.00 / 0.00pt** | 0.00 / 0.00pt | 0.00 / 0.00pt (flush-direita, bug) |
| k: folga esq/dir | **0.352 / 0.517pt** | 0.352 / 0.517pt | ≈0 dir (fundido com `⎠`) |

A medição fresca **bate com P991 ao milésimo** — a correcção está estável
através de P992/P993/P994. Nenhum passo posterior reintroduziu regressão.

## Passo 3 — interpretação: as duas medições não se contradizem

A "contradição" dissolve-se com a comparação fresca contra o vanilla
(repro mínimo `$ (n \ k) $` em `/tmp/nk-min.typ`, renders em
`temp/revisao/nk-min-{c,v}.png`):

1. **A alegação paralela é literalmente verdadeira no HEAD**: n está a
   0.00pt dos delimitadores (folga zero) — no output normal E no oráculo.
2. **A correcção de P991 também é verdadeira e estável**: o alinhamento
   das linhas está centrado (o vanilla centra as duas linhas uma sob a
   outra — centros medidos 39.31 = 39.31pt no mínimo), já não alternado.
3. **A divergência residual é OUTRA coisa, estrutural**: o vanilla trata
   `\` como **quebra de linha** — `(n \ k)` são duas linhas centradas com
   parênteses de **tamanho natural, um por linha, sem esticar** (render
   vanilla: `(` e `)` pequenos, com gap vertical entre linhas). O
   cristalino trata como **grelha delimitada de 1 coluna com parênteses
   esticados** a cobrir as duas linhas — e é o esticamento que deixa o n
   (elemento mais largo) encostado às peças do delimitador (folga zero),
   mesmo com a centragem correcta. P991 corrigiu o alinhamento (paridade
   real, verificada); o tratamento dos parênteses nunca esteve no seu
   âmbito.

**Conclusão da reconciliação**: cadeia de evidência fechada e consistente
— P991 (original) + reconstrução isolada (`b59db7f16`) + esta medição
fresca convergem ao milésimo; a investigação paralela mediu uma faceta
real e ainda divergente (a folga zero do n), que **não é regressão** —
é o tratamento de parênteses esticados vs. quebra de linha do vanilla.

## Achado para passo futuro (registado, não corrigido aqui)

**`(n \ k)`: quebra de linha vs. grelha delimitada esticada.** No vanilla,
`\` dentro de `( … )` quebra a linha ANTES do emparelhamento lr — os
parênteses ficam por emparelhar, tamanho natural, um por linha; as linhas
centram-se uma sob a outra. O cristalino emparelha primeiro e estica os
delimitadores sobre a grelha de 1 coluna. Medição: vanilla `(`/`)` a
~11pt (natural), cristalino esticados (~31pt) sobre as duas linhas; n com
folga 0.00/0.00 (cristalino) vs. afastamento natural de lado (vanilla).
Candidato a passo próprio (semântica de `\` vs. lr no eval/layout math).
Não cobre o achado 4/secção 36 (transformações), pendente separado per
indicação do dono.


---
**Commit final**: `132f039e8` — "docs(p995): reconciliação (n \ k) — medição
fresca bate P991 ao milésimo; divergência residual é estrutural (parênteses
esticados vs quebra de linha do vanilla)".
