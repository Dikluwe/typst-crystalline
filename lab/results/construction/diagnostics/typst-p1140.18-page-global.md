# Diagnóstico P1140.18 — auditoria da função global `page`

**Data:** 2026-08-24  
**Estado:** fechado com decisão γ  
**Código L1:** não alterado

## 1. Conclusão

`page(...)` é superfície vigente no vanilla ratificado, não uma forma legacy.
Contudo, restaurar o binding agora seria publicamente enganoso: o cristalino
representa completamente apenas 3 dos 19 parâmetros e não possui os mecanismos
que definem a morfologia do constructor — isolamento do page-run, `flush`
invisível não vazio e restauração da configuração anterior.

Decisão: **γ — manter `page` como lacuna conhecida e materializar primeiro os
pré-requisitos**. O antigo `native_page` removido em P335 não será ressuscitado.

## 2. Evidência que refuta P335

P335 classificou `page(...)` como legacy porque não havia call-sites internos e
`#set page(...)` já existia. A fonte ratificada refuta os dois saltos:

- `lab/typst-original/crates/typst-library/src/layout/page.rs:21-25` documenta
  uso explícito como função;
- `page.rs:53-54` declara `PageElem` com `Construct`;
- `page.rs:494-501` exige `body` e promete múltiplas páginas com restauração das
  propriedades anteriores;
- `page.rs:504-523` implementa uma sequência delimitada, `flush` e styling
  local;
- call-sites relevantes são programas Typst externos, não referências Rust
  internas.

O inventário P1140.17 confirma `page` como `function` vanilla e
`MISSING_BINDING` cristalino.

## 3. Sondas públicas

Executadas contra `/usr/local/bin/typst`, build ratificado `a51e02804`, e
`target/release/typst` da working tree:

| Sonda | Vanilla | Cristalino |
|---|---|---|
| `repr(type(page))` | `"function"` | `unknown variable page` |
| `repr(page)` | `"page"` | `unknown variable page` |
| `repr(std.page)` | `"page"` | `std` sem campo `page` |
| `repr(page[alpha])` | sequência delimitada | binding ausente |
| `repr(page(body: [alpha]))` | erro: body é posicional | binding ausente |
| `repr(page())` | erro: body ausente | binding ausente |
| `repr(page(foo: 1)[x])` | `unexpected argument: foo` | binding ausente |
| `repr(page([x], [y]))` | `unexpected argument` | binding ausente |

O catálogo runtime marcou `body` como não posicional; a execução é o oracle de
linguagem e o corrigiu: content block e primeiro argumento fornecem o body,
enquanto `body:` é rejeitado com hint para retirar o nome.

## 4. Morfologia medida

`repr(page[alpha])` produz:

```text
sequence(
  pagebreak(weak: true),
  flush(),
  [alpha],
  pagebreak(weak: true),
)
```

Quando há propriedades, a sequência aparece dentro de `styled(child: ..., ..)`.
`page[]` conserva o `flush()`. Duas chamadas consecutivas conservam dois
markers e suas fronteiras. `block(page[x])` conserva a mesma sequência dentro
do body do bloco. A fonte esclarece que a fronteira final é uma boundary ainda
mais fraca e que o mapa de estilos é local; a representação pública não expõe
essa diferença mecânica, mas os efeitos de isolamento e restauração são
observáveis.

## 5. Classificação A/B/C/D

