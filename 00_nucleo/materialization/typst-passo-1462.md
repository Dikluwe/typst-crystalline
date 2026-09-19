# Passo 1462 — materializar `format.svg` e a opção documental `pretty`

**Estado:** L0 R2 PÓS-CONFIRMAÇÃO — CORREÇÃO DE PARIDADE CAUSAL ANTES DO CÓDIGO

**Data da medição:** 2026-09-19
**Baseline cristalino:** `7442d33b6bf48f136826d28cf71bd62f8710a432`
**Tree do baseline:** `4209cff88befc34ed8bfcd0c06fcddd012800917`
**Working tree no baseline:** limpa
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**Tree vanilla:** `2847920eeaa98d26a41dc6fb0f9c02334b9d593b`
**SHA-256 binário vanilla:**
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
**SHA-256 binário cristalino:**
`d3167689f50cc0f1cd4415f94efe7ab8fd2d5cae633d8664de93a806abd16f82`

**Confirmação ADR-0127:** recebida em 2026-09-19, depois do commit causal
`514b2ae8f35b661127e38620dcbdaefa9e158624`. A revisão R2 abaixo corrige uma
medição de paridade refutada antes de qualquer candidato; por ADR-0127, segue
em fluxo contínuo L0-primeiro, com novo selo discriminatório obrigatório.

## 1. Objetivo e fronteira

Completar somente `format.svg`, primeiro filho da raiz `format` criada no
P1461, e transportar sua opção documental `pretty` até o export SVG. O passo
inclui a precedência pública:

```text
--pretty explícito > format.svg.pretty do documento > false
```

P1462 não implementa `format.png`, `format.pdf` ou `format.html`; esses
subconjuntos permanecem respectivamente em P1463, P1464 e P1465. Também não
altera geometria, conteúdo, fonts, destinations, SVG de `html.frame`, PDF,
PNG, HTML, `--compact`, tags PDF ou feature gates.

## 2. Medição antes da decisão

### 2.1 Superfície pública

No vanilla ratificado:

```typ
#repr((type(format.svg), format.svg, format.svg.pretty))
```

produz `(function, svg, false)`. A chamada `format.svg()` falha com
`cannot be constructed manually`. `#set format.svg(pretty: true)` é aceito;
`other: true` falha com `unexpected argument: other`; `pretty: 1` falha com
`expected boolean, found integer`.

No cristalino P1461, a raiz existe com scope vazio e todas essas sondas param
em `module format does not contain svg`. Um `set` pontuado alcança o fallback
de `eval_set_rule` com target textual vazio e gera warning de target não
suportado, sem transportar a decisão.

Calibração independente posterior à confirmação, ainda sem candidato, mediu
também `#set format.svg(pretty: true)` seguido de `#set format.svg()`. No
vanilla ratificado, a saída permaneceu indentada/`true` em `forward`,
`reverse` e `forward-repeat`. Logo named omitido **não escreve** o default:
uma set rule vazia é semanticamente neutra e preserva a decisão cascaded
anterior. O diagnóstico reproduzível é
`00_nucleo/diagnosticos/p1462-oraculos-independentes.md` (SHA-256
`f452d0229c4dba6a87b5c3ed316daaaca84c9c066686a197034460555ed624d7`).

### 2.2 Efeito e precedência no export

Uma página vazia de 10 pt foi exportada bilateralmente. No vanilla:

| Documento | CLI | Forma observada |
|---|---|---|
| sem regra | ausente | compacta |
| `pretty: true` | ausente | indentada |
| `pretty: true` | `--pretty=false` | compacta |
| `pretty: false` | `--pretty=true` | indentada |

Na fixture medida, a forma compacta tinha 243 bytes e SHA-256
`daa6764b54a6521b0a68ca2e11177293b73fc2bbba1c1503467dd4a858f704ac`;
a forma indentada tinha 248 bytes, três LF e SHA-256
`bd2fbab674ae0cbdbcfb777772e2298ddcbf25867b24b66cfd72e954a3a0146`.
Esses números identificam a medição e não são critério normativo de bytes.

