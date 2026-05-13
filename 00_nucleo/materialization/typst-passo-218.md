# Passo 218 — `native_columns` stdlib + `extract_count` helper

**Série**: 218 (quarto sub-passo materialização Layout
Fase 3; segundo sub-fase (b) DEBT-56; aditivo trivial à
stdlib).
**Marco**: nenhum (sétimo passo pós-M9c; aditivo stdlib).
**Tipo**: aditivo trivial à stdlib registando
`native_columns` paridade mecânica P156I/J.
**Magnitude**: S (~1h).
**Pré-condição**: P217 concluído (`Content::Columns { count:
usize, gutter: Option<Length>, body: Box<Content> }`
materializado; arms exhaustivos em 4 ficheiros L1; 1952
tests verdes; stub transparente em Layouter); ADR-0078
PROPOSTO anotada P217; humano fixou Caminho 1 (P218
imediatamente per recomendação subjectiva relatório P217 §8).
**Output**: 1 ficheiro relatório curto + código alterado em
`stdlib/layout.rs` + L0 `stdlib.md` extensão minimal (1
linha tabela) + ADR-0078 anotada (sem transição de status).

---

## §1 Trabalho

P217 adicionou variant `Content::Columns` ao enum mas sem
stdlib. P218 regista `native_columns` em
`stdlib/layout.rs::make_stdlib` permitindo `#columns(2)[body]`
ou `#columns(2, gutter: 1em)[body]` parsearem como
`Value::Content(Content::Columns { count, gutter, body })`.

**Aditivo trivial paridade mecânica** com 10 stdlib funcs
registadas pós-P156C (pad, hide, h, v, pagebreak, skew,
block, box, stack, repeat). Não-objectivos explícitos:
consumer multi-region real (P219), `Content::Colbreak`
(P220), `Content::Columns` reclassificação em §A.5
(P221 encerramento).

**Decisão central de P218**: assinatura paridade vanilla
`columns(count: usize, body, gutter: ?)`:
- `count`: **posicional obrigatório** (não Option;
  diferença vs P157B TableCell.x/y opcionais).
- `body`: **posicional obrigatório** (Content ou Str
  como em P156J `Repeat.body`).
- `gutter`: **named opcional** (Option<Length>; ADR-0064
  Caso C).

Validações:
- `count >= 1` rejeita `count = 0` per ADR-0054 graded
  (paridade `NonZeroUsize` vanilla).
- `gutter` negativo rejeitado (paridade `Stack.spacing`
  P156I).
- Named arg desconhecido rejeitado.
- Body não-Content/não-Str rejeitado.

Helper novo `extract_count(args, fn_name)` — paridade
`extract_usize_or_none_min` P157B mas para **posicional
obrigatório**:
- Lê `args.items[0]`.
- Aceita `Value::Int(n)` com `n >= 1`.
- Rejeita `n < 1`, tipos não-Int, ausência.
- Não-Option porque count é obrigatório.

Reuso de dados (sem recolha nova):

- P217 `Content::Columns` variant em
  `01_core/src/entities/content.rs`.
- `extract_length` helper N=7 (P156C/D/G/H/I/J/L) — N=8
  cumulativo pós-P218 via `gutter`.
- Precedente `native_repeat` (P156J) para forma
  `(body, gap: ?, justify: ?)` — base estrutural.
- ADR-0078 PROPOSTO anotada P217.
- ADR-0064 Caso C cumulativo (já aplicado a `gutter` em
  variant P217; P218 valida em stdlib).

---

## §2 Cláusulas (10)

### C1 — Inventário pré-P218: confirmar `make_stdlib` actual

Auditoria empírica:

```
grep -c "native_" 01_core/src/rules/stdlib/mod.rs
grep -c "native_" 01_core/src/rules/stdlib/layout.rs
grep "register\|insert" 01_core/src/rules/stdlib/layout.rs | tail -20
```

Hipótese: ~53 stdlib funcs registadas (Tabela B inventário
148 declara ~53 pós-M9c). Confirmar local exacto de
`native_repeat` registado (`stdlib/layout.rs::make_stdlib`)
— P218 adiciona `native_columns` imediatamente após.