| Parâmetro | Classe | Evidência cristalina | Lacuna |
|---|:---:|---|---|
| `paper` | C | sem campo em `SetPage`/`PageConfig` | parsing e tabela de papel |
| `width` | A | `eval/rules.rs:1240`, `set_page.rs:32-38` | — |
| `height` | A | `eval/rules.rs:1241`, `set_page.rs:39-45` | — |
| `flipped` | C | sem campo em `SetPage`/`PageConfig` | orientação |
| `margin` | B | `eval/rules.rs:1242-1281`, `set_page.rs:46-50` | `inside`/`outside` e binding |
| `bleed` | C | sem campo em `SetPage`/`PageConfig` | domínio e export |
| `binding` | C | sem campo em `SetPage`/`PageConfig` | lado lógico |
| `columns` | A | `eval/rules.rs:1296-1309`, `set_page.rs:60-62` | — |
| `fill` | C | sem campo em `SetPage`/`PageConfig` | fundo físico da página |
| `numbering` | B | `eval/rules.rs:1283-1294`, `set_page.rs:56-59` | só string/none; função ausente |
| `supplement` | C | sem campo em `SetPage`/`PageConfig` | introspecção/referência |
| `number-align` | C | sem campo em `SetPage`/`PageConfig` | header/footer automático |
| `header` | C | sem campo em `SetPage`/`PageConfig` | repetição por página |
| `header-ascent` | C | sem campo em `SetPage`/`PageConfig` | geometria de header |
| `footer` | C | sem campo em `SetPage`/`PageConfig` | repetição por página |
| `footer-descent` | C | sem campo em `SetPage`/`PageConfig` | geometria de footer |
| `background` | C | sem campo em `SetPage`/`PageConfig` | camada atrás do body |
| `foreground` | C | sem campo em `SetPage`/`PageConfig` | camada acima do body |
| `body` | B | `Content` existe | sem page-run/flush/restauro local |

Totais: A=3, B=3, C=13, D=0. A fonte estrutural comum é
`entities/content.rs:516-532`; o parser atual está em
`compiler/eval/rules.rs:1175-1358`; o consumer está em
`compiler/layout/set_page.rs:19-95`; `PageConfig` contém somente as cinco
propriedades em `entities/layout_types.rs:674-705`.

Inferência: `width`, `height` e `columns` podem ser reutilizados pelo futuro
constructor. Isso será refutado se testes de isolamento mostrarem que o
consumer mutável não pode operar dentro de uma fronteira restaurável sem mudar
o modelo. Por isso A significa suporte do valor, não constructor pronto.

## 6. Por que não escolher α ou β

α exigiria completar 13 propriedades e três contratos parciais em um único
passo, com blast radius em eval, entidades, layout, introspecção e export.

β ainda não é segura para um subconjunto mínimo: a própria semântica do
constructor exige preservar página vazia e restaurar configuração depois do
body. O cristalino só possui `Content::SetPage`, que muda `PageConfig` a partir
do ponto de ocorrência (`set_page.rs:29-75`), e não um page-run lexical. Expor
uma função parcial com `SetPage + body` repetiria o erro do antigo P335 e
vazaria propriedades.

## 7. Sequência de pré-requisitos

O L0 de `compiler/stdlib/layout` registra a divisão:

1. P1140.19 — fronteira de page-run, marker invisível não vazio e restauração;
2. P1140.20 — propriedades ausentes/parciais e seus consumers, atomizadas em
   subconjuntos explícitos;
3. P1140.21 — constructor, binding global/`std`, diagnósticos e rebaseline.

Cada passo ainda precisa medir, atualizar seu L0 e aplicar o gate ADR-0127
quando alterar contrato público ou comportamento por defeito.

SHA-256 final desse L0:
`573b7c0642299ba3115c570b73a99a75c0a22d77c887000e70f51ccd1116fe9d`.
O arquivo L1 vinculado ainda usa o formato antigo de lineage sem
`@prompt-hash`; `crystalline-lint --fix-hashes .` respondeu `Nothing to fix`.

## 8. Proveniência

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- estado: working tree não commitada;
- hora da medição final: `2026-08-24T13:36:50-03:00`;
- `git diff HEAD --stat` antes destes documentos:
  `83 files changed, 734 insertions(+), 505 deletions(-)`;
- vanilla: `/usr/local/bin/typst`, ratificado `upstream/main a51e02804`;
- cristalino: `target/release/typst` reconstruído em P1140.17.

Os totais 3/3/13/0 pertencem somente a esse estado e às linhas citadas. Não
devem ser usados como fecho depois de alterações na entidade de página sem nova
medição.

Validação final em `2026-08-24T13:38:13-03:00`:

- working tree: `84 files changed, 775 insertions(+), 508 deletions(-)`;
- `crystalline-lint .`: exit 0;
- `git diff --check`: aprovado;
- nenhum teste/build foi repetido, pois P1140.18 não alterou código executável.
