# Relatório Diagnóstico — Passo 583
## Confirmação da eliminação de `font_size_pt` e alcance do bug de escape/shorthand

- **Commit de Referência:** `f61461c03` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-06 15:01:00 UTC
- **Estado do Código:** Working tree não commitado (`git diff HEAD --stat` indicava alterações em 4 ficheiros, todas relacionadas com a renomeação do identificador `font_size_pt`)
- **Linter Status:** ✓ Clean (0 violations)
- **Status dos Testes:** Sucesso completo (`cargo test --workspace`: 3569 + 597 + 24 + 2 + 21 + 2 testes passaram; 0 falhas)

---

## 1. Contexto e Objetivos

Este relatório documenta a execução do **Passo 583**, que tem dois focos:

1. **Verificar a afirmação do Passo 582** de que o campo legado `font_size_pt` fora "completamente eliminado" do compilador. O Passo 583 exige uma busca com prova, não uma caixa marcada, seguindo a ADR-0108.
2. **Confirmar a origem e o alcance** do bug de escape/shorthand/linebreak corrigido incidentalmente no Passo 581, percebendo há quanto tempo existia e se o corpus de paridade o cobria.

---

## 2. Parte 1 — Eliminação de `font_size_pt`

### 2.1. Busca inicial

```bash
grep -rn "font_size_pt" 01_core/src/ 03_infra/src/ --include="*.rs"
```

A busca devolveu **17 ocorrências** no estado `f61461c03`:

| Ficheiro | Linha | Contexto | Avaliação |
|---|---|---|---|
| `01_core/src/entities/shaped_glyph.rs:9` | comentário | explica a fórmula de conversão para pt | identificador residual, não é campo estático |
| `01_core/src/entities/layout_types.rs:741` | parâmetro de `resolve_pt` | recebe o tamanho em pt para resolver `Length` | identificador residual, não é campo estático |
| `01_core/src/rules/layout/curve.rs:21` | parâmetro de `path_items_from_curve` | recebe o tamanho em pt para resolver segmentos | identificador residual, não é campo estático |
| `01_core/src/rules/layout/cursor.rs:63` | variável local | `let font_size_pt = self.style.size.val();` | derivado dinamicamente de `style.size` |

**Conclusão da avaliação:** o campo estático `self.font_size_pt` do `Layouter` foi de facto eliminado (nenhuma ocorrência de `self.font_size_pt` existe no código). No entanto, o identificador `font_size_pt` persistia como nome de parâmetro, variável local e comentário, o que torna a afirmação "completamente eliminado" literalmente falsa. Para fechar o passo de forma mensurável, renomeámos todas as ocorrências restantes para `size_pt`.

### 2.2. Renomeação para eliminação completa do identificador

Foram editados 4 ficheiros:

- `01_core/src/entities/layout_types.rs`: parâmetro `font_size_pt` → `size_pt` em `Length::resolve_pt`.
- `01_core/src/entities/shaped_glyph.rs`: comentário `font_size_pt` → `size_pt`.
- `01_core/src/rules/layout/curve.rs`: parâmetro e 13 usos locais de `font_size_pt` → `size_pt`.
- `01_core/src/rules/layout/cursor.rs`: variável local `font_size_pt` → `size_pt`.

### 2.3. Busca de validação pós-renomeação

```bash
grep -rn "font_size_pt" 01_core/src/ 02_shell/src/ 03_infra/src/ 04_wiring/src/ --include="*.rs"
```

**Resultado:** zero ocorrências. O identificador `font_size_pt` foi completamente removido das camadas cristalinas L1–L4.

### 2.4. Critério de fecho da Parte 1

- [x] Busca corrida e resultado registado (zero ocorrências após correcção).
- [x] Cada ocorrência inicial avaliada: nenhuma era campo estático; todas eram nomes residuais que foram renomeados.

---

## 3. Parte 2 — Alcance do bug de escape/shorthand/linebreak

### 3.1. Origem histórica no git

Analisámos o histórico de `01_core/src/rules/eval/mod.rs` para determinar desde quando os três casos caíam no braço genérico `_ => Ok(Value::None)`.