Se contagem divergir: registar `P218.div-1`.

### C2 — Helper `extract_count(args, fn_name)`

Adicionar helper privado em `01_core/src/rules/stdlib/layout.rs`:

```rust
/// Helper P218: extrai `count: usize` posicional obrigatório
/// para `native_columns`. Rejeita `count = 0` (paridade
/// `NonZeroUsize` vanilla per ADR-0054 graded).
///
/// **Posicional obrigatório** (diferente de
/// `extract_usize_or_none_min` P157B que era named opcional).
fn extract_count(args: &Args, fn_name: &str) -> SourceResult<usize> {
    match args.items.first() {
        Some(Value::Int(n)) => {
            if *n < 1 {
                return Err(eco_format!(
                    "{}(count): count deve ser >= 1, recebeu {}",
                    fn_name, n
                ));
            }
            Ok(*n as usize)
        }
        Some(other) => Err(eco_format!(
            "{}(count): espera Int, recebeu {}",
            fn_name, value_type(other)
        )),
        None => Err(eco_format!(
            "{}: argumento posicional count obrigatório ausente",
            fn_name
        )),
    }
}
```

**Decisão fixada em C2**: helper `extract_count` privado em
`stdlib/layout.rs`. Promoção a `pub(super)` ou helper público
diferida até N=2-3 reuso (política consistente com
`extract_tracks` P157A, `extract_usize_or_none_min` P157B,
`extract_bool_with_default` P157C — todos privados aguardando
reuso).

**Hipótese provável**: `extract_count` é único uso em P218.
N=1 pós-P218. Refactor para reusar em features futuras
similares (e.g. `range(n)` em foundations) é candidato
diferido.

### C3 — `native_columns` function

Adicionar em `01_core/src/rules/stdlib/layout.rs`:

