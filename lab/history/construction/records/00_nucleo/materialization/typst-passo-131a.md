# Passo 131A — Diagnóstico para materialização de `Lang`

**Série**: 131A (passo **micro-S** em L0; primeiro de dois
sub-passos para materializar tipo `Lang`).
**Precondição**: Passo 130 encerrado; 1057 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos. `StyleDelta.lang:
Option<EcoString>` captura sem validação — divergência semântica
ADR-0033 em aberto.
**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional com vanilla) — violação actual
  a resolver.
- **ADR-0034** (diagnóstico obrigatório para materialização de
  tipos vanilla) — **este passo é o cumprimento**.
- **ADR-0036** (atomização progressiva) — dependências explícitas.
- **ADR-0037** (coesão por domínio) — localização do tipo.

**Natureza**: passo L0. **Sem código**. **Sem testes**. Produz
documento de diagnóstico + proposta de ADR.

---

## Contexto

O Passo 130 capturou `lang` como `EcoString` raw. Vanilla usa
tipo `Lang` dedicado com parser BCP 47 e validação. O relatório
130 caracterizou a divergência como "semântica suave"; revisão
subsequente reclassificou como **divergência semântica activa**
que viola ADR-0033 (comportamento observável diferente:
`"xx-invalid"` passa silencioso no cristalino, erra no vanilla).

Decisão tomada fora do enunciado: **materializar `Lang` como
tipo L1** com validação e erro hard em valores inválidos —
paridade total com vanilla.

ADR-0034 obriga a produzir diagnóstico estruturado antes de
escrever código. Este passo cumpre essa obrigação.

---

## Contexto estratégico

Sub-passo 1 de 6 para fechar DEBT-1 (roadmap revisto):

- **131A**: diagnóstico `Lang` (este).
- **131B**: materialização `Lang` + migração `StyleDelta.lang:
  Option<EcoString>` → `Option<Lang>` + arm eval com erro hard.
- **132**: `text.font` + substituição canary DEBT-50.
- **133**: activar target `par` em `eval_set_rule`.
- **134**: migrar `leading` de `text` para `par`.
- **135**: fechar DEBT-1 no DEBT.md.

131B só pode ser enunciado depois de 131A encerrar e o
diagnóstico ser aprovado.

---

## Objectivo

Produzir dois artefactos em L0:

1. **Diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`
   com os 7 itens mínimos do ADR-0034.

2. **ADR proposta** (status `PROPOSTO`) em
   `00_nucleo/adr/typst-adr-NNNN-lang-tipo-semantico.md`
   (número a atribuir — 0052 é o candidato).

Ao fim do passo:

- Diagnóstico aprovado (por ti, fora da execução do passo).
- ADR-NNNN em `PROPOSTO` com decisão arquitectural completa mas
  sem código.
- Nenhum teste novo. Nenhum ficheiro de código tocado.
- Enunciado do 131B pode ser escrito com base no diagnóstico.

Este passo **não**:

- Implementa `Lang` em L1.
- Migra `StyleDelta.lang` para o novo tipo.
- Adiciona validação ao arm `"lang"`.
- Toca testes existentes.
- Fecha DEBT-1.

---

## Decisões já tomadas (entrada do passo)

1. **Rejeição em lang inválido**: **erro hard**, replica vanilla.
   ADR-0033 paridade total.
2. **Materialização em L1**: `Lang` vive em L1 (não em L3 nem
   L2). Decisão implícita no ADR-0034 (tipos vanilla
   materializam em L1 por default).
3. **Ambito do diagnóstico**: só `Lang`. **Não** inclui `Region`,
   `Script`, `Dir` — passos separados se/quando necessário.

## Decisões diferidas (para o diagnóstico resolver)

4. **Forma interna de `Lang`**:
   - `struct Lang(EcoString)` — newtype sobre string validada.
   - `enum Lang { ... }` — 100+ variantes enumeradas (improvável).
   - `struct Lang { language: EcoString, region: Option<...>, ... }`
     — decomposto em tags BCP 47.
   - Forma vanilla (registar no diagnóstico).

5. **Validação BCP 47**:
   - Parser completo (script, region, variant, extension).
   - Parser mínimo (só language subtag + region subtag opcional).
   - Regex simples.
   - Crate externa (se autorizada — verificar `[l1_allowed_external]`).

6. **Constantes pré-definidas**:
   - Vanilla tem `Lang::ENGLISH`, etc.? Quais?
   - Se sim, quantas (3, 10, 100)?
   - Replicar todas ou subset?

7. **API de acesso**:
   - `lang.as_str() -> &str` (óbvio).
   - `lang.primary() -> &str` (ex: `"en"` de `"en-GB"`)?
   - `lang.region() -> Option<&str>`?
   - Decidir mínimo necessário para consumer futuro.

8. **Mensagem de erro em inválido**:
   - Forma exacta da mensagem.
   - Hint (ex: "expected BCP 47 code")?
   - Span do erro (toda a chamada `#set text(...)`? só o valor?).