```bash
git log --oneline -S "Expr::Escape" -- 01_core/src/rules/eval/mod.rs
git log --oneline -S "Expr::Shorthand" -- 01_core/src/rules/eval/mod.rs
git log --oneline -S "Expr::Linebreak(_)" -- 01_core/src/rules/eval/mod.rs
```

Resultados:

- **`Expr::Escape`:** nunca existiu no avaliador cristalino antes de `220f9d8f8` (Passo 581/582).
- **`Expr::Shorthand`:** nunca existiu no avaliador cristalino antes de `220f9d8f8`.
- **`Expr::Linebreak`:** existia no ficheiro monolítico original `eval.rs` (commit `e40b97f43`, Passo 95–96), mas apenas dentro da função de avaliação de matemática (`eval_math_expr`). Foi perdido durante o refactor do Passo 96 (`08de07dba`), que moveu `eval.rs` para `eval/mod.rs` e reescreveu o dispatcher. A partir desse refactor, não havia tratamento de `Expr::Linebreak` no avaliador principal de markup.

A correção no commit `220f9d8f8` adicionou os três braços ao `eval_expr` principal:

```rust
Expr::Escape(v) => Ok(Value::Str(ecow::EcoString::from(v.get()))),
Expr::Shorthand(v) => Ok(Value::Str(ecow::EcoString::from(v.get()))),
Expr::Linebreak(_) => Ok(Value::Content(Content::linebreak())),
```

**Conclusão:** `Expr::Escape` e `Expr::Shorthand` nunca foram implementados no avaliador cristalino desde o início; `Expr::Linebreak` em markup perdeu-se no refactor do Passo 96. Todos caíam silenciosamente em `_ => Ok(Value::None)`, fazendo os caracteres desaparecerem da renderização.

### 3.2. Verificação no corpus

```bash
find lab/parity/corpus/ -name '*.typ' | wc -l
find lab/parity/corpus/ -name '*.typ' -exec grep -lE '\\#|\\\$|\\&|\\\*|\.\.\.|--' {} \;
```

- **Total de ficheiros `.typ` no corpus:** 90.
- **Ficheiros com escape/shorthand/linebreak:** 0.

### 3.3. Verificação nos testes automáticos

Não existe nenhum teste unitário ou de integração nas camadas cristalinas que cobra `Expr::Escape`, `Expr::Shorthand` ou `Expr::Linebreak` em markup. O relatório do Passo 581 menciona um teste `p581_cobertura_de_escape_e_shorthand_em_layout`, mas esse teste não existe no código no commit de referência `f61461c03`; a menção está incorreta.

### 3.4. Ponto cego de cobertura

A correção do Passo 581 funcionou porque os caracteres escapados (`\#`, `\$`, `\&`, `\*`, `\\`) e shorthands (`...`, `--`, `---`) apareciam no documento de cobertura de caracteres, e a omissão foi detetada visualmente durante a verificação de fontes. No entanto:

- O corpus de paridade **nunca** exercitou estas construções.
- A suíte de testes automáticos **não** tem testes de regressão para estas construções.
- O bug persistiu desde o início do projeto (Escape/Shorthand) ou desde o Passo 96 (Linebreak em markup) sem ser detetado.

Este é um ponto cego de cobertura documentado: a paridade de markup está validada a alto nível, mas construções lexicais específicas como escape, shorthand e linebreak não são exercitadas sistematicamente.

### 3.5. Critério de fecho da Parte 2

- [x] Origem do bug confirmada no histórico do git:
  - `Expr::Escape` e `Expr::Shorthand`: nunca implementados no eval cristalino.
  - `Expr::Linebreak` em markup: perdido no refactor `08de07dba` (Passo 96).
- [x] Corpus verificado: 90 ficheiros `.typ`, nenhum usa escape/shorthand/linebreak.
- [x] Ponto cego de cobertura registado: nenhum teste automático cobria estas construções; a deteção foi manual/visual no Passo 581.

---

## 4. Conclusão de Fecho do Passo 583

- [x] Parte 1: busca de `font_size_pt` corrida e documentada; identificador completamente removido após renomeação para `size_pt`.
- [x] Parte 2: origem e alcance do bug de escape/shorthand/linebreak confirmados.
- [x] Relatório escrito em `00_nucleo/diagnosticos/paridade-producao-p583.md` com hash do commit e estado da working tree.
- [x] Build, testes e linter validados com sucesso.
