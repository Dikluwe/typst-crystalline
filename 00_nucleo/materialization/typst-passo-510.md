---

# P510 — Materialização de Math Styles: 12 Funções de Estilo Tipográfico

> **Passo:** 510
> **Data:** 2026-06-30
> **Foco:** Fechar empiricamente as 12 funções de estilo matemático ausentes no cristalino: `bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright`. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via corpus P490+P500.
> **Tamanho:** M (~45 min de implementação + 15 min de validação).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P509 (stdlib core fechado, 37/37 OK), P490 (math base funciona).

---

## 1. Contexto

O P509 fechou o corpus P490+P500 em **37/37 OK**. O diagnóstico P508 identificou que **12 funções de estilo math** estão ausentes no cristalino:

| Função | Estilo | Mapeamento Unicode | Exemplo |
|--------|--------|-------------------|---------|
| `bb(x)` | Blackboard bold (double-struck) | U+1D538–U+1D56B (Mathematical Alphanumeric Symbols) | `𝕏` |
| `bold(x)` | Negrito | U+1D400–U+1D433 | `𝐗` |
| `cal(x)` | Caligráfico | U+1D49C–U+1D4CF | `𝒳` |
| `frak(x)` | Fraktur | U+1D504–U+1D537 | `𝔛` |
| `italic(x)` | Itálico | U+1D434–U+1D467 | `𝑋` |
| `mono(x)` | Monoespaçado | U+1D670–U+1D6A3 | `𝚇` |
| `sans(x)` | Sans-serif | U+1D5A0–U+1D5D3 | `𝖷` |
| `scr(x)` | Script (cursivo) | U+1D4B0–U+1D4E3 | `𝓍` |
| `script(x)` | Alias para `scr` | Idem `scr` | `𝓍` |
| `serif(x)` | Serif (normal) | U+0041–U+007A (ASCII) | `X` |
| `sscript(x)` | Sans-serif script | U+1D7E2–U+1D7F5 | `𝕏` (variante) |
| `upright(x)` | Reto (não-itálico) | U+1D5A0–U+1D5D3 (sans upright) | `𝖷` |

**Nota:** No vanilla, estas funções aplicam estilos tipográficos a expressões matemáticas. O cristalino já tem `MathStyle` (usado em `MathStyleElem` com `style: "bold"`), mas não expõe as 12 funções como construtores nativos.

---

## 2. Metodologia

Para cada função, implementar o mapeamento para `MathStyle`, criar teste `.typ`, correr contra vanilla 0.15.0 e cristalino.

```bash
# Baseline P509 (antes de tocar código):
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK" || echo "FAIL: $(basename $f)"
done
# Esperado: 37/37 OK

# Após cada sub-tarefa:
# Re-executar corpus e verificar não-regressão
```

---

## 3. Diagnóstico de Causa Raiz

O cristalino tem `MathStyleElem` com campo `style: Str` (documentado em `entities/elements/math_style.md`), mas:

1. **H1:** As 12 funções não estão registradas no módulo `math` do stdlib.
2. **H2:** O mapeamento de nome de estilo → `MathStyle` variant não está completo.
3. **H3:** O layout de `MathStyleElem` não aplica a transformação de glifos (shaping math).

**Verificação rápida:**
```bash
rg -n "MathStyleElem" src/entities/elements/ --type rs
rg -n "native_bb\|native_bold\|native_cal" src/stdlib/ --type rs
rg -n "math_namespace" src/stdlib/mod.rs --type rs
```

---

## 4. Implementação

### 4.1 — Adicionar Funções ao Módulo `math`

**Arquivo alvo:** `src/stdlib/math.rs` (ou `src/engine/stdlib/math.rs`)

```rust
// Registrar as 12 funções no namespace do módulo math
math_namespace.define("bb", Value::Func(native_math_style("bb")));
math_namespace.define("bold", Value::Func(native_math_style("bold")));
math_namespace.define("cal", Value::Func(native_math_style("cal")));
math_namespace.define("frak", Value::Func(native_math_style("frak")));
math_namespace.define("italic", Value::Func(native_math_style("italic")));
math_namespace.define("mono", Value::Func(native_math_style("mono")));
math_namespace.define("sans", Value::Func(native_math_style("sans")));
math_namespace.define("scr", Value::Func(native_math_style("scr")));
math_namespace.define("script", Value::Func(native_math_style("script")));
math_namespace.define("serif", Value::Func(native_math_style("serif")));
math_namespace.define("sscript", Value::Func(native_math_style("sscript")));
math_namespace.define("upright", Value::Func(native_math_style("upright")));
```