No cristalino, L2 já preserva `pretty: Option<bool>`, mas L4 executa
`unwrap_or(false)` antes do dispatch. L3 recebe somente `bool` e o passa a
`SvgOptions`; não existe carrier documental.

### 2.3 Top-level e containers

No vanilla, regra top-level controla o documento. A mesma classe de regra
dentro de container falha com:

```text
format set rules are not allowed inside of containers
```

`StyleDelta.custom` e `Content::Styled` já transportam estilos no cristalino;
`Content::map_content` já percorre a árvore exaustivamente. Não há extração de
opção documental ou associação a `PagedDocument`.

### 2.4 Proveniência das fontes medidas

- `lab/typst-original/crates/typst-svg/src/format.rs` —
  `2ef662dcd04bbe5efd27fa3f8f1fd5ed675e0c1c543d2bfaadefdebf474ef04b`
- `lab/typst-original/crates/typst-svg/src/lib.rs` —
  `254107d5f264eb40070e4fa34071d144067845dc922d6102ee13ddec565739b5`
- `lab/typst-original/crates/typst-library/src/format.rs` —
  `37ce18defae73642aa9e6f74cb8a78dffba575b004d9a8ddfbf52cb272898a1f`
- `lab/typst-original/crates/typst-realize/src/lib.rs` —
  `860317758996d74c0f76547bdca141619064b777ab86b8f9e5294239af9e0b53`
- `lab/typst-original/crates/typst-cli/src/compile.rs` —
  `a943a87637dd1f85a95f39c29175225c378cbf6d7a0e8d30645eb5cca7470703`
- `lab/typst-original/crates/typst-cli/src/world.rs` —
  `25750a6dd5e7dcf086dcdeab9a416da4c7a3229e56ee0a5e505ceb0a7222328d`

## 3. Classificação ADR-0107/0108

São observáveis de linguagem: existência/tipo/nome/default de `format.svg`,
fields públicos, validação de named/tipo, impossibilidade de construção,
efeito da set rule, restrição top-level, mensagem de container e precedência
CLI/documento/default. Whitespace SVG é observável somente quanto ao modo
compacto/pretty solicitado; a morfologia normalizada deve permanecer igual.

São mecânica livre: macros/traits upstream, tipo concreto de coleção, downcast,
endereços, número de passes, uso de contadores, forma Rust do carrier, bytes
exatos e SHA da fixture, desde que os observáveis acima sejam preservados.

Inferência marcada: uma travessia exaustiva comparada à travessia exclusiva de
envelopes root distingue regras nested sem enumerar containers. Refutadores:
variante não percorrida por `map_content`, perda da ordem lexical, set rule sem
`Content::Styled` ou falso positivo em conteúdo root legítimo. Qualquer
refutador reabre L0; não se corrige por exceção nominal ad hoc.

## 4. Decisão arquitetural causal

1. `compiler/stdlib/format.rs` acrescenta o filho settable `svg`, único membro
   novo, com namespace `pretty: false` e constructor manual sempre falho.
2. `compiler/eval/rules.rs` reconhece estruturalmente `format.svg`, valida
   somente `pretty: bool` e, quando o named está presente, escreve
   `format.svg.pretty` no carrier custom. Regra sem argumentos não escreve a
   chave e preserva a cascata anterior; `false` é default somente quando não
   existe decisão documental.
3. Novo owner puro `compiler/layout/format_options.rs` extrai opções root,
   rejeita ocorrência nested e devolve `DocumentFormatOptions`.
4. Novo owner `entities/document_format_options.rs` declara os carriers
   públicos, deliberadamente incompletos até P1463–P1465.