9. **Localização do ficheiro**:
   - `01_core/src/entities/lang.rs` (novo ficheiro dedicado).
   - `01_core/src/entities/text/lang.rs` (subfolder text se
     existe).
   - Consolidar com outros tipos tipográficos (`01_core/src/entities/font_book.rs`
     já tem `FontWeight` — cabe lá)?
   - Decidir com base em ADR-0037 (coesão por domínio) e na
     estrutura actual confirmada em inventário.

---

## Sub-passos

### 131A.1 — Inventário vanilla

**Leitura de `lab/typst-original/`**:

1. `grep -rn "pub struct Lang\|pub enum Lang\|^struct Lang\|^enum Lang" lab/typst-original/`
2. Localizar ficheiro principal de `Lang` — candidato:
   `lab/typst-original/crates/typst-library/src/text/lang.rs`
   (ou `text/mod.rs`).
3. Ler o ficheiro inteiro. Registar:
   - Declaração do tipo (struct/enum, campos, derives).
   - Impl blocos (métodos públicos).
   - `impl FromValue for Lang` — parser.
   - Constantes (`pub const ENGLISH: Lang = ...`).
   - Dependências externas (crates usadas — validar que são L1-safe).
   - Display/Debug/PartialEq/Eq/Hash — traits derivadas.
4. Localizar `TextElem.lang`:
   - Tipo exacto (`Lang`, `Option<Lang>`, algo mais).
   - Default.

### 131A.2 — Inventário cristalino

1. `grep -rn "Lang\b" 01_core/src/` — confirmar que não existe
   tipo com nome colidido.
2. Listar tipos tipográficos L1 existentes:
   - `FontWeight` em `font_book.rs` (confirmado Passo 129).
   - `Length`, outros.
3. Verificar `01_core/Cargo.toml`:
   - `ecow` autorizado (ADR-0018 / ADR-0035).
   - Outras crates eventualmente necessárias.
4. Confirmar que `EcoString` já é o tipo de `Value::Str`
   (ADR-0024).

### 131A.3 — Escrever diagnóstico

Ficheiro:
`00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`.

**Template dos 7 itens mínimos (ADR-0034)**:

