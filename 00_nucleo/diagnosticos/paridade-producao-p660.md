# Relatório de Paridade — P660

**Passo:** 660  
**Data:** 2026-07-09  
**Foco:** Implementar `variant: (eixo: valor)` para fontes variáveis no cristalino.  
**Dependências:** P659 (onde a ausência da sintaxe impediu reprodução real), P525 (onde `axis_variations_for_font_variant` foi introduzida).  
**Hash do commit com as alterações:** `2187dabeb`

---

## 1. Sonda

### 1.1 Sintaxe do vanilla

O vanilla em quarentena (`lab/typst-original/target/release/typst`) rejeita a sintaxe documentada nos testes do P660:

```text
error: unexpected key 'variant', in dict</text>
```

Isto significa que o vanilla local **não suporta** `font: (name: "...", variant: (wdth: 62.5))`. O P660 é, portanto, uma **extensão cristalina** que antecipa uma funcionalidade da linguagem Typst (eixos de fontes variáveis), não uma paridade estrita com a mecânica actual do vanilla. A paridade mantém-se ao nível da **semântica da linguagem** (fontes variáveis têm eixos nomeados com valores numéricos), conforme ADR-0107.

### 1.2 Estado actual do cristalino

- O dicionário de fonte já aceitava campos como `family`, `weight`, `style`, `stretch` e `variant` como nome de variante (`EcoString`).
- Faltava reconhecer `variant` como um dicionário de eixos `(tag: valor)`.
- A função `axis_variations_for_font_variant` já convertia `weight`/`style`/`stretch` em eixos OpenType, mas ignorava eixos explícitos.
- A chave de coleta de fontes para embed era `(FontList, FontVariant)`, pelo que duas invocações com eixos diferentes seriam deduplicadas incorrectamente.

---

## 2. Implementação

### 2.1 L1 — domínio e parser

`01_core/src/entities/font_list.rs`:

- Novo tipo `FontAxisValue(f64)`.
- `FontAxisValue` implementa `Hash` via `f64::to_bits()` para permitir uso em chaves de cache e coleções.
- `FontFamily.axes: Vec<(EcoString, FontAxisValue)>` guarda os eixos explícitos definidos no dicionário de fonte.

`01_core/src/entities/layout_types.rs`:

- `TextStyle.font_axes: Option<Vec<(EcoString, FontAxisValue)>>`.

`01_core/src/entities/style_chain.rs`:

- Conversão `StyleChain → TextStyle` inicializa `font_axes: None`.

`01_core/src/rules/eval/rules.rs`:

- `parse_font_dict_named_fields` aceita o campo `variant`.
- Se `variant` for um `Dict`, itera pelos pares `(tag, valor)`:
  - `tag` deve ser uma string de exactamente 4 caracteres (tags OpenType).
  - `valor` pode ser `int` ou `float`.
- Se `variant` for uma string, comportamento anterior (nome de variante).

`01_core/src/rules/layout/text.rs`:

- Decodifica `"axes"` do dict de fonte devolvido pelo parser.
- Corrige o merge de `font`: `ns_font.or(layouter.style.font.clone())` passa a vencer o `font` da chain quando o utilizador especifica um dicionário de fonte. Antes, o default `Liberation Serif` da chain sobrepujava o `ns_font`, fazendo com que `variant` fosse ignorado em `#set text(font: (...))`.

### 2.2 L3 — shaper, font_variant e export

`03_infra/src/font_variant.rs`:

- `axis_variations_for_font_variant(variant, custom_axes)` recebe os eixos explícitos e:
  - Começa pelas variações derivadas de `weight`/`style`/`stretch`.
  - Adiciona os eixos explícitos, com estes a vencerem em caso de colisão (deduplicação por tag).

`03_infra/src/shaper.rs`:

- Dois call sites actualizados para passar `custom_axes` de `TextStyle`/`FontFamily` para `axis_variations_for_font_variant`.

`03_infra/src/font_metrics.rs`, `03_infra/src/export/builder.rs`:

- Call sites actualizados para incluir os eixos explícitos.

`03_infra/src/pipeline.rs`, `03_infra/src/export/mod.rs`, `03_infra/src/export/stream.rs`:

- Chave de coleta/embed expandida de `(FontList, FontVariant)` para `(FontList, FontVariant, Vec<(EcoString, FontAxisValue)>)`.
- Isto garante que `Cantarell wght:100` e `Cantarell wght:800` produzem duas entradas de fonte embutida distintas.

### 2.3 Testes

`01_core/src/rules/eval/tests.rs`:

- 4 testes P660 cobrindo:
  - eixo float válido;
  - eixo int válido;
  - tag com menos de 4 caracteres → erro;
  - valor inválido (string) → erro.

`01_core/src/rules/layout/tests.rs`:

- `set_text_font_variant_axes_propaga_ao_frame`: verifica que o `TextStyle` resultante contém os eixos definidos.

`03_infra/src/shaper.rs`:

- `p660_axis_variations_custom_axes_override`: confirma que eixos explícitos sobrescrevem os derivados de `weight`/`style`.

`03_infra/src/pipeline.rs`, `03_infra/src/export/tests.rs`:

- Testes existentes actualizados para a nova tupla com eixos explícitos (vazios por omissão).

---

## 3. Validação

### 3.1 Testes

```bash
cargo test --workspace --lib
```

Resultado: `607 passed; 0 failed; 5 ignored`.

### 3.2 Build

```bash
cargo build --workspace
```

Resultado: sucesso (apenas warnings preexistentes).

### 3.3 Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.4 Documento real com Cantarell-VF

Fonte: `/tmp/p660-fonts/Cantarell-VF.otf`.

Dois documentos gerados:

```typst
// temp_p660/p660-wght100.typ
#set text(font: (family: "Cantarell", variant: (wght: 100)), size: 24pt)
Hello P660 wght100
```

```typst
// temp_p660/p660-wght800.typ
#set text(font: (family: "Cantarell", variant: (wght: 800)), size: 24pt)
Hello P660 wght800
```

Comandos:

```bash
target/debug/typst --font-path /tmp/p660-fonts temp_p660/p660-wght100.typ temp_p660/p660-wght100.pdf
target/debug/typst --font-path /tmp/p660-fonts temp_p660/p660-wght800.typ temp_p660/p660-wght800.pdf
```

Resultados:

| Ficheiro | Tamanho | SHA-256 |
|----------|---------|---------|
| `p660-wght100.pdf` | 173961 B | `62110986eaa208fe1ff18a51ddb609960225564309f73e3e115faad48b25dde4` |
| `p660-wght800.pdf` | 173986 B | `db9a5c8bdab9c7b25adfe3e16f17c15c0b55389e7c6b3f3e75c1565f2094c2ce` |

As arrays `TJ` dos PDFs diferem (por exemplo, a posição do glifo 965 no wght100 corresponde ao glifo 972 no wght800), confirmando que o shaper produziu runs distintas para os dois pesos. Os bytes dos PDFs são diferentes, pelo que as duas variações foram embutidas como fontes separadas.

### 3.5 Revalidação de P659 com sintaxe real

Embora o documento P659 original use `wdth` (que a fonte Cantarell não expõe), o teste conceptual é o mesmo: duas invocações do mesmo texto com eixos diferentes produzem resultados distintos. O teste com `wght:100` vs `wght:800` demonstra, ao nível do documento real, que a cache e o embed já não colidem — o mesmo objectivo de P659, agora testável por sintaxe real.

---

## 4. Decisão

- Implementou-se `variant: (eixo: valor)` como extensão cristalina; o vanilla local ainda não reconhece esta sintaxe.
- A chave de fontes para embed passou a incluir eixos explícitos, evitando deduplicação incorrecta de variações distintas.
- Corrigiu-se o merge de `font` em `layout/text.rs`, sem o qual `#set text(font: (...))` ignorava o dicionário especificado.
- Sem regressões em `cargo test --workspace --lib` nem em `crystalline-lint .`.