5. `PagedDocument` recebe `format_options`, inicializado com default.
6. L3 coleta opções do conteúdo realizado final, associa-as somente ao
   documento completo e resolve `Option<bool>` contra o valor documental.
7. L4 preserva `CompileIntent.pretty` como `Option<bool>` apenas no braço SVG.
8. L2 e o exporter SVG não mudam: já fornecem o tri-state cru e consomem o
   booleano final, respectivamente.

O Núcleo Tekt
`00_nucleo/prompts/_nuclei/export/format-option-precedence.toml` possui
SHA-256 bruto
`ac1500ecb978576471139f7525c6b824b273fa2fc3a02ff3ca304c6880598295` e
SHA-256 efetivo V26
`51f5e11108498faec8be851c97ce6ddc642cabf580eaada021b64369dd04cb9f`.
O pin efetivo é consumido pelos prompts de pipeline e wiring. Código nunca
aponta para o Núcleo.

## 5. Ownership 1:1

| Prompt L0 | Consumer exclusivo após confirmação |
|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` |
| `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` |
| `compiler/layout.md` | `01_core/src/compiler/layout/mod.rs` |
| `compiler/layout/format_options.md` | `01_core/src/compiler/layout/format_options.rs` |
| `entities/document_format_options.md` | `01_core/src/entities/document_format_options.rs` |
| `entities/mod.md` | `01_core/src/entities/mod.rs` |
| `entities/layout_types.md` | `01_core/src/entities/layout_types.rs` |
| `infra/pipeline.md` | `03_infra/src/pipeline.rs` |
| `wiring.md` | `04_wiring/src/main.rs` |
| `wiring/tests/p1462-format-svg.md` | `04_wiring/tests/p1462_format_svg.rs` |

Não alterar `shell/cli.rs`, `infra/export/svg.rs`, `style_chain.rs`,
`entities/func.rs`, `entities/module.rs` ou as fachadas de export sem um
refutador demonstrado e atualização L0 anterior.

## 6. Gate ADR-0127

P1462 adiciona função/elemento público, set rule, campos públicos, API L1/L3,
muda o default efetivo do export SVG quando a flag está ausente e insere
extração entre realização e documento completo. O gate é obrigatório.

Sequência imutável:

1. este passo e os L0s são commitados sem código;
2. o dono lê o commit causal e confirma explicitamente;
3. só então são escritos testes RED e congelados os oráculos;
4. só depois nasce a implementação candidata;
5. ataques, mutation, preseal, hashes e validação final vêm depois.

A autorização anterior para P1461 e o pedido que originou este documento não
substituem a confirmação posterior ao commit P1462.

## 7. Protocolo segregado após confirmação

A execução seguirá o skill `tekt-materializacao-segregada` em protocolo
completo. O workspace é compartilhado; portanto a atestação será
`executado sem atestação de isolamento`. O implementador não receberá a
redação dos oráculos adversariais além do contrato público necessário.

Artefatos planejados em `00_nucleo/diagnosticos/`:

- `p1462-contrato-congelado.md` — matriz de obrigação e inventário de owners;
- `p1462-oraculos-independentes.md` — probes bilaterais e expected imutável;
- `p1462-mutantes.md` — catálogo, execução e kill matrix;
- `p1462-implementacao.md` — diff causal e desvios justificados;
- `p1462-ataques.md` — repetição, inversão, negativos, controls e unknowns;
- `p1462-preseal.md` — auditoria independente antes dos hashes;
- `p1462-veredito.md` — proveniência final, gates e conclusão limitada.

Papéis independentes: autor do contrato, autor dos oráculos, implementador,
adversário e preseal/veredito. Reuso de papel ou contacto inevitável é
registrado no artefato correspondente; não é alegada independência forte.

## 8. Oráculos e testes mantidos

### Unidade L1

- identidade, repr, namespace/default, alias e construção proibida de
  `format.svg`;
- named/tipo/duplicado/posicional de `#set format.svg`;
- regra vazia neutra, inclusive depois de `true`, sem carrier artificial;
- collector: ausência, true/false, repetição/inversão, sequence/styled root,
  ao menos dois containers, estado interno inválido e neutralidade futura;