```markdown
# Diagnóstico de `Lang` — Passo 131A

**Data**: 2026-MM-DD
**ADR alvo**: ADR-NNNN (a criar em 131A.4) — "Lang como tipo
semântico em L1".
**Motivação**: Passo 130 capturou `lang` como `EcoString` raw,
violando ADR-0033 (divergência semântica activa: `"xx-invalid"`
silent em cristalino vs erro em vanilla). Este diagnóstico
cumpre ADR-0034 antes de materializar.

---

## 1. Localização no vanilla

- Ficheiro: `lab/typst-original/crates/typst-library/src/text/lang.rs`
  (ou confirmar).
- Linhas: [X:Y].
- Outros sítios relevantes: [TextElem.lang, cast! macro, ...].

## 2. Campos / variantes

- Declaração exacta (copiar).
- Se struct: campos e tipos.
- Se enum: variantes e payloads.
- Derives: `Debug, Clone, Copy?, PartialEq, Eq, Hash`.

## 3. Operadores / métodos

- Métodos públicos (assinatura completa).
- `impl FromValue` / `impl IntoValue` — se existe.
- Display/Debug impls.
- Comparação, ordenação.

## 4. Dependências

- Crates externas usadas (`unic-langid`, `icu_locid`, custom
  parser, regex, ...).
- Tipos internos vanilla referenciados.
- Se usa crate externa: verificar se está em
  `01_core/Cargo.toml [l1_allowed_external]`.

## 5. Semântica

- Representação canónica (ex: lowercase, hyphen-separated).
- Regras de validação (BCP 47 subset? full RFC 5646?).
- Casos edge: empty string, apenas region, language sem region,
  script tag, variant tag, private-use.

## 6. Mensagens de erro

- Forma exacta no vanilla.
- Hints.
- Span.

## 7. Divergências propostas para L1

- **Forma interna**: decisão (struct/enum/newtype) + razão.
- **Validação**: nível exacto (full/subset/simples).
- **Constantes**: quais replicar (ou nenhumas).
- **API**: métodos mínimos.
- **Mensagem de erro**: texto proposto.
- **Localização em L1**: ficheiro + razão (coesão ADR-0037).

---

## Itens adicionais (além dos 7 mínimos)

### Impacto em call-sites

- `StyleDelta.lang: Option<EcoString>` → `Option<Lang>`.
- Arm `"lang"` em `eval_set_text` — adaptar para erro hard.
- Testes L1 existentes: 3 do Passo 130 precisam revisão:
  - `eval_set_text_lang_passo_130`: OK, input `"pt"` continua a
    funcionar.
  - `eval_set_text_lang_bcp47_composto_passo_130`: `"en-GB"`
    continua OK; teste mantém-se como canary.
  - `eval_set_text_font_canary_passo_130`: inalterado.
- Testes L3 DEBT-49 rotados no 130 (`alignment`): inalterados.

### Plano de migração para 131B

- Ordem de edição:
  1. Novo ficheiro `entities/lang.rs` com tipo `Lang`.
  2. `entities/mod.rs` expõe `Lang`.
  3. `StyleDelta.lang: Option<EcoString>` → `Option<Lang>`.
  4. Arm `"lang"` adaptado: `Value::Str(s) → Lang::parse(&s) →
     Err → warn/error; Ok → delta.lang = Some(lang)`.
  5. Testes novos para `Lang::parse`.
  6. Testes existentes adaptados.

### Plano de teste para 131B

- Unit tests em `entities/lang.rs`:
  - `lang_parse_simples_aceita_codigos_iso` (ex: "pt", "en", "de").
  - `lang_parse_composto_aceita_bcp47` (ex: "en-GB", "zh-Hant").
  - `lang_parse_inválido_devolve_erro` (ex: "", "xxxx",
    "en_GB" underscore).
  - `lang_display_preserva_canónico`.
- Integration tests em `rules/eval/tests.rs`:
  - `eval_set_text_lang_inválido_emite_erro_hard_passo_131b`.
  - Manter os 3 do 130 (adaptados se necessário).
```

### 131A.4 — Escrever ADR proposta

Ficheiro:
`00_nucleo/adr/typst-adr-NNNN-lang-tipo-semantico.md`.

**Primeiro**: determinar número. `ls 00_nucleo/adr/typst-adr-*.md |
sort | tail -5` — confirmar o último número atribuído. Candidato
base: **ADR-0052** (README listava até 0038 mas o resumo pós-127
mencionou 51 ADRs activas e ADR-0038 com notas; o resumo pós-127
e relatórios subsequentes mencionaram ADRs até 0051).

Template (copiar estrutura das ADRs recentes — ver
`00_nucleo/adr/typst-adr-0038-*.md` e sucessoras):

```markdown
# ⚖️ ADR-NNNN: Lang como tipo semântico em L1

**Status**: `PROPOSTO`
**Data**: 2026-MM-DD
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`

---

## Contexto

[Parágrafo: Passo 130 capturou `lang` como EcoString,
divergência semântica ADR-0033, decisão de materializar tipo
com paridade total vanilla.]

## Decisão

1. Materializar `Lang` em L1 com forma <a definir no
   diagnóstico>.
2. Parser BCP 47 <nível a definir>.
3. Erro hard em valor inválido (paridade vanilla).
4. `StyleDelta.lang: Option<Lang>` substitui `Option<EcoString>`.
5. Localização: <ficheiro a definir>.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Manter `EcoString` raw | XS, zero refactor | Viola ADR-0033 activamente |
| Validar BCP 47 inline sem tipo | S | Dispersa lógica; dívida estrutural |
| **Materializar `Lang`** | Paridade total; coesão por domínio | M, exige migração |
| Crate externa (unic-langid) | Correcto por construção | Dependência nova, pode não estar em allowlist |

## Consequências