### 4.2 — Implementar `native_math_style`

```rust
fn native_math_style(style_name: &'static str) -> impl Fn(Args) -> SourceResult<Value> {
    move |args: Args| -> SourceResult<Value> {
        let body = args.expect::<Content>("body")?;
        Ok(Value::Content(Content::MathStyle(MathStyleElem {
            style: style_name.into(),
            body: Box::new(body),
        })))
    }
}
```

**Nota:** `script` é alias para `scr`. A implementação pode ser:

```rust
math_namespace.define("script", Value::Func(native_math_style("scr")));
```

### 4.3 — Mapeamento de Estilos para Unicode (Shaping Math)

**Arquivo alvo:** `src/engine/layout/math.rs` (ou `src/shaping/math.rs`)

O layout de `MathStyleElem` deve transformar caracteres alfabéticos no corpo para os blocos Unicode correspondentes:

```rust
fn layout_math_style(elem: &MathStyleElem, ctx: &mut LayoutContext) -> Vec<Frame> {
    let style = elem.style.as_str();
    let body = &elem.body;

    // Aplicar transformação de estilo ao conteúdo
    let styled_body = apply_math_style(body, style);
    layout_content(&styled_body, ctx)
}

fn apply_math_style(content: &Content, style: &str) -> Content {
    match content {
        Content::Text(text) => {
            let styled: String = text.chars().map(|c| map_char_to_math_style(c, style)).collect();
            Content::Text(styled.into())
        }
        Content::Sequence(seq) => {
            Content::Sequence(seq.iter().map(|c| apply_math_style(c, style)).collect())
        }
        // ... outros casos
        _ => content.clone(),
    }
}

fn map_char_to_math_style(c: char, style: &str) -> char {
    match style {
        "bb" => map_to_double_struck(c),
        "bold" => map_to_bold(c),
        "cal" => map_to_caligraphic(c),
        "frak" => map_to_fraktur(c),
        "italic" => map_to_italic(c),
        "mono" => map_to_monospace(c),
        "sans" => map_to_sans_serif(c),
        "scr" => map_to_script(c),
        "serif" => map_to_serif(c),
        "sscript" => map_to_sans_script(c),
        "upright" => map_to_upright(c),
        _ => c,
    }
}
```

**Implementação do mapeamento:**

```rust
// Mapeamento para Mathematical Alphanumeric Symbols (U+1D400–U+1D7FF)
fn map_to_double_struck(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D538 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D552 + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_bold(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_caligraphic(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D49C + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D4B6 + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_fraktur(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D504 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D51E + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_italic(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D44E + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_monospace(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D670 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D68A + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_sans_serif(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D5A0 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D5BA + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_script(c: char) -> char {
    match c {
        'A'..'Z' => char::from_u32(0x1D4B0 + (c as u32 - 'A' as u32)).unwrap_or(c),
        'a'..'z' => char::from_u32(0x1D4CA + (c as u32 - 'a' as u32)).unwrap_or(c),
        _ => c,
    }
}

fn map_to_serif(c: char) -> char {
    // Serif é o default — retorna o caractere original
    c
}

fn map_to_sans_script(c: char) -> char {
    // Sans-serif script — usar double-struck como aproximação ou mapeamento específico
    map_to_double_struck(c)
}

fn map_to_upright(c: char) -> char {
    // Upright sans-serif — mapear para bloco sans-serif
    map_to_sans_serif(c)
}
```

**Nota:** O mapeamento exato de `sscript` e `upright` pode variar. Verificar o vanilla 0.15.0 para o comportamento correto. `sscript` pode ser um alias para `bb` ou ter seu próprio bloco Unicode.

### 4.4 — Otimização: Tabela de Mapeamento

Para evitar `match` em cada caractere, pré-computar uma tabela:

```rust
lazy_static::lazy_static! {
    static ref MATH_STYLE_TABLES: HashMap<&'static str, HashMap<char, char>> = {
        let mut tables = HashMap::new();

        let mut bb = HashMap::new();
        for c in 'A'..='Z' {
            bb.insert(c, char::from_u32(0x1D538 + (c as u32 - 'A' as u32)).unwrap());
        }
        for c in 'a'..='z' {
            bb.insert(c, char::from_u32(0x1D552 + (c as u32 - 'a' as u32)).unwrap());
        }
        tables.insert("bb", bb);

        // ... repetir para os outros 11 estilos

        tables
    };
}

fn map_char_to_math_style(c: char, style: &str) -> char {
    MATH_STYLE_TABLES.get(style)
        .and_then(|table| table.get(&c))
        .copied()
        .unwrap_or(c)
}
```

