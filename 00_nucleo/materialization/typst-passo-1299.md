# Passo 1299 — rebaseline quadrilateral da superfície pública e seleção causal do P1300

**Estado inicial:** autorizado para execução após o fechamento do P1298.
**Natureza:** auditoria diagnóstica somente leitura do produto; não altera L0 nem L1–L4.
**Baseline de autoria:** `1f082370e59939de7b57992e137a9f74bfb6758f`.
**Vanilla ratificado:** `a51e02804`, binário `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
**ADRs:** ADR-0107, ADR-0108, ADR-0127 e ADR-0129.

---

## 1. Objetivo

Produzir uma medição nova e reproduzível da superfície pública após o selo P1298,
classificar cada divergência ainda observável e escolher **um único lote causal** para o
P1300.

P1299 não implementa a correção escolhida. Em particular, não remove aliases, não muda
features, não corrige spans, não edita Prompt L0 e não ressela hashes. Quando houver
conflito entre o vanilla ratificado, o código e o L0 vigente, o resultado deste passo é
`L0_CONTRADICTION`, nunca uma autorização implícita para modificar produto.

A alegação máxima permitida é:

> A superfície pública foi reenumerada no baseline P1298, as divergências observadas
> foram classificadas por perfil, owner e natureza de linguagem, e um lote causal foi
> selecionado para um passo posterior; não se alega paridade funcional geral.

---

## 2. Medição que motiva o passo — antes da decisão

Medição do autor em `2026-09-03T18:02:48-03:00`:

- `HEAD` = `1f082370e59939de7b57992e137a9f74bfb6758f`;
- `git status --short` vazio;
- `git diff HEAD --stat` vazio;
- `target/release/typst` não existia, portanto não há binário cristalino corrente que
  possa ser presumido válido;
- `p1297-surface-default.json`, SHA-256
  `f8e36e6a38e97aa570da3905021a029321aaffa214e5b3fa78b489e9504312eb`:
  `111 total / 99 MATCH / 12 DIFFERENCE_OR_DISABLED`;
- `p1297-surface-html.json`, SHA-256
  `407570734adae8e7920f7780f9cdd5c9083bf3d4307577efd2d44e508ae7fa2e`:
  `115 total / 104 MATCH / 11 DIFFERENCE_OR_DISABLED`.

As 23 ocorrências não são 23 defeitos distintos. Elas contêm:

1. `html` desligado no perfil default, com diagnóstico byte a byte igual nos dois
   binários;
2. `pdf.data-cell` desligado sem `a11y-extras`, com texto e hints iguais, mas span
   diferente: vanilla ancora apenas `data-cell`; cristalino ancora `pdf.data-cell`;
3. 16 paths extras distintos amostrados nos dois perfis:
   `accent`, `bb`, `cal`, `calc.deg`, `counter_at`, `frak`, `grid_footer`,
   `grid_header`, `hsl`, `inline`, `lot`, `op`, `state_at`, `table_header`,
   `underover` e `upright`.

Essa lista de 16 é uma **amostra**, não o inventário completo. O enumerador P1282 mediu
45 extras numa revisão anterior, e o código atual ainda registra outros aliases globais
da mesma família. Logo, é proibido escolher o P1300 apenas contando os 16 nomes
amostrados.

### 2.1 Causas já localizadas, ainda não decididas

- `01_core/src/compiler/eval/mod.rs:1876-2000` registra helpers históricos de
  `state`/`counter` e funções matemáticas também no scope global;
- `01_core/src/compiler/eval/mod.rs:2057-2110` mantém aliases flat de `table` e `grid`
  além dos namespaces canônicos;
- `01_core/src/compiler/eval/mod.rs:1648-1654` registra `hsl`/`hsv` globalmente;
- `01_core/src/compiler/stdlib/calc.rs:88-92` declara `calc.deg`/`calc.rad` como
  extensões cristalinas mantidas deliberadamente;
- o L0 vigente de `math_style`, `state`, `counter` e `structural` legitima parte desses
  globais como compatibilidade histórica; `structural.md` declara `lot()` uma
  divergência intencional;
- o L0 de `color` afirma que os oito constructors são globais, mas a fonte vanilla
  ratificada em `lab/typst-original/crates/typst-library/src/lib.rs:397-401` registra
  globalmente somente `luma`, `oklab`, `oklch`, `rgb` e `cmyk`. O binário ratificado
  rejeita `hsl` global e aceita `color.hsl`.

Esses fatos provam que há, no mínimo, uma mistura de vazamentos públicos, extensões
deliberadas e contradições documentais. Eles não autorizam remoção em P1299.

---

## 3. Congelamento e proveniência obrigatória

Antes de qualquer build, criar:

- `00_nucleo/diagnosticos/p1299-baseline-status.txt`;
- `00_nucleo/diagnosticos/p1299-manifest.json`.

O baseline deve registrar:

- instante ISO-8601 com timezone;
- `git rev-parse HEAD` e branch;
- saída integral de `git status --short`;
- `git diff HEAD --stat` e seu SHA-256, mesmo vazio;
- `Cargo.lock`, `rustc --version` e `cargo --version`;
- SHA-256 de todo input usado;
- paths, argv, exit code, stdout, stderr, duração e SHA-256 dos binários;
- se a árvore deixar de estar limpa, a lista exata dos arquivos alterados.

Inputs mínimos pinados:

| Input | SHA-256 esperado no baseline |
|---|---|
| `lab/surface-inventory/probes.json` | `1e7534a0cc8ebcfa1f5ed6652918711b738d7471d94a883a500becf297357b78` |
| `lab/surface-inventory/run_probes.py` | `3bd082751fbd05c89f24a312353d81f74f2882902db5a203180e5ee5fb2c10cf` |
| `lab/surface-inventory/src/main.rs` | `1f56149b1be5d93c24d686259186314708d806b69279e70097eec755ce828056` |
| `lab/surface-inventory/merge.py` | `dcca5437dc8ab3829f8896b5b4f08f76fdaa49f3bf0973a62721d0b33610e329` |
| `lab/surface-inventory/extra_seeds.json` | `ec398902ad74aa7f7d21b733674e3712ed47ee84e6f79fd12fbdca5293d87623` |
| `p1282-vanilla-default.json` | `2f70d832f2b24f8c8eae8ffb15dd1e2770c667b9bfb24fcadefdcad43b4cfb27` |
| `p1282-vanilla-html.json` | `2ef941beab9ef3e2859de83c8bcfadc7a4482af417db23029b1bec0997f22c61` |
| `p1297-surface-default.json` | `f8e36e6a38e97aa570da3905021a029321aaffa214e5b3fa78b489e9504312eb` |
| `p1297-surface-html.json` | `407570734adae8e7920f7780f9cdd5c9083bf3d4307577efd2d44e508ae7fa2e` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

Qualquer identidade divergente produz `P1299_BLOCKED_INPUT_DRIFT`. Não atualizar um hash
esperado silenciosamente.

---

## 4. Build cristalino fresco

Criar diretório efêmero explícito com `mktemp -d /tmp/p1299.XXXXXX`. Não usar o
`target/release/typst` histórico e não apagar targets alheios.

Build mínimo:

```bash
TYPST_COMMIT_SHA=1f082370e59939de7b57992e137a9f74bfb6758f \
CARGO_TARGET_DIR="$P1299_RUN/target" \
cargo build --release -p typst-wiring --bin typst
```

O manifesto deve pinar o SHA-256 de
`$P1299_RUN/target/release/typst`, a versão observada e o HEAD. A string de versão não
substitui o hash do produto.

---

## 5. Reenumeração integral — default e HTML

Regerar o catálogo cristalino dos perfis `default` e `html` com o enumerador runtime.
Os catálogos vanilla P1282 podem ser reutilizados somente após validar envelope,
`vanilla_revision=a51e02804`, features e SHA do produto. O conjunto de candidatos deve
ser a união do catálogo vanilla, do catálogo cristalino descoberto e de
`extra_seeds.json`.

Produzir:

- `p1299-crystalline-default.json`;
- `p1299-crystalline-html.json`;
- `p1299-inventory-default.json`;
- `p1299-inventory-html.json`.

Invocação de referência para cada perfil:

```bash
CARGO_TARGET_DIR="$P1299_RUN/inventory-target" cargo run \
  --manifest-path lab/surface-inventory/Cargo.toml --release --quiet -- \
  <catalogo-cristalino.json> <catalogo-vanilla.json> <default|html> \
  lab/surface-inventory/extra_seeds.json "$P1299_RUN/target/release/typst"

