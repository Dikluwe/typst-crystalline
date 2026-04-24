# Passo 129 — DEBT-1 subset: `text.weight` simbólico

**Série**: 129 (passo XS em L1; quarta aplicação do pattern
Passos 126/127/128).
**Precondição**: Passo 128 encerrado; 1049 total tests (818 L1 +
24 L2 + 186 L3 + 21 L4 + 6 ignorados); zero violations; 51 ADRs
activas; 11 DEBTs abertos. DEBT-1 subset com 7 propriedades
activas (bold, italic, size, fill, weight u16, tracking, leading).
**ADRs aplicáveis**: ADR-0038 (Style/Styles/StyleChain; 2 notas
— Passos 126 e 127), ADR-0040 (#set text activo via bake-in).
**ADR tocada**: decidida em 129.A.

---

## Objectivo

Estender o arm `"weight"` em `eval_set_text` para aceitar também
`Value::Str`, mapeando 9 nomes simbólicos canónicos do Typst
vanilla para `u16`:

| Nome | Valor |
|------|-------|
| `thin` | 100 |
| `extralight` | 200 |
| `light` | 300 |
| `regular` | 400 |
| `medium` | 500 |
| `semibold` | 600 |
| `bold` | 700 |
| `extrabold` | 800 |
| `black` | 900 |

Após o Passo 126, o arm aceita apenas `Value::Int` com
`u16::try_from(n)`. Este passo adiciona `else if let Value::Str`.

Ao fim do passo:

1. Arm `"weight"` aceita ambos: inteiro (comportamento 126) +
   string (comportamento 129).
2. Nome desconhecido é capturado silenciosamente — `delta.weight`
   fica `None`, sem warning, sem erro (espelha pattern 126/127/128).
3. Canary `font` (DEBT-50) preservado.
4. 2-3 testes L1 novos (captura + desconhecido + canary).
5. Efeito visível no PDF: nenhum (documentado, como 126/127/128).

Este passo **não**:
- Adiciona outras propriedades.
- Implementa consumo em layout.
- Adiciona ou remove semântica ao comportamento numérico do Passo
  126.
- Toca L2, L3, L4 (excepto teste DEBT-49 rotativo se pool exigir).

---

## Decisões já tomadas

1. **Só weight simbólico** — disciplina dos 126/127/128
   reiterada.
2. **9 nomes canónicos** — espelha vanilla (confirmado em
   typst.app/docs/reference/text/text e discussão
   github.com/typst/typst/discussions/2508).
3. **Nome desconhecido silencioso** — espelha pattern DEBT-1 XS.
   Confirmar em 129.A que o vanilla não tem comportamento
   radicalmente divergente.

## Decisões diferidas (129.A)

4. **Localização do mapeamento nome → u16**:
   - **(a) Inline** no arm `"weight"` do `eval_set_text`
     (match sobre `s.as_str()`).
   - **(b) Helper** `weight_from_name(&str) -> Option<u16>` em
     módulo L1 tipográfico existente (ADR-0037 — coesão por
     domínio).
   - **(c) Helper em módulo novo** se nenhum módulo tipográfico
     natural existe.

   Preferência: (b) se existe módulo candidato; (a) se não
   existe e criar módulo excede XS.

5. **ADR-0038**:
   - **Não anotar** se decisão é (a) — pattern literal.
   - **Anotar** se decisão é (b) ou (c) — documenta extracção de
     helper fora do arm como variante permitida do pattern.

---

## Escopo

**Dentro**:
- `01_core/src/rules/eval/rules.rs` — extensão do arm `"weight"`.
- `01_core/src/<módulo tipográfico>.rs` — helper
  `weight_from_name` (se decisão (b)).
- `01_core/src/<novo>.rs` — helper em ficheiro novo (se decisão
  (c); evitar).
- `01_core/src/rules/eval/tests.rs` — 2-3 testes.
- Prompt L0 relacionado se existe + hash.
- ADR-0038 anotada se 129.A decidir.

**Fora**:
- `StyleDelta.weight` — já existe (Passo 126). Zero mudança.
- `StyleDelta::empty()` — já inicializa. Zero mudança.
- Outras propriedades.
- Consumo em layout.
- L2, L3, L4.

---

## Sub-passos

### 129.A — Inventário

**Parte 1 — Vanilla `FromValue` de `FontWeight`**:

1. Localizar em `lab/typst-original/`:
   - `grep -rn "FontWeight" lab/typst-original/crates/typst-library/src/text/`
   - Candidato provável: `text/mod.rs` ou `foundations/cast.rs`,
     procurar `impl FromValue for FontWeight` ou macro
     `cast!{ FontWeight, ... }`.
2. Registar:
   - Os 9 nomes aceites pelo vanilla. Confirmar que são
     exactamente os da tabela acima.
   - Comportamento em nome inválido:
     - Erro de cast (provável)?
     - Silencioso?
     - Warning?
   - Se há suporte a alias (ex: `"normal"` → 400)?

**Parte 2 — Arm actual do `"weight"`**:

1. `grep -n "\"weight\"" 01_core/src/rules/eval/rules.rs`.
2. Registar corpo exacto. Confirmar que hoje faz apenas:

   ```rust
   "weight" => {
       if let Value::Int(n) = val {
           if let Ok(w) = u16::try_from(n) {
               delta.weight = Some(w);
           }
       }
   }
   ```

   (ou variante — registar como está.)

**Parte 3 — Módulo tipográfico em L1**:

1. Listar: `ls 01_core/src/entities/ 01_core/src/`.
2. Procurar módulos candidatos para o helper:
   - `font.rs`, `weight.rs`, `text.rs`, `typography.rs`,
     `entities/text.rs`, `entities/layout_types.rs` (se tem
     `FontWeight`).
3. Registar:
   - Módulo existente com semântica tipográfica — **sim/não**.
   - Se sim, qual e se tem tipo `FontWeight` ou similar.
   - Se não, candidato: arm inline (decisão (a)).

**Parte 4 — Teste DEBT-49**:

1. `grep -n "weight" 03_infra/src/integration_tests.rs`.
2. Confirmar que `debt49_set_text_multiplas_propriedades_desconhecidas`
   (ou variante L3) **não** usa `weight` como propriedade
   desconhecida — é propriedade conhecida desde 126.
3. Se usa, rotar.

**Parte 5 — Decisão (a), (b), ou (c)**:

Matriz:

| Cenário inventário | Decisão |
|---|---|
| Módulo tipográfico existe com `FontWeight` ou similar | **(b)** |
| Módulo tipográfico existe mas sem conceito de weight | **(b)** — adicionar helper lá mesmo |
| Nenhum módulo tipográfico natural | **(a)** |

**Escrever** em `00_nucleo/diagnosticos/inventario-weight-simbolico-passo-129.md`:

```
Vanilla FontWeight FromValue:
  localização: <caminho>
  nomes aceites: [thin, extralight, light, regular, medium,
                  semibold, bold, extrabold, black]
  comportamento inválido: <erro / silencioso / warning>
  aliases: <lista ou "nenhum">

Arm "weight" actual:
  ficheiro: 01_core/src/rules/eval/rules.rs:XXX
  corpo: <colar>

Módulo tipográfico L1:
  existe: sim/não
  candidato: <path>
  tem FontWeight: sim/não

DEBT-49 usa weight: sim/não

Decisão localização: (a) / (b) / (c)
Razão: [...]

Divergência vanilla:
  nome inválido silencioso vs <comportamento vanilla>
  categoria ADR-0033: estrutural / semântica
  aceita-se: sim/não
```

**Gate 129.A**:

- Se decisão é (b) e exige criar módulo novo (sem candidato
  natural) → **parar e perguntar**. Criar módulo é escopo extra.
- Se 129.A.1 revela comportamento vanilla divergente em **forma
  semântica** (ex: vanilla emite warning específico, não
  silencioso) → **parar e discutir**. Pode exigir anotação em
  ADR-0033 ou revisão da decisão 2.
- Se total linhas esperadas > 50, reportar. Limite XS.

### 129.B — ADR (condicional)

**Decisão (a) + pattern literal**: sem anotação. Quarta aplicação
idêntica; pattern sólido.

**Decisão (b) ou (c)**: anotação em ADR-0038:

```markdown
### Nota Passo 129 — `weight` simbólico com helper externo

Extensão do arm `"weight"` (Passo 126) para aceitar `Value::Str`
com 9 nomes canónicos. Mapeamento nome → u16 extraído para
helper `weight_from_name` em <módulo> (ADR-0037 — coesão por
domínio). Variante (b)/(c) do pattern DEBT-1 XS: arm permanece
trivial, lógica tipográfica vive no módulo tipográfico.

Precedente para futuras propriedades simbólicas (style names,
stretch names, etc.).
```

### 129.C — Implementação

**129.C.1 — Helper (se decisão (b) ou (c))**:

Localização: decidida em 129.A.3.

```rust
/// Mapeia nome simbólico de peso tipográfico para valor numérico.
///
/// Aceita os 9 nomes canónicos do Typst vanilla
/// (thin/extralight/light/regular/medium/semibold/bold/
/// extrabold/black).
/// Devolve `None` para qualquer outro string.
pub fn weight_from_name(name: &str) -> Option<u16> {
    match name {
        "thin" => Some(100),
        "extralight" => Some(200),
        "light" => Some(300),
        "regular" => Some(400),
        "medium" => Some(500),
        "semibold" => Some(600),
        "bold" => Some(700),
        "extrabold" => Some(800),
        "black" => Some(900),
        _ => None,
    }
}
```

**129.C.2 — Extensão do arm `"weight"`**:

**Variante (a) — inline**:

```rust
"weight" => {
    if let Value::Int(n) = val {
        if let Ok(w) = u16::try_from(n) {
            delta.weight = Some(w);
        }
    } else if let Value::Str(s) = val {
        let mapped = match s.as_str() {
            "thin" => Some(100),
            "extralight" => Some(200),
            "light" => Some(300),
            "regular" => Some(400),
            "medium" => Some(500),
            "semibold" => Some(600),
            "bold" => Some(700),
            "extrabold" => Some(800),
            "black" => Some(900),
            _ => None,
        };
        if let Some(w) = mapped {
            delta.weight = Some(w);
        }
    }
}
```

**Variante (b)/(c) — com helper**:

```rust
"weight" => {
    if let Value::Int(n) = val {
        if let Ok(w) = u16::try_from(n) {
            delta.weight = Some(w);
        }
    } else if let Value::Str(s) = val {
        if let Some(w) = weight_from_name(s.as_str()) {
            delta.weight = Some(w);
        }
    }
}
```

**129.C.3 — Testes novos (L1)**:

```rust
#[test]
fn eval_set_text_weight_simbolico_aceita_nomes_canonicos_passo_129() {
    let casos = [
        ("thin", 100u16),
        ("extralight", 200),
        ("light", 300),
        ("regular", 400),
        ("medium", 500),
        ("semibold", 600),
        ("bold", 700),
        ("extrabold", 800),
        ("black", 900),
    ];
    for (nome, esperado) in casos {
        let src = format!(r#"#set text(weight: "{}")"#, nome);
        let (delta, warnings) = eval_inline(&src);
        assert_eq!(delta.weight, Some(esperado),
            "nome '{}' devia mapear para {}", nome, esperado);
        assert!(warnings.iter().all(|w| !w.message.contains("'weight'")),
            "nome '{}' gerou warning inesperado", nome);
    }
}

#[test]
fn eval_set_text_weight_simbolico_ignora_nome_desconhecido_passo_129() {
    // Documenta por assertion: nome inválido é capturado silenciosamente.
    // Se o comportamento mudar (ex: vira warning), este teste falha e força
    // revisão consciente.
    let src = r#"#set text(weight: "arcoiris")"#;
    let (delta, warnings) = eval_inline(src);
    assert_eq!(delta.weight, None);
    assert!(warnings.iter().all(|w| !w.message.contains("'weight'")));
}

#[test]
fn eval_set_text_font_canary_passo_129() {
    // Canary DEBT-50: font continua a emitir warning (quarta iteração).
    let src = "#set text(font: \"X\")\nOlá";
    let (_, warnings) = eval_inline(src);
    assert!(warnings.iter().any(|w| w.message.contains("'font'")));
}
```

**129.C.4 — Rotação DEBT-49**:

Grep em 129.A.4 informa. Esperado: `weight` não está no pool de
"desconhecidas" desde o Passo 126 — sem rotação.

**129.C.5 — Prompt L0 + hash**:

Se o mapeamento weight é referido em algum L0
(`00_nucleo/prompts/...`), actualizar e correr
`crystalline-lint --fix-hashes .`.

### 129.D — Verificação

1. `cargo test -p typst-core` — L1: 818 → **821** (+3).
2. `cargo test --workspace` — total ≥ 1052.
3. `crystalline-lint` zero violations.
4. Manual:

   ```bash
   $ cat w.typ
   #set text(weight: "bold")
   Olá
   $ typst w.typ -o w.pdf 2>&1
   # stderr: vazio
   exit=0

   $ cat winv.typ
   #set text(weight: "arcoiris")
   Olá
   $ typst winv.typ -o winv.pdf 2>&1
   # stderr: vazio (capturado silenciosamente)
   exit=0

   $ cat wnum.typ
   #set text(weight: 700)
   Olá
   $ typst wnum.typ -o wnum.pdf 2>&1
   # stderr: vazio (regressão do 126 — deve continuar OK)
   exit=0
   ```

5. Canary:

   ```bash
   $ typst f.typ -o f.pdf 2>&1       # #set text(font: "X")
   # stderr contém warning 'font'
   ```

### 129.E — Encerramento

Relatório em `typst-passo-129-relatorio.md`:

- Inventário resultado (especial: 129.A.1 vanilla vs nosso).
- Decisão (a)/(b)/(c) + razão.
- ADR anotada ou não + razão.
- Pattern 126/127/128 aderência.
- Diff arm "weight" + helper (se aplicável) + testes.
- Divergência vanilla (nome inválido) documentada.
- Candidatos futuros: outras propriedades simbólicas (style
  names, stretch names) podem reutilizar o helper pattern.

---

## Critério de conclusão

1. Inventário 129.A escrito.
2. Decisão localização documentada.
3. ADR-0038 anotada **se aplicável**.
4. Arm `"weight"` aceita `Value::Str` com 9 nomes canónicos.
5. Nome desconhecido silencioso (teste documenta).
6. Valor numérico continua a funcionar (regressão 126).
7. Canary DEBT-50 preservado.
8. 3 testes L1 novos passam.
9. DEBT-49 rotado se necessário (esperado: não).
10. `cargo test --workspace` passa (≥ 1052).
11. `crystalline-lint` zero violations.
12. Relatório 129.E escrito.

---

## O que pode sair errado

- **Vanilla emite erro/warning específico em nome inválido**:
  nossa captura silenciosa diverge. Decidir em 129.A.1 se:
  (i) aceita divergência como temporal (pattern DEBT-1 XS é
  silencioso por design), (ii) ajusta para emitir warning, ou
  (iii) pára e discute antes de 129.C.

- **Módulo tipográfico com `FontWeight` existe mas é complexo**
  (enum, variantes, métodos): adicionar helper pode ser fácil
  mas exige não colidir com estrutura existente. Gate 129.A.3
  detecta.

- **Pool DEBT-49 esgotou**: teste L3 precisa de 3 propriedades
  "desconhecidas". Com 7 propriedades activas, pool pode estar
  a < 5 (stroke, alignment, lang, first-line-indent, justify,
  ...). Se esgotou → passo dedicado para repensar teste.

- **Alias no vanilla** (ex: `"normal"` → 400 como sinónimo de
  `"regular"`): se 129.A.1 revela, decidir se incluir (2-3
  linhas extra) ou deferir.

- **`Value::Str` em contexto tracked**: já validado no projecto
  (convenções comemo em CLAUDE.md). Sem surpresa esperada.

---

## Notas operacionais

- Quarta aplicação do pattern DEBT-1 XS. 126 (u16 numérico), 127
  (Length), 128 (Length divergente por contexto), 129 (u16
  simbólico). Cada aplicação mostrou nuance diferente — vale
  documentar se o padrão está a cobrir variações suficientes
  para considerar estável.

- Candidato pattern: "propriedade com conversão de símbolo para
  número/valor" (weight hoje; style `"italic"/"oblique"/"normal"`
  possível no futuro; stretch com ratios).

- Se 129.A revela que o vanilla suporta alias (`"normal"` →
  `"regular"`), **não** incluir neste passo sem discussão. Alias
  é decisão semântica separada.

- Pool DEBT-49: após este passo, propriedades ainda desconhecidas
  incluem provavelmente `stroke`, `alignment`, `lang`,
  `first-line-indent`, `justify`, `hyphenate`, `dir`. Pool
  saudável.

- Candidato futuro registado anteriormente: "Extract helper
  `eval_with_warnings` em L1 test harness" (Passo 127). Ainda
  pendente — não executa neste passo.
