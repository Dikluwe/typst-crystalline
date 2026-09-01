# P1292 — amendment-6: projeção `repr` dos filhos sintáticos de vec

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`46ca2b443742ebe563449f1c4656b4e7a833fbac8c747bfbad85e4f55add5751`

**Predecessor v6:** canônico
`c5eb479c218354af0910c17eaf24447122b8d91775d70c3031950c426ff27f3b`,
seal `28b5eec0abb668408b68672a0b8230628f08393022d9602827a4b04e540e5410`.

**Proveniência:** `HEAD`
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Medição em `2026-09-01T02:15:43-03:00`; antes da decisão,
`git diff HEAD --stat` registrou 35 arquivos, 1.556 inserções e 330 remoções.
Após o L0, em `2026-09-01T02:16:23-03:00`, registrou 35 arquivos, 1.619
inserções e 330 remoções.

Binários: vanilla ratificado `/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
candidato `target/debug/typst` SHA-256
`f5f10503bf30b1a612d1013063e1baebb4b91ce6a16426cca3e392ef3406a9aa`.

## Medição black-box

Fixture principal `/tmp/p1292-amendment6/probe-matrix.typ` SHA-256
`f7549e952a21f33d8b6fb7f59d36d40b634cb38bc8a1b7fe7681a8bdbbab42d6`.
Comando bilateral:

```text
<binário> query /tmp/p1292-amendment6/probe-matrix.typ \
  '<p1292-a6>' --field value --one --pretty
```

| Classe | Sintaxe vanilla | Qualificada vanilla | Candidato antes do amendment |
|---|---|---|---|
| identificadores `a,b` | `([a], [b])` | `([a], [b])` | `(a, b)` |
| números `1,23` | `([1], [23])` | `([1], [23])` | `("1", "23")` |
| texto multicaractere `"foo","bar"` | `([foo], [bar])` | `([foo], [bar])` | já bilateral |
| id multigrapheme `alpha,beta` | `([α], [β])` | `([α], [β])` | `("α", "β")` |
| grupo `(a+b),(c)` | `lr(body: ...)` | mesmo `lr(body: ...)` | sintaxe e math-body qualificada convergem na forma genérica própria |
| markup strong/emph | `strong(...)`, `emph(...)` | iguais | já bilateral |

As células mostram apenas o valor de `children` quando isso evita repetir
`vec(children: ...)`. O probe básico confirma também que a equação externa é
`equation(block: false, body: vec(...))`; P1292 não a altera.

`foo` sem aspas não é testemunha válida: o vanilla devolve `unknown variable
foo` e recomenda aspas ou letras separadas. O texto quoted cobre conteúdo
multicaractere; `alpha`/`beta` cobrem o identificador multigrapheme resolvido.

## Classificação e decisão

Converter `MathIdent` armazenado em `Text` é refutado porque muda o render
matemático já bilateral. A paridade exigida é de morfologia pública, não da
estrutura Rust (ADR-0107). O owner correto já é
`compiler/eval/repr.md`: ao formatar somente itens diretos de
`MathVec.children`, folhas `MathIdent` e `MathText` projetam sua sequência
visível pelo formatter canônico de conteúdo textual, resultando em `[a]`,
`[23]` e `[α]`. A entidade e o renderer continuam recebendo as variantes
originais.

Texto e markup não são reprojetados. Variantes estruturadas usam seu próprio
formatter, sem recursão especial. A diferença genérica candidata na forma
interna de `MathLr` é edge scope preexistente: sintaxe e chamada qualificada
com o mesmo math-body convergem, mas igualdade vanilla exata desse formatter
não pode ser mascarada por wrappers ou por mudança em `Equation` neste lote.

Refutadores: qualquer mudança visível de layout; mudança de igualdade/hash,
span ou traversal; double-wrap; escaping divergente; projeção dentro de grupo;
ou não convergência dos pares de folha medidos. Conteúdo/variantes não medidos
permanecem `Unknown`.

## Ownership e escopo

Único L0 alterado:

```text
00_nucleo/prompts/compiler/eval/repr.md
SHA-256 8f4f3f874b2c08d8e05278763ea795fe8d125ccb67509ce93ec4f5cfda53880d
owner 01_core/src/compiler/eval/repr.rs
```

O set permanece com 22 owners. Nenhum campo/wrapper, renderer, formatter
genérico de `Equation`, código, teste, ataque, oráculo ou veredito foi
lido/editado. `--fix-hashes` não foi usado. O implementador recebe somente a
projeção estreita acima; a divergência V5 esperada deve ser fechada por ele ao
materializar o novo L0. **PARAGEM.**