---

## 5. Validação

### 5.1 Testes Básicos

```typst
// test-math-styles.typ
#assert.eq($bb(X)$, $𝕏$)
#assert.eq($bold(X)$, $𝐗$)
#assert.eq($cal(X)$, $𝒳$)
#assert.eq($frak(X)$, $𝔛$)
#assert.eq($italic(X)$, $𝑋$)
#assert.eq($mono(X)$, $𝚇$)
#assert.eq($sans(X)$, $𝖷$)
#assert.eq($scr(X)$, $𝓍$)
#assert.eq($script(X)$, $𝓍$)  // alias para scr
#assert.eq($serif(X)$, $X$)   // default
#assert.eq($sscript(X)$, $𝕏$)  // ou variant
#assert.eq($upright(X)$, $𝖷$)  // sans upright
```

### 5.2 Testes com Expressões Complexas

```typst
// Estilos aninhados
$bb(bold(x + y))$

// Estilos com subscripts
$cal(A)_i^j$

// Estilos com frações
$frak(1/2)$
```

### 5.3 Corpus P490+P500

Re-executar o corpus após implementação:

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK" || echo "FAIL: $(basename $f)"
done
```

**Esperado:** 37/37 OK (não-regressão).

---

## 6. Critério de Fecho

- [ ] 12 funções registradas no módulo `math`.
- [ ] `MathStyleElem` suporta os 12 estilos.
- [ ] Mapeamento Unicode implementado para `A`–`Z` e `a`–`z`.
- [ ] `script` é alias para `scr`.
- [ ] `sscript` e `upright` têm mapeamento definido (mesmo que aproximado).
- [ ] 12 testes unitários novos passam (`p510_math_bb`, `p510_math_bold`, ..., `p510_math_upright`).
- [ ] Corpus P490+P500: 37/37 OK (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/math.md` ou `entities/elements/math_style.md`).
- [ ] Sentinela `p510_math_styles` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p510.md` produzido.

---

## 7. Próximo Passo (P511)

Com P510 fechado, as brechas restantes de linguagem são:

| Brecha | Tamanho | Impacto |
|--------|---------|---------|
| Math elements granulares (`BinomElem`, `ClassElem`, `LimitsElem`, `MidElem`, `PrimesElem`, `ScriptsElem`, `StretchElem`) | M | Médio — math avançado |
| Table/Grid HLine/VLine | M | Médio — tabelas com linhas |
| Curve elements | M | Baixo — gráficos vetoriais |

**Recomendação:** P511 = **Math Elements Granulares** — 7 elementos que complementam os 12 estilos. Se o P510 for rápido (S-size coletivo), P511 pode ser M-size.

Alternativa: Iniciar **Trilha 5 (fontdb)** em paralelo.

---

## A. Apêndice — Tabela Unicode de Referência

| Estilo | Bloco Unicode | Range |
|--------|---------------|-------|
| Bold | Mathematical Bold | U+1D400–U+1D433 |
| Italic | Mathematical Italic | U+1D434–U+1D467 |
| Bold Italic | Mathematical Bold Italic | U+1D468–U+1D49B |
| Caligraphic | Mathematical Script | U+1D49C–U+1D4CF |
| Bold Caligraphic | Mathematical Bold Script | U+1D4D0–U+1D503 |
| Fraktur | Mathematical Fraktur | U+1D504–U+1D537 |
| Bold Fraktur | Mathematical Bold Fraktur | U+1D538–U+1D56B |
| Double-struck (bb) | Mathematical Double-struck | U+1D538–U+1D56B |
| Bold Double-struck | Mathematical Bold Double-struck | U+1D56C–U+1D59F |
| Sans-serif | Mathematical Sans-serif | U+1D5A0–U+1D5D3 |
| Sans-serif Bold | Mathematical Sans-serif Bold | U+1D5D4–U+1D607 |
| Sans-serif Italic | Mathematical Sans-serif Italic | U+1D608–U+1D63B |
| Sans-serif Bold Italic | Mathematical Sans-serif Bold Italic | U+1D63C–U+1D66F |
| Monospace | Mathematical Monospace | U+1D670–U+1D6A3 |

**Nota:** O vanilla pode usar `sscript` para "sans-serif script" (U+1D7E2–U+1D7F5) ou outro mapeamento. Verificar comportamento exato.