```rust
/// Stdlib `columns(count, body, gutter: ?)` — Layout Fase 3
/// per ADR-0078 PROPOSTO.
///
/// Forma: `#columns(2)[body]` ou
/// `#columns(2, gutter: 1em)[body]`.
///
/// Aditivo P218 — arm Layouter é stub transparente em P217
/// (consumer real P219 sub-fase (b) DEBT-56).
pub fn native_columns(
    _ctx: &mut EvalContext<'_>,
    args: &Args,
) -> SourceResult<Value> {
    // 1. Extract count (posicional obrigatório).
    let count = extract_count(args, "columns")?;

    // 2. Extract body (posicional [1], Content ou Str).
    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.clone()),
        Some(other) => return Err(eco_format!(
            "columns(body): espera Content ou Str, recebeu {}",
            value_type(other)
        )),
        None => return Err(eco_format!(
            "columns: argumento posicional body obrigatório ausente"
        )),
    };

    // 3. Validate no extra positionals.
    if args.items.len() > 2 {
        return Err(eco_format!(
            "columns: aceita 2 posicionais (count, body), recebeu {}",
            args.items.len()
        ));
    }

    // 4. Extract gutter (named opcional, Option<Length>).
    let gutter = match args.named.get("gutter") {
        Some(val) => Some(extract_length(val, "columns", "gutter")?),
        None => None,
    };
    if let Some(g) = &gutter {
        if g.is_negative() {
            return Err(eco_format!(
                "columns(gutter): negativo rejeitado"
            ));
        }
    }

    // 5. Reject unknown named args.
    for key in args.named.keys() {
        if key != "gutter" {
            return Err(eco_format!(
                "columns: named arg desconhecido `{}` (esperado: gutter)",
                key
            ));
        }
    }

    // 6. Build variant.
    Ok(Value::Content(Content::Columns {
        count,
        gutter,
        body: Box::new(body),
    }))
}
```

Magnitude isolada: XS (~10min).

### C4 — Registar `native_columns` em `make_stdlib`

Editar `01_core/src/rules/stdlib/layout.rs::register_layout_natives`
(ou função análoga) adicionando linha:

```rust
scope.insert("columns", Value::Func(Func::native(
    "columns",
    native_columns,
)));
```

Posição: imediatamente após `native_repeat` registo (paridade
ordem ADR-0061 Fase 3 — `repeat` é sub-passo 1; `columns` é
sub-passo seguinte).

Stdlib funcs count: ~53 → 54.

### C5 — Sentinelas P218

Tests unitários P218 em `stdlib/layout.rs::tests` (paridade
P156I native_stack / P156J native_repeat):

- `p218_native_columns_count_valido_sem_gutter` — `columns(2)
  [text]` produz `Content::Columns { count: 2, gutter: None,
  body }`.
- `p218_native_columns_count_zero_rejeita` — `columns(0)
  [text]` falha com mensagem clara "count deve ser >= 1".
- `p218_native_columns_count_negativo_rejeita` — `columns(-1)
  [text]` falha.
- `p218_native_columns_count_nao_int_rejeita` — `columns("a")
  [text]` falha.
- `p218_native_columns_count_ausente_rejeita` — `columns()`
  falha.
- `p218_native_columns_body_ausente_rejeita` — `columns(2)`
  sem body falha.
- `p218_native_columns_gutter_length_aceita` — `columns(2,
  gutter: 1em)[text]` produz `gutter: Some(Length::em(1.0))`.
- `p218_native_columns_gutter_negativo_rejeita` —
  `columns(2, gutter: -1em)[text]` falha.
- `p218_native_columns_named_arg_desconhecido_rejeita` —
  `columns(2, foo: bar)[text]` falha.
- `p218_native_columns_extra_positional_rejeita` —
  `columns(2, [a], [b])` falha (>2 posicionais).

Total: **10 stdlib unit tests** P218.

Layout E2E test (1 test em `tests.rs`):
- `p218_columns_stdlib_parseado_renderiza_body` —
  `eval(#columns(2)[hello])` parseia + `layout` produz
  PagedDocument com `plain_text() == "hello"` (arm
  transparente P217 preservado).

Total tests P218: **10 unit + 1 E2E = 11 tests**.
Esperado pós-P218: **1952 + 11 = 1963 verdes**.

### C6 — L0 `stdlib.md` extensão minimal

Editar `00_nucleo/prompts/rules/stdlib.md` adicionando linha
em tabela "Funções Nativas Registadas":

```markdown
| `native_columns` | `columns(count, body, gutter?)` | `Content::Columns` (Fase 3 Layout per ADR-0078 PROPOSTO) |
```

**Decisão sobre extensão L0**:
- **Opção α** — linha minimal em tabela (este passo).
- **Opção β** — secção dedicada com semantic detalhada.
- **Opção γ** — sem extensão L0 (paridade P217 que decidiu
  inline-doc no .rs).

**Hipótese provável**: **Opção α** — linha minimal preserva
inventário stdlib visível em L0 sem inflação. Decisão
empírica do P217 (não tocar L0 content.md) foi específica
ao variant; stdlib registo é diferente — tabela L0 stdlib.md
é o local canónico de inventário.

Hash propagado via `crystalline-lint --fix-hashes`.

### C7 — Verificação tests workspace

Critério: 1952 verdes pré-P218 + 11 novos = **1963 verdes**.

```
cargo test --workspace 2>&1 | tail -20
```

**Erro tolerado**: zero. Qualquer test pre-existente red
indica regressão (P218 é aditivo puro — registo novo da
stdlib).

Hipótese provável: 1963 verdes. Aditivo trivial em isolated
function; sem refactor estrutural.

### C8 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hash propagado em
`rules/stdlib.md` (L0) + `rules/stdlib/layout.rs` (L1).

Esperado: 1-2 hashes drift initial (stdlib.md + layout.rs);
zero pós-`--fix-hashes`.

### C9 — Inventário 148 anotação progresso

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**§A.5 Layout linha `columns(n)`**: **mantém-se `ausente`**.
Reclassificação a `implementado` ocorre só pós-P219+P220
(consumer real + colbreak). Anotação inline ampliada:
"P217 variant + arms; P218 stdlib registada; consumer
real diferido P219".

**Tabela B.2 Content**: actualização **diferida a P221**
encerramento série (consistente com decisão empírica P217).

**Sem footnote nova em P218**. Anotação cumulativa P217+P218
fica em ADR-0078 + L0 stdlib.md.

### C10 — ADR-0078 anotação P218

`00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
§"Plano de materialização" anotado com bloco
**`### P218 materializado 2026-05-12`**:

```markdown
Sub-fase (b) DEBT-56 — segundo sub-passo aditivo trivial:
- `native_columns(count, body, gutter: ?)` registada em
  `stdlib/layout.rs::make_stdlib` (stdlib funcs ~53 → 54).
- Helper `extract_count(args, fn_name)` novo para `count`
  posicional obrigatório (paridade `extract_usize_or_none_min`
  P157B mas para posicional; N=1 pós-P218).
- Validações: count >= 1, gutter negativo rejeitado, named
  arg desconhecido rejeitado, body Content/Str obrigatório.
- 11 tests adicionados (10 unit + 1 E2E). Tests workspace:
  1952 → 1963 verdes.
- ADR-0064 Caso C cumulativo via `gutter` (já materializado
  em P217 variant; P218 valida em stdlib).
- `extract_length` reuso cumulativo N=7 → 8 (P218 via
  `gutter`).

Status ADR-0078: PROPOSTO mantido. 3 sub-passos restantes
(P219 consumer multi-column real, P220 colbreak, P221
fecho).
```

**Status**: PROPOSTO mantido. Não transita ainda.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-218-relatorio.md`.

Estrutura (~5-7 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P218 stdlib funcs registadas (C1).
- §3 Helper `extract_count` novo (C2; N=1).
- §4 `native_columns` function + registo (C3 + C4;
  count/body posicionais + gutter named).
- §5 Decisões substantivas (count posicional vs Option;
  promoção helper diferida; L0 stdlib.md linha minimal vs
  secção; helpers reuso cumulativo N=7→8).
- §6 Resultados verificação (tests + lint).
- §7 Inventário 148 anotação progresso + ADR-0078 anotação
  P218.
- §8 Próximo sub-passo (P219 consumer multi-column real;
  Caminho 1 continuação).

Código alterado:
- **Editado**: `01_core/src/rules/stdlib/layout.rs` (+
  helper `extract_count` ~25 LOC + `native_columns` ~50
  LOC + registo ~3 LOC + 10 unit tests ~80 LOC).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+ 1
  E2E test ~10 LOC).
- **Editado**: `00_nucleo/prompts/rules/stdlib.md` (+ 1
  linha tabela "Funções Nativas Registadas").
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (+ anotação P218).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Consumer multi-region real no Layouter — diferido P219.
- `Content::Colbreak` variant — diferido P220.
- Reclassificar §A.5 inventário 148 `columns` ausente →
  implementado — só pós-P219+P220.
- Show rules `#show columns: ...` — fora de escopo Fase 3.
- Mudança observable em features existentes (P218 é
  aditivo puro).
- Promover ADR-0078 → IMPLEMENTADO — só P221.
- Fechar DEBT-56 — só P221.
- Promover `extract_count` a helper público — diferido
  N=2-3 reuso futuro.
- Adicionar fields adicionais ao variant `Content::Columns`
  — variant fixado em P217 (3 fields).
- Tocar em `Layouter` ou `Region`/`Regions` — P218 é
  stdlib puro.

---

## §5 Riscos a evitar

1. **Forma incorrecta da assinatura**: tentação de fazer
   `count: Option<usize>` (paridade P157B). Rejeitada —
   `count` é posicional obrigatório vanilla; sem default
   sensato (0 colunas é absurdo). `extract_count` novo é
   helper específico.
2. **Validação `count >= 1` em construtor Rust em vez de
   stdlib**: tentação de validar em
   `Content::columns(body, count, gutter)`. Rejeitada per
   spec P217 §C4 — validação é da stdlib (P218);
   construtor Rust aceita `count = 0` como caso degenerate
   (consumer P219 pode tratar como passthrough).
3. **`extract_length` em vez de helper novo para `count`**:
   `extract_length` retorna `Length` não `usize`. Helper
   novo é necessário.
4. **`gutter` validação em construtor Rust**: idem item 2.
   Validação `is_negative()` na stdlib.
5. **Esquecer `expect_no_named` ou rejeitar named arg
   desconhecido**: paridade pattern P156D-J. Rust não
   detecta; precisa loop manual.
6. **Tests P218 que dependem de consumer real**: tests
   verificam só **parsing + extract → variant**. Não
   verificam comportamento multi-column real (P219).
   Critério de transparência stub preservado de P217.
7. **Promover helper público prematuramente**: tentação
   de fazer `extract_count` `pub(super)`. Rejeitada —
   N=1 pós-P218; promoção diferida a N=2-3.
8. **L0 extensão excessiva**: tentação de criar secção
   dedicada em stdlib.md (Opção β). Rejeitada — linha
   minimal em tabela (Opção α) preserva pattern visível;
   detalhe técnico fica em ADR-0078.
9. **Mudança observable acidental**: P218 é aditivo
   puro. Hipótese provável: zero quebras em tests
   pre-existentes.
10. **Posição do registo em `make_stdlib`**: tentação
    aleatória. Posição correcta: imediatamente após
    `native_repeat` (ordem ADR-0061 Fase 3 sub-passo
    natural).

---

## §6 Hipótese provável

C1 confirmará ~53 stdlib funcs registadas (paridade Tabela
B inventário 148).

C2 criará `extract_count` privado em ~25 LOC; N=1 pós-P218.

C3 criará `native_columns` em ~50 LOC com 6 validações
(count, body, extra positional, gutter Length, gutter
negativo, named arg desconhecido).

C4 registará `native_columns` em `make_stdlib`; stdlib
funcs ~53 → 54.

C5 criará 10 unit tests + 1 E2E test (paridade P156I
native_stack — também tinha ~10 testes).

C6 estenderá stdlib.md com 1 linha minimal.

C7 reportará 1963 tests verdes (1952 + 11).

C8 reportará 0 violations pós-fix-hashes.

C9 manterá §A.5 ausente; Tabela B.2 diferida a P221.

C10 anotará ADR-0078 cumulativo.

Custo real: S (~1h). Maior parcela em C5 (10 unit tests
+ 1 E2E) + C8 (lint hashes propagação).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P218

P218 é estruturalmente distinto na trajectória pós-M9c:

- **Segundo sub-passo sub-fase (b) DEBT-56** — pós-P217
  variant. Aditivo trivial.
- **Anti-inflação 13ª aplicação cumulativa** pós-P205D —
  separação P217 (variant) + P218 (stdlib) + P219 (consumer)
  por atomização ADR-0036 cumprida.
- **Padrão "extract_length" reuso N=7 → 8** — `gutter:
  Option<Length>` em P218 valida via helper canónico.
  Patamar N=8 reforça candidatura a promoção a helper
  público (refactor escopo XS futuro).
- **Helper novo `extract_count` N=1** — paridade pattern
  P157B `extract_usize_or_none_min` (parametrizado), mas
  específico para posicional obrigatório (não Option).
  Possível reuso futuro em features similares
  (e.g. `range(n)` posicional).
- **Pattern "stub transparente" preservado N=1** — P218
  não muda Layouter; arm continua transparente. Pattern
  N=2 só em P220 (Colbreak também stub transparente até
  P221 fecho).
- **Stdlib funcs registadas pós-Layout Fase 3 sub1-2**:
  10 (Fase 1+2 P156C-I) + 2 (Fase 3 P156J + P218) = **12
  funcs Layout registadas**. Cobertura stdlib Layout
  ~67% (12/18) — preserva 78% cobertura factual via
  arm/variant materializados.

Por isso §5 risco 7 (promover helper prematuro) é o mais
provável. Tentação óbvia é "extract_count é claro candidato
a public helper". Defesa: política consistente N=2-3 mínima
(precedente `extract_length` N=7 ainda privado; `extract_tracks`
N=2 promovido a `pub(super)` mas não público).

**Critério de aceitação P218**:
- 11 tests novos verdes (10 unit + 1 E2E).
- 1952 tests pre-existentes preservados verdes (= 1963
  total).
- 0 violations.
- Stdlib funcs registadas ~53 → 54.
- Sub-fase (b) DEBT-56: 1/4 → 2/4 (segunda atomização).