**Positivas**:
- ADR-0033 satisfeito.
- Base para consumer futuro (shaping por language).
- Base para extensões (`region`, `script`, `dir`).

**Negativas**:
- Custo de migração (Passo 131B ≈ M).
- Mais 1 tipo em L1 a manter.

**Neutras**:
- `StyleDelta` cresce em complexidade tipada (mas não em número
  de campos).

## Referências

- ADR-0033 (paridade funcional).
- ADR-0034 (diagnóstico obrigatório).
- Passo 130 relatório (divergência detectada).
- Vanilla: `lab/typst-original/...` (link exacto do diagnóstico).
```

---

## Verificação

1. Diagnóstico criado em `00_nucleo/diagnosticos/`.
2. Diagnóstico tem os 7 itens mínimos preenchidos com factos do
   vanilla, não com placeholders.
3. ADR-NNNN criada com `Status: PROPOSTO` e todos os campos
   canónicos do README de ADRs.
4. Número de ADR atribuído correctamente (não colide com
   existentes).
5. Nenhum ficheiro em `01_core/src/`, `03_infra/src/`,
   `02_shell/src/`, `04_wiring/src/` tocado.
6. Nenhum ficheiro de testes tocado.
7. `cargo test --workspace` continua a passar com os mesmos
   1057 resultados.
8. `crystalline-lint` zero violations.

---

## Critério de conclusão

1. `diagnostico-lang-passo-131a.md` existe e tem todos os 7
   itens do ADR-0034 preenchidos.
2. `typst-adr-NNNN-lang-tipo-semantico.md` existe com
   `Status: PROPOSTO`.
3. Diagnóstico referencia a ADR (`ADR alvo:`).
4. ADR referencia o diagnóstico (`Diagnóstico prévio:`).
5. ADR lista alternativas consideradas com prós/contras.
6. Zero ficheiros de código L1/L2/L3/L4 tocados.
7. `cargo test --workspace` inalterado (1057, 6 ignorados).
8. `crystalline-lint` zero violations.
9. Relatório 131A.E escrito.

---

## O que pode sair errado

- **Vanilla usa crate externa não autorizada em L1**
  (ex: `unic-langid`, `icu_locid`): decisão arquitectural extra
  — adicionar à allowlist (ADR nova de autorização) ou
  implementar parser interno. Diagnóstico regista; ADR proposta
  enfrenta o problema.

- **`Lang` vanilla tem 100+ constantes**: replicar todas é
  grande trabalho. Decidir subset pragmático (PT, EN, DE, FR,
  ES, IT, ZH, JA, ...) ou nenhumas (só parser). Documentar no
  diagnóstico.

- **`Lang` vanilla tem campos privados com lógica complexa**:
  se a semântica interna não é extractível sem refactor grande,
  diagnóstico recomenda adiar materialização e abrir DEBT
  específico. Escalar antes de continuar.

- **Número de ADR incorrecto**: se eu (o enunciado) assumi 0052
  mas o número certo é outro, o executor corrige e regista no
  relatório. Baixo risco.

- **ADR-0038 pode precisar de nota de inter-relação**:
  materializar `Lang` altera `StyleDelta` novamente, depois de
  3 notas já acumuladas. Avaliar no diagnóstico se ADR-0038
  ganha quarta nota referenciando a nova ADR.

---

## Notas operacionais

- Este é o primeiro passo L0-puro da série recente (126–130 foram
  todos L1). Output é documento, não código. O valor é o
  diagnóstico correcto, não a velocidade.

- ADR-0034 existe precisamente para forçar leitura atenta do
  vanilla antes de código. Resistir à tentação de adivinhar
  campos sem olhar.

- Se o diagnóstico revela que materializar `Lang` é **mais
  complexo do que M** (ex: depende de tipos não materializados
  como `Region`, `Script`), **parar e reportar**. 131B pode
  precisar de ser desdobrado em 131B + 131C, ou pode precisar
  de passo prévio de materialização de `Region`.

- A ADR em `PROPOSTO` não é "em vigor" — o vocabulário do
  README dos ADRs deixa claro: `PROPOSTO` = decisão tomada mas
  não implementada. Passa a `IMPLEMENTADO` quando 131B
  encerrar.

- **Não perseguir perfeição** no diagnóstico. 7 itens mínimos,
  factuais, testáveis. Detalhes de estilo de código deixam-se
  para 131B.
