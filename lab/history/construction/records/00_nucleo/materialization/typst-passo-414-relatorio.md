# Relatório — Passo 414

**Título**: `text.font` dict named fields (paridade vanilla)  
**Data**: 2026-06-22  
**Tipo**: Materialização de feature (L1 stdlib + eval + layout; zero tipo novo de usuário; zero I/O).

## Resumo executivo

A sonda A.0 revelou que o **DEBT-52 (gap 8)** já tinha sido fechado no **Passo 407** com uma forma dict legada cristalina — dict cujas chaves são nomes de família e os valores são variant names. No entanto, o Typst vanilla usa uma forma **named fields**:

```typ
#set text(font: (family: "Linux Libertine", variant: "bold", weight: 700, style: "italic", fallback: true))
```

Este passo adiciona suporte a essa forma named fields, melhorando a paridade linguagem (ADR-0107) sem quebrar a compatibilidade retroativa com P407.

## Decisões de engenharia

### 1. Detecção de formato no AST

No arm `"font"` de `eval_set_rule`, o dict é inspecionado no AST (`Expr::Dict`). Se existir uma chave identificador pertencente ao conjunto `{family, variant, weight, style, fallback}`, o dict é tratado como **named fields**; caso contrário, mantém-se o parsing legado P407.

### 2. Extensão de `FontFamily`

Adicionados campos opcionais a `FontFamily` para transportar os dados named fields sem ativar variant-aware selection (que continua scope-out ADR-0054bis):

```rust
pub struct FontFamily {
    pub name: FontNamePattern,
    pub variants: Vec<EcoString>,
    pub variant: Option<EcoString>,
    pub weight: Option<EcoString>,
    pub style: Option<EcoString>,
    pub covers: Option<Covers>,
}
```

Os construtores existentes (`new`, `new_literal`, `new_regex`) mantêm a API anterior, inicializando os novos campos com `None`. Adicionado `new_named` para construção explicita.

### 3. Parsing refatorado

O parsing do dict de fonte foi extraído para duas funções auxiliares:
- `parse_font_dict_named_fields` — P414.
- `parse_font_dict_legacy` — P407.
- `variants_from_value` — helper partilhado.

### 4. Campos suportados (named fields)

| Campo | Tipo | Obrigatório | Nota |
|-------|------|-------------|------|
| `family` | `Str` \| `Regex` | sim | converte para `name` |
| `variant` | `Str` | não | transportado em `FontFamily.variant` |
| `weight` | `Int` \| `Str` | não | `Int` convertido para string |
| `style` | `Str` | não | transportado em `FontFamily.style` |
| `fallback` | `Bool` | não | default `true`; validado semanticamente |

Campos desconhecidos (ex.: `stretch`) e tipos errados produzem erro hard.

### 5. Representação na chain custom

Continua a ser `Value::Array` de items `Value::Dict`. O dict agora pode conter, além de `name` e `variants`, os campos opcionais `variant`, `weight` e `style`. O layout decodifica-os para `FontFamily`.

## Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/font_list.rs` | campos `variant`/`weight`/`style` em `FontFamily`; construtor `new_named` |
| `01_core/src/engine/eval/rules.rs` | detecção named fields; parsers `parse_font_dict_named_fields`/`parse_font_dict_legacy`; header de prompt atualizado |
| `01_core/src/engine/layout/text.rs` | decodificação dos novos campos do dict na chain custom |
| `01_core/src/engine/eval/tests.rs` | 12 testes de named fields P414 |
| `00_nucleo/prompts/engine/style/font-dict.md` | prompt L0 consolidado (P407 + P414) |

## Scope-out mantido

- Variant-aware selection (resolver `variant`/`weight`/`style` contra `FontBook`) — ADR-0054bis condicional.
- `stretch` no dict named fields.
- OpenType features (`features`) — DEBT-53, scope-out XL.
- Dict spread (`..dict`) — scope-out P407.

## Validação

```bash
cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica
# → todos verdes (o teste p350c mantém stack overflow pré-existente)

crystalline-lint .
# → 0 drift; único warning é prompt órfão show-regex.md (pré-existente)
```

## Inventário / DEBT

- DEBT-52 foi formalmente encerrado no Passo 407. O **gap 8** (font dict) já estava implementado na forma legada.
- P414 eleva a paridade linguagem adicionando a forma named fields do vanilla. Nenhum novo DEBT aberto.