python3 lab/surface-inventory/merge.py \
  <catalogo-vanilla.json> <catalogo-cristalino.json> <inventario.json> \
  --profile <default|html> \
  --expected-crystalline-sha256 <sha256-do-binario-fresco>
```

`UNKNOWN`, `invalid_provenance` e `blocked_by_ancestor` não contam como match nem podem
ser descartados do denominador decisório. Se aparecerem, o passo deve explicá-los e
terminar bloqueado, salvo quando uma nova observação bilateral eliminar a incerteza.

---

## 6. Matriz quadrilateral de features

O runner P1282 só conhece `default` e `html`; não o adulterar, pois seu hash é parte da
comparabilidade histórica. Criar um runner diagnóstico P1299 separado, com teste próprio,
capaz de executar exatamente os quatro perfis abaixo, sempre de modo simétrico nos dois
binários:

| Perfil | argv de feature |
|---|---|
| `default` | nenhum |
| `html` | `--features html` |
| `a11y` | `--features a11y-extras` |
| `html+a11y` | `--features html,a11y-extras` |

O catálogo de probes P1299 deve ser derivado mecanicamente de:

1. todos os `EXTRA_BINDING`, `MISSING_BINDING`, `MISSING_MEMBER`, `WRONG_KIND`,
   `UNVERIFIED_METADATA` e `UNKNOWN` da reenumeração;
2. todas as diferenças P1297;
3. canários explícitos `html`, `pdf.table-summary`, `pdf.header-cell` e
   `pdf.data-cell`;
4. para cada extra com rota canônica conhecida, um par `extra`/`canônico`, por exemplo
   `hsl`/`color.hsl`, `state_at`/`state.at`, `table_header`/`table.header` e
   `bb`/`math.bb`.

O runner deve guardar argv literal, exit code, stdout e stderr integrais. A classificação
por execução é fechada:

- `MATCH_VALUE`: ambos exit `0` e stdout idêntico;
- `MATCH_DIAGNOSTIC`: ambos falham com exit e stderr idênticos;
- `CRYSTALLINE_ONLY`: cristalino aceita e vanilla rejeita;
- `VANILLA_ONLY`: vanilla aceita e cristalino rejeita;
- `DIFFERENT_VALUE`: ambos aceitam, mas o valor público difere;
- `DIFFERENT_DIAGNOSTIC`: ambos rejeitam, mas diagnóstico, hints ou span diferem;
- `EXECUTION_UNKNOWN`: timeout, sinal, I/O incompleto ou estado não classificável.

Não normalizar carets, ranges, nomes de campo ou mensagens: no diagnóstico, essa mecânica
é o observável público. `EXECUTION_UNKNOWN` bloqueia.

Produzir `00_nucleo/diagnosticos/p1299-feature-matrix.json` e um teste do runner que mate,
no mínimo, classificadores trocados para `CRYSTALLINE_ONLY`, `MATCH_DIAGNOSTIC` e
`DIFFERENT_DIAGNOSTIC`.

---

## 7. Ledger de ownership e classificação semântica

Para cada path divergente, gerar uma linha em
`00_nucleo/diagnosticos/p1299-owner-ledger.tsv` com:

```text
path
profiles
runtime_class
crystalline_registration_file_line
vanilla_source_file_line
canonical_route
owner_prompt
owner_prompt_hash_declared
owner_prompt_sha256
l0_claim
language_class
gate_class
inference
refutation
recommended_action
```

`language_class` usa exatamente uma destas classes:

1. `EXPECTED_FEATURE_DISABLED` — ausência bilateral governada pela mesma feature;
2. `DIAGNOSTIC_SPAN_DIVERGENCE` — falha semanticamente equivalente, mas mensagem/hint/span
   público difere;
3. `LEAKED_PUBLIC_ALIAS` — path extra no cristalino e a função continua alcançável pela
   rota canônica vanilla;
4. `INTENTIONAL_PRODUCT_EXTENSION` — extensão pública explicitamente assumida pelo L0,
   sem falsa alegação de paridade;
5. `L0_CONTRADICTION` — L0 reivindica paridade/forma pública que a fonte e o binário
   ratificados refutam;
6. `MISSING_LANGUAGE_MEMBER` — presente no vanilla, ausente no cristalino;
7. `WRONG_PUBLIC_KIND_OR_IDENTITY` — ambos presentes, forma pública distinta;
8. `UNRESOLVED` — prova insuficiente.

Regras de adjudicação:

- comentários históricos como “compatibilidade” não transformam uma diferença de
  linguagem em paridade;
- uma extensão explicitamente desejada também não pode ser removida sem mudança de
  contrato público;
- se a função extra não tiver rota canônica funcional, não classificá-la como alias;
- presença em `std.<path>` e presença bare devem ser medidas separadamente quando a
  construção do scope as clona;
- a classificação vem **depois** das linhas de medição e deve registrar uma inferência e
  a observação que a refutaria;
- `Unknown` nunca é sucesso.

---

## 8. Seleção determinística do P1300

O relatório deve formar coortes por causa e owner, sem misturar remoção de superfície com
correção de feature ou de span. Aplicar esta prioridade:

1. `L0_CONTRADICTION` com rota canônica já funcional;
2. `LEAKED_PUBLIC_ALIAS` com rota canônica já funcional;
3. `DIAGNOSTIC_SPAN_DIVERGENCE`;
4. `MISSING_LANGUAGE_MEMBER` ou `WRONG_PUBLIC_KIND_OR_IDENTITY`;
5. `INTENTIONAL_PRODUCT_EXTENSION` somente se o dono decidir reabrir o contrato.

Dentro da mesma classe, preferir a coorte que:

1. compartilha um único owner de registro;
2. possui maior número de paths confirmados bilateralmente;
3. exige menor número de Prompts L0 proprietários;
4. preserva integralmente as rotas canônicas já verdes.

Empate final: ordem lexicográfica do nome da coorte.

O output deve selecionar exatamente uma coorte e listar explicitamente os paths incluídos
e excluídos. Não escolher “todos os extras” como lote genérico.

### Gate ADR-0127 para o passo seguinte

Remover ou esconder qualquer binding público é mudança de contrato/compatibilidade.
Portanto o P1300 selecionado deve começar assim:

1. medir novamente o fragmento escolhido;
2. atualizar primeiro cada Prompt L0 proprietário afetado;
3. apresentar o diff L0 e **PARAR para confirmação humana**;
4. somente após confirmação escrever testes RED e código.

P1299 não pode antecipar esse gate nem escrever a solução.

---

## 9. Artefatos obrigatórios

Ao final, devem existir:

1. `p1299-baseline-status.txt`;
2. `p1299-manifest.json`;
3. `p1299-crystalline-default.json`;
4. `p1299-crystalline-html.json`;
5. `p1299-inventory-default.json`;
6. `p1299-inventory-html.json`;
7. `p1299-probe-catalog.json`;
8. `p1299-run-matrix.py` e seus testes;
9. `p1299-feature-matrix.json`;
10. `p1299-owner-ledger.tsv`;
11. `p1299-decision-report.md`;
12. `p1299-certificate.json`.

O certificado deve pinar por SHA-256 todos os inputs e outputs, registrar contagens por
perfil e por classe, a coorte P1300 escolhida, os blockers e a claim máxima literal.

---

## 10. Gates finais

Executar, no mínimo:

```bash
python3 -m unittest \
  lab/surface-inventory/test_merge.py \
  lab/surface-inventory/test_run_probes.py \
  00_nucleo/diagnosticos/test_p1299_run_matrix.py

cargo build
cargo test --workspace
cargo fmt --all -- --check
crystalline-lint .
crystalline-lint --fix-hashes --dry-run .
git diff --check
git status --short
```

Como P1299 não altera produto ou L0, qualquer diff fora dos paths diagnósticos enumerados
acima e deste próprio passo bloqueia. O passo não faz staging nem commit.

Vereditos terminais permitidos:

- `P1299_PASS_P1300_COHORT_SELECTED`;
- `P1299_BLOCKED_INPUT_DRIFT`;
- `P1299_BLOCKED_UNKNOWN`;
- `P1299_BLOCKED_NO_SINGLE_CAUSAL_COHORT`;
- `P1299_BLOCKED_VERIFICATION`.

É proibido declarar `PASS` se o relatório não contiver a matriz dos quatro perfis, o
ledger completo, a coorte única e o gate humano ADR-0127 do P1300.
