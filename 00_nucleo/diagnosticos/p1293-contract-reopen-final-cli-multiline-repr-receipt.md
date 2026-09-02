# P1293 — reabertura final test-only do `repr` HTML na suíte CLI

Data: 2026-09-02  
Papel: autor L0/contrato segregado  
Estado: **L0 atualizado; aguardando resselo de lineage e selo test-only separado**. Este receipt não aprova P1293, não altera produto e não é certificado.

## Regime e autoridade

Foi aplicado o protocolo completo de materialização segregada, limitado à
autoria da obrigação test-only. O autor leu o blocker público, o owner L0 e o
consumer de integração; não leu nem editou oráculos protegidos e não escreveu
produto, consumer, manifesto, selo, ataques ou veredito.

Escritas desta fase:

- `00_nucleo/prompts/wiring/tests/cli.md`;
- este receipt.

Qualquer escrita posterior no consumer depende de resselo e nova autoridade
separada. O filesystem é compartilhado; portanto a alegação é segregação por
capacidades e artefatos, sem atestação de isolamento técnico de leitura.

## Inputs congelados

| Entrada | SHA-256 |
|---|---|
| blocker `00_nucleo/diagnosticos/p1293-definitive-final-verification-blocker-receipt.json` | `6e658e3bb2293705755b70e5cf8a1cebdbac0ca51bc0fc3c665e5d3a23d72920` |
| L0 pré-decisão `00_nucleo/prompts/wiring/tests/cli.md` | `ccd47ca2c66c6830b3e5cde505987c7731aa7a78be4b0e58e7b1c29e261413d4` |
| consumer `04_wiring/tests/cli.rs` | `cb19a63665aa5c66feae200a03ce4537097a4f25c67489dd457c03006031b288` |
| header vigente do consumer | `aa5502ce` |
| L0 pós-decisão | `4f12b058d78aad0b4c3944efcab9b1ebaa5382a2ff85e08e5a3993a65db51e32` |
| diff focal do L0 | `92a2d76281a5592403d1f2c8ce2a9c65cb73bb942096d17a7f2dee2d64d7ff9d` |

Proveniência: HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, working tree não commitada,
snapshot `2026-09-02T19:53:47-03:00`; `git status --porcelain=v1`
SHA-256 `2d58e5e37c056cebb0bd54fdcfba7141093ba2129f19cad0141ee5d069e96475` e
`git diff HEAD --stat` SHA-256
`85faad44d0ff52900662ae90621ccf099b0ae7313dde17ba7b5373fa7ae41be0`.

## Medição antes da decisão

O blocker independente mediu `cargo test --workspace -q`: core `5417/5417`,
infra `918/918` e wiring CLI `64/71`. Os sete failures são exclusivamente:

1. `p1168_html_typed_batch_repr_and_dom`;
2. `p1173_1_html_lote_global_only_2_e_whitespace_block`;
3. `p1174_1_html_lote_global_only_3`;
4. `p1175_1_html_lote_global_only_4_e_espaco_protegido`;
5. `p1176_1_html_lote_residual_normal`;
6. `p1177_1_html_familia_ruby`;
7. `p1178_1_html_familia_documento`.

No consumer atual:

- `:1849-1867` procura `elem(tag: ...)` dentro do tuple P1168;
- `:2063-2079`, `:2108-2124`, `:2153-2169`, `:2198-2214`,
  `:2243-2259` e `:2288-2304` comparam strings monolinha de `abbr`, `mark`,
  `picture`, `summary`, `ruby` e `title`;
- os mesmos testes continuam depois com compilação e expectativas DOM, sem
  depender da asserção `repr` para construir o HTML.

Sete execuções read-only do release com os mesmos expressions confirmaram:

- o tuple P1168 conserva doze membros e a ordem vigente; membros simples podem
  permanecer inline, mas `p` é um bloco `elem(` com linhas indentadas `tag`,
  `attrs`, `body`, vírgulas de campo e `)`;
- os seis escalares são blocos multiline com `tag`, `attrs`, `body` nessa ordem;
- valores medidos permanecem `abbr/sigla/A`, `mark/m/M`, `picture/pic/P`,
  `summary/s/S`, `ruby/r/R` e `title/t/T`, todos com `hidden: ""`;
- o output não possui conversão final para a antiga forma monolinha.

O mesmo blocker registrou C-P08 protegido normal/reverso `11/11`, focal P1293
`51/51`, superfícies default/HTML completas e zero witness DOM divergente. Os
sete testes param na expectativa textual antes de chegar ao controlo DOM.

## Decisão e refutador

O L0 agora exige que P1168 compare o tuple canônico completo, incluindo o bloco
multiline do membro `p`, todos os doze membros e sua ordem. P1173–P1178 devem
comparar exatamente os seis blocos multiline medidos, sem normalizar whitespace,
aceitar simultaneamente monolinha ou reduzir a prova a buscas parciais.

Conteúdo, tags, attrs, body, ordem, escaping, sources, outputs e expectativas
DOM ficam byte-conceitualmente preservados. P1169–P1172 e todos os demais testes
ficam fora do escopo. Produto e repr canônico não podem ser revertidos para
monolinha.

Inferência: a troca exclusiva das sete expectativas/buscas fecha um stale-test
causado pela evolução C-P08 já confirmada. Refutador: se qualquer teste continuar
RED após a comparação multiline exata, se um DOM posterior divergir, se conteúdo
ou ordem precisarem mudar ou se produção for necessária, a correção para e o
owner causal é reaberto; não se adapta a expectativa por proximidade.

Classificação:

- ADR-0107/0108: `repr` textual é o observável; a medição antecedeu a decisão;
- ADR-0127: test-only e alinhamento à decisão C-P08 já confirmada, sem contrato
  público, default, fase ou quebra nova; fluxo contínuo, sem gate humano;
- ADR-0128: target HTML permanece eixo separado;
- ADR-0129: owner 1:1 permanece
  `wiring/tests/cli.md` → `04_wiring/tests/cli.rs`.

## Ownership, gates e dry-run

- V15: **PASS**, zero violações.
- V26: **PASS**, zero violações.
- `git diff --check`: **PASS**.
- `crystalline-lint --fix-hashes --dry-run .`: exatamente uma mudança esperada:

```text
Would fix ./04_wiring/tests/cli.rs prompt=00_nucleo/prompts/wiring/tests/cli.md old=aa5502ce hash-a=7396cebb hash-b=d4efbe07
```

Nenhum outro consumer aparece no dry-run. `--fix-hashes` não foi executado.

## Próxima ação e STOP

O coordenador pode ressellar somente o header de `04_wiring/tests/cli.rs` e
executar os gates arquiteturais. Depois, um selo separado pode autorizar apenas
as sete expectativas/buscas `repr` de P1168 e P1173–P1178 e o receipt de
implementação correspondente.

PARE antes de editar o consumer. Não alterar produto, DOM, oráculos, manifesto,
selo, expectativas fora das sete, ou emitir aprovação/certificado.