- defaults/clone/igualdade dos carriers e default em `PagedDocument`.

### Integração L3/L4

- matriz completa CLI/documento/default;
- XML válido, compactação/indentação e morfologia normalizada igual;
- regra nested falha e não publica artefato;
- PDF/PNG/HTML e SVG interno de `html.frame` não observam a opção;
- wrappers SVG antigos preservam compatibilidade explícita.

### Inventário público P1357

P1462 acrescenta exatamente `format.svg`: `BOTH +1`, `VANILLA_ONLY -1`,
`CRYSTALLINE_ONLY` invariável. Totais esperados:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY |
|---|---:|---:|---:|
| default | 2000 | 21 | 42 |
| a11y-extras | 2003 | 24 | 42 |
| html | 2115 | 136 | 42 |
| html+a11y | 2118 | 139 | 42 |

Qualquer filho adicional é fuga de escopo.

## 9. Mutantes obrigatórios

O conjunto mínimo inclui:

- omitir `format.svg` ou registrá-lo sob nome errado;
- default `pretty == true`;
- permitir construção manual;
- aceitar/ignorar named desconhecido ou tipo não booleano;
- não transportar set rule ou usar chave errada;
- fazer set rule vazia apagar a decisão anterior com `false`;
- aplicar primeira regra em vez da última e o inverso indevido;
- aceitar nested, rejeitar root ou não detectar segundo container;
- colapsar `None` em `false` em L4/L3;
- fazer documento vencer CLI explícito;
- vazar a opção para PDF/PNG/HTML/frame HTML;
- adicionar `format.png`, `format.pdf` ou `format.html` antecipadamente.

Mutante equivalente ou não compilável é classificado com evidência e não conta
como kill. Timeout, crash, falta de binário, baseline divergente ou assert
opaco são `Unknown`, nunca PASS.

## 10. Gates finais planejados

Após confirmação e implementação:

```text
cargo test -p typst-core <testes focais P1462>
cargo test -p typst-infra <testes focais P1462>
cargo test -p typst-wiring --test p1462_format_svg
cargo test --workspace
cargo build
crystalline-lint .
git diff --check
```

Também serão executados os inventários P1357 e os probes bilaterais congelados.
O baseline P1461 registrava oito falhas históricas P1450/P1451 no workspace;
elas não recebem crédito verde e devem ser separadas de qualquer regressão
P1462. A conclusão só pode alegar o fragmento `format.svg` efetivamente
provado, nunca paridade global de `format.*` ou SVG.

## 11. Adendo de recuperação — ADR-0131

Em 2026-09-19, a campanha R4 concluiu o vetor `direct` com P00 preservado e
M01–M18 rejeitados, mas uma interrupção externa durante `direct_repeat/M12`
impediu a terceira ordem e o inventário P1357. O descarte integral foi
registrado no commit `bb4ee22f9204764a75b0f191055e7800566942b8`.

O dono aprovou ADR-0131 depois desse resultado. Para a retomada de P1462:

1. o vetor `direct` completo permanece evidência válida após auditoria dos
   pins registrados;
2. o estado parcial de M12 é `Interrupted` e não recebe crédito;
3. o inventário P1357 é a única unidade pré-candidato ainda obrigatória;
4. P1357 verde + auditoria dos pins autorizam o teste RED e o candidato;
5. a certificação final repete por risco P00, M03, M12, M13 e fronteiras
   afetadas, sem rerodar universalmente M01–M18 em três ordens.

Este adendo substitui apenas a mecânica de repetição e retomada. Semântica,
owners, catálogo M01–M18, score `1.0`, gates L0/V15/V26 e separação de papéis
permanecem inalterados.
