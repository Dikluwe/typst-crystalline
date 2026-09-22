# Passo 1479 — Sanitização do agregador `export/tests.rs`: decomposição por cluster

**Estado:** MATERIALIZADO — `crystalline-lint .` zero achados `[PARSE]`
(critério de sucesso primário atingido), V5/V15/V26 zero, regressão
completa GREEN — ver §7

**Data da auditoria:** 2026-09-22T14:58:18-03:00

**Baseline de redação:** `d8a5787c7fad01f63643fbd4177098b1702a3ec1`,
tree `b91396a6e911ed7f7047c52cc4d6f5178fffde2c`, working tree limpo.

## 0. Origem

Fecha a pendência deixada explicitamente em aberto por
`typst-passo-1478.md` §10 ("A sanitização dedicada do agregador de
testes... é um passo próprio, fora do escopo deste") e por
`infra/export/builder.md` §P1477/§P1478 (achado `[PARSE]` do
`crystalline-lint` em `tests.rs:11146`, bloqueando o gate `0 erros` de
P1477 e P1478 apesar de ambos estarem funcionalmente GREEN).

## 1. Medição — a decomposição já era antecipada, não proibida

`00_nucleo/prompts/infra/export/tests.md` (L0 atual, hash `0c915794`)
§"Restrições estruturais": "Não decompôr por cluster em P307b.1 — perda
de cobertura cross-feature esperada. **Decomposição futura só se ratio
LOC/coverage justificar.**"

ADR-0100 §"Consequências / Positivas": "Testes podem decompôr-se
gradualmente: `tests.rs` agregador é primeira fase; P307b.2/posterior
**pode separar por cluster se útil**."

ADR-0037 Regra 5, Ajuste C: a exceção ao limite de 800 linhas é para
**testes E2E cross-cutting que cruzam múltiplos domínios na mesma
invocação**. Os 230 testes originais (`p270_*` CMYK, `p273_*` gradient
relative+clip, `p272_*` conic Coons, etc., catalogados no L0 atual)
satisfazem essa definição — cruzam gradients+images+multifont+layout
na mesma chamada `export_pdf`. Os quatro módulos `p1475_pdfa_
conformance_core_tests`/`p1476_pdfa_attachments_af_tests`/`p1477_pdfa_
icc_version_and_cidset_tests` (em `tests.rs`) e `p1478_pdfa4f_empty_
embedded_files_tests` (em `builder.rs`, por decisão explícita de
P1478) **não** são cross-cutting nesse sentido — cada um testa
exclusivamente o contrato de um único passo, com helpers locais
próprios já duplicados entre si (`object_body`/`catalog_body`/
`standards_for` reimplementados quase identicamente três vezes). São
"cluster" no sentido exato do texto de ADR-0100 — candidatos naturais
à extração, sem perda de cobertura cross-feature (não têm nenhuma).

Confirmado por bissecção controlada (feita durante a integração de
P1477, registada em `infra/export/builder.md` §P1477): remover **só**
o módulo `p1477_pdfa_icc_version_and_cidset_tests` (473 linhas) do
`tests.rs` (11221 linhas) faz o achado `[PARSE]` desaparecer. `rustc`
(via `cargo build`/`cargo test`) aceita o ficheiro inteiro sem qualquer
erro em todas as medições desta sessão — o limite é do parser próprio
do `crystalline-lint`, não da linguagem.

## 2. Objetivo

Extrair os quatro módulos por-passo (`p1475`/`p1476`/`p1477`, hoje em
`tests.rs`; `p1478`, hoje em `builder.rs`) para ficheiros próprios sob
`03_infra/src/export/tests/`, convertendo `tests.rs` em `tests/mod.rs`
— mesmo padrão já em vigor para `gradients/` (`mod.rs` + `linear.rs` +
`radial.rs` + ...). Os 230 testes originais cross-cutting **ficam**
em `tests/mod.rs` — não são candidatos a extração (são exatamente o
caso que ADR-0037 Regra 5 Ajuste C protege).

Resultado esperado: `crystalline-lint .` deixa de reportar `[PARSE]`;
P1477 e P1478 podem ser marcados `MATERIALIZADO`/selados retroativamente
(nota, não reabertura de gate — zero mudança de comportamento).

## 3. Contrato

### 3.1 Estrutura de ficheiros

```
03_infra/src/export/tests.rs                         (renomeado)
    ↓
03_infra/src/export/tests/mod.rs                      (conteúdo restante de tests.rs)
03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs
03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs
03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs
03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs  (novo — movido de builder.rs)
```

`export/mod.rs` não muda (`#[cfg(test)] mod tests;` já resolve
`tests/mod.rs` automaticamente pela convenção padrão do Rust — ficheiro
vs. diretório com `mod.rs` são intercambiáveis para o resolvedor de
módulos).

### 3.2 Mecânica da extração (por módulo)

Para cada um dos quatro módulos: o conteúdo da chave `{ ... }` do
`mod pXXXX_..._tests { ... }` (sem o wrapper `mod { }` em si — o
ficheiro passa a SER o corpo do módulo, convenção padrão de
ficheiro-como-módulo) move-se verbatim para o novo ficheiro. Cabeçalho
`//! Crystalline Lineage` + `@prompt`/`@prompt-hash`/`@layer`/
`@updated` novo, apontando ao L0 próprio (§3.3). Comentário
introdutório existente (hoje como `//` antes do `mod { ... }`) vira
`//!` no topo do novo ficheiro.

Em `tests/mod.rs`, cada extração é substituída por uma declaração
`#[cfg(test)] mod pXXXX_..._tests;` (explícito, mesmo que redundante
com o `#[cfg(test)]` já herdado de `export/mod.rs:994` — clareza local,
o ficheiro fica auto-explicativo sem depender de ler a cadeia inteira).

### 3.3 P1478 — de `builder.rs` para ficheiro próprio

O módulo `p1478_pdfa4f_empty_embedded_files_tests` foi colocado em
`builder.rs` por decisão explícita de P1478 (só para evitar agravar o
achado `[PARSE]`, não por pertencer arquitetonicamente a `builder.rs`).
Com a sanitização, a causa que motivou essa exceção desaparece — o
módulo volta ao lugar natural (`tests/`, ao lado dos seus irmãos
`p1475`/`p1476`/`p1477`).

Ajuste de imports: `super::super::export_pdf_with_document_id_and_tags_
and_serialization_and_pages_and_standard`/`super::super::PdfTags`
continuam corretos sem alteração (mesma profundidade de aninhamento —
`export/mod.rs` → `tests/mod.rs` → `tests/p1478_....rs`, dois níveis,
idêntico a `export/mod.rs` → `builder.rs` → `mod p1478{}` antes). Faltam
três imports explícitos que hoje vêm de bare-imports do topo de
`builder.rs` (não disponíveis a partir de `tests/mod.rs`, que não os
importa bare): `PdfStandards`, `AttachedFileRelationship`,
`PagedDocument` — acrescentar `use typst_core::entities::document_
format_options::PdfStandards;`, `use typst_core::entities::elements::
pdf_attach::AttachedFileRelationship;`, `use typst_core::entities::
layout_types::PagedDocument;`, mesmo padrão já usado por
`p1476_pdfa_attachments_af_tests`.

### 3.4 Owners L0 — cardinalidade 1:1 (ADR-0129)

Cinco L0s, mesmo padrão de `gradients/*.md`:

- `00_nucleo/prompts/infra/export/tests/mod.md` — sucessor de
  `tests.md` (mesmo conteúdo, ajustado ao novo ficheiro-alvo e à lista
  de submódulos extraídos).
- `00_nucleo/prompts/infra/export/tests/p1475_pdfa_conformance_core_tests.md`
- `00_nucleo/prompts/infra/export/tests/p1476_pdfa_attachments_af_tests.md`
- `00_nucleo/prompts/infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests.md`
- `00_nucleo/prompts/infra/export/tests/p1478_pdfa4f_empty_embedded_files_tests.md`

`00_nucleo/prompts/infra/export/tests.md` (antigo) é removido (o
ficheiro que legitimava deixou de existir — `git mv` no L0 espelhando
o `git mv` no código, não apenas apagar).

`infra/export/builder.md` perde a nota "RED — localização especial"
de §P1478 (não se aplica mais) — substituída por uma nota curta
remetendo para `tests/p1478_....md`.

## 4. Invariantes (ADR-0037 Regra 7)

- **Zero mudança de comportamento observável.** Nenhum teste muda de
  corpo, nome de função, ou asserção.
- **Contagem de testes preservada.** `cargo test -p typst-infra --lib`
  reporta o mesmo total antes/depois (só a distribuição por "caminho"
  reportado pelo runner muda, ex. `export::tests::p1475_...` vira
  `export::tests::p1475_pdfa_conformance_core_tests::...` — o `mod`
  interno já tinha esse nome, só o ficheiro que o contém muda).
- **Zero nova dependência externa.**
- **`crystalline-lint .` zero violations** — critério de sucesso
  primário deste passo é precisamente isto passar a ser verdade de
  novo (inclui o achado `[PARSE]` desaparecer).
- Histórico Git preservado via `git mv`/deteção automática de rename
  onde possível (extração parcial de um ficheiro para vários não é
  rename puro — Git deteta similaridade parcial mesmo assim).

## 5. Gate ADR-0127

Fluxo contínuo — reorganização de ficheiros de teste, zero mudança de
comportamento, já antecipada por ADR-0037/ADR-0100 como evolução
natural ("decomposição futura só se ratio LOC/coverage justificar" /
"pode separar por cluster se útil"). Não é contrato público, não é
comportamento por defeito do produto, não é fase de pipeline. Sem
paragem humana.

## 6. Critério de fechamento

1. `cargo build --workspace` sucesso.
2. `cargo test -p typst-infra --lib` — mesma contagem total de antes
   (1032), zero falhas novas.
3. `cargo test -p typst-core --lib`/`-p typst-shell --lib` inalterados.
4. `cargo test -p typst-wiring` — mesma falha pré-existente não
   relacionada (P1471), nenhuma nova.
5. `cargo clippy -p typst-infra --all-targets` comparado por contagem
   contra o baseline — zero avisos novos.
6. **`crystalline-lint .` → 0 erros, incluindo o achado `[PARSE]`
   resolvido** (critério de sucesso primário).
7. `crystalline-lint --checks v5,v15,v26 .` → 0 violações (comando
   volta a funcionar, já não bloqueado pelo `[PARSE]`).
8. `git diff --check` sucesso.
9. Retroativamente, `typst-passo-1477.md`/`typst-passo-1478.md` ganham
   nota curta registando que a sanitização (este passo) resolveu o
   bloqueio de selagem — sem reabrir nem alterar as suas próprias
   medições/decisões.

## 7. Execução e resultado (2026-09-22)

### 7.1 Mecânica da extração

Extração precisa por balanceamento de chavetas (script Python,
`sed`/`awk` para localizar limites, nunca edição manual linha-a-linha
de ~1300 linhas). `tests.rs` (11221 linhas) → `tests/mod.rs` (9965
linhas, 230 testes históricos) + três ficheiros extraídos
(`p1475`: 400 linhas/11 testes; `p1476`: 398/9; `p1477`: 494/11).
`builder.rs` (4119 linhas) → `builder.rs` (3881 linhas) +
`tests/p1478_....rs` (251 linhas/6 testes), com três imports
explícitos acrescentados (`PdfStandards`, `AttachedFileRelationship`,
`PagedDocument` — antes vinham de bare-imports de `builder.rs`).
`cargo build --workspace`/`cargo test -p typst-infra --lib --no-run`
aceitaram a estrutura sem nenhum erro logo na primeira tentativa —
confirma que a extração preservou `use super::*`/`super::super::`
corretamente (mesma profundidade de aninhamento antes/depois).

### 7.2 Achado real — a causa-raiz não era tamanho de ficheiro

Depois da extração, `crystalline-lint .` **continuou** a reportar
`[PARSE]`, agora em `tests/p1477_pdfa_icc_version_and_cidset_tests.rs`
(494 linhas) — refutando a hipótese original de P1477/P1478 ("tamanho
acumulado do ficheiro"), já que um ficheiro isolado de 494 linhas não
deveria acionar nenhum limite de tamanho razoável. Bissecção controlada
(truncamento com restauro entre tentativas — nunca `git checkout --`
sobre ficheiro com trabalho real, lição de P1477) isolou a causa a um
identificador local: a variável `raw` em duas funções de teste quase
idênticas (`p1477_cidset_present_in_font_descriptor_and_stream_is_
dense_bitset`/`p1477_cidset_present_for_multifont_path`). Renomear
`raw` → `raw_bytes` (zero outra mudança) fez o achado desaparecer,
confirmado isoladamente e no projeto inteiro. `rustc` sempre aceitou
ambas as formas sem qualquer aviso — não é uma regra de Rust, é um
limite/bug específico do parser do `crystalline-lint`, cuja causa
exata não foi determinada (binário fora do alcance desta sessão para
auditar).

### 7.3 Achado adicional — deduplicação genuína (não só contorno)

As duas funções acima tinham corpos quase idênticos linha-a-linha.
Extraídas para um helper único (`assert_cidset_present_and_dense`),
reduzindo duplicação real — decisão tomada como hipótese intermediária
durante a mesma investigação (antes do achado do `raw`), mantida por
ser melhoria genuína já verificada GREEN. Consequência observável
mínima e positiva: o caminho `build_multifont` ganhou a mesma
verificação de `/Filter /FlateDecode` que só `build_cidfont` tinha
antes (reforço de cobertura — `build_multifont` já usa o mesmo
`build_cidset_stream`, a asserção já era verdadeira, só não testada
nesse caminho).

### 7.4 Achado colateral — 298 avisos pré-existentes, agora visíveis

Com o `[PARSE]` resolvido, `crystalline-lint .` completa pela primeira
vez um scan integral desta sessão e revela ~298 avisos `warning:`
(V12/V16/V17/V18) espalhados pelo projeto inteiro (`04_wiring`,
matemática, cor, etc.) — **confirmados pré-existentes** por medição no
baseline `8380beef5` (anterior a P1475, antes de qualquer trabalho
desta sessão): mesma contagem (298 avisos, exit code 0). Não são
regressão de P1475-P1479, não é âmbito deste passo corrigi-los —
registados aqui só para não ficarem invisíveis outra vez (ADR-0108).
`exit:0` do `crystalline-lint` confirma que nenhum é `fatal:`/`error:`
— o "0 erros" que este e os passos anteriores mediram sempre se referiu
a esse nível de severidade, nunca a "0 avisos" no projeto inteiro.

### 7.5 Correção mecânica final

`builder.rs` ficou com uma linha em branco a mais no EOF (efeito da
truncagem por `head -n`), pega por `git diff --check`. Corrigida.

### 7.6 Gates finais medidos

```text
cargo build --workspace                       -> sucesso
cargo test -p typst-infra --lib                 -> 1032/1032 (mesma
                                                   contagem de antes —
                                                   ADR-0037 Regra 7)
cargo test -p typst-core --lib                  -> 6129/6129
cargo test -p typst-shell --lib                  -> 83/83
cargo test -p typst-wiring                       -> 1 falha pré-existente
                                                    não relacionada
                                                    (P1471, igual à de
                                                    P1475-P1478)
cargo clippy -p typst-infra --all-targets --message-format=json
    comparado por contagem contra baseline
    `d8a5787c7`                                  -> 801=801, zero
                                                    avisos novos reais
                                                    (43 relocados de
                                                    tests.rs para
                                                    tests/mod.rs pelo
                                                    rename; 1 achado
                                                    real corrigido —
                                                    markdown list item
                                                    num `//!` convertido
                                                    de `//`)
crystalline-lint .                                -> exit 0, zero
                                                    fatal:/error:,
                                                    zero [PARSE] em
                                                    todo o projeto
crystalline-lint --checks v5,v15,v26 .            -> "No violations
                                                    found" (comando
                                                    volta a completar
                                                    pela primeira vez
                                                    desde P1477)
git diff --check                                  -> sucesso
```

### 7.7 Veredito final

Critério de fechamento (§6) satisfeito nos 9 itens. `crystalline-lint
.` deixa de reportar `[PARSE]` em todo o projeto — P1477 e P1478
passam de `GREEN/NÃO SELADO` para `MATERIALIZADO` retroativamente
(notas próprias adicionadas a cada um, sem reabrir medição/decisão).
Achado colateral (298 avisos pré-existentes, agora visíveis) registado
para referência futura, fora do escopo de correção deste passo.
