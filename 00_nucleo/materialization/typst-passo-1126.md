# L0 — Passo 1126: Plano Consolidado Final de Implementação e Paridade

**Gate**: `ADR-0127` — Implementação e validação integral das 11 frentes reconciliadas sem contradições, cobrindo o escopo do P1124 e as extensões do P1125.

---

## 1. Escopo das 11 Correções Reconciliadas

### §1. Colapso Genérico de Margens em Equações de Bloco (`equation.rs`)
- **Problema**: Aplicação incondicional de `gap = prev_block_below_pending` ignorando `above` explícito da equação.
- **Mecanismo**: `gap = max(prev.below, curr.above)`.
- **Alvos**: Convergência exata das Secções 32 e 35 e preservação dos testes Iso A-D do P1063.

### §2. Resolução de Espaçamento em `+` antes de `dif` (`spacing.rs` / `structural/math.rs` — Secção 27)
- **Problema**: Soma cumulativa de espaçamento `Binary` sobre o `HSpace(1/6em)` intrínseco de `dif`.
- **Mecanismo**: Resolução por precedência fraca/máximo (`max()`) em vez de adição cega.

### §3. Sobrescrito Empilhado com `base_ascent` do Núcleo (`attach.rs` — Secção 41)
- **Problema**: Quando a base já possui scripts anteriores, `base_ascent` recebe a altura da caixa acumulada, inflando o `drop_term` do sobrescrito.
- **Mecanismo**: Uso da altura real do núcleo da base para calcular o deslocamento vertical.

### §4. Operador `sum` Inline com Altura Base (`attach.rs` — Secção 31)
- **Problema**: Operadores grandes em modo inline usavam constantes de glifo comum, ignorando a altura da base grande e `sub_superscript_gap_min`.
- **Mecanismo**: Ajuste dos deslocamentos superior e inferior considerando o tamanho real do operador e o gap mínimo.

### §5. Delimitador `|` como `MathClass::Fence` (`spacing.rs` — Secções 9, 26, 28)
- **Problema**: Falta do braço `(MathClass::Fence, _)` em `spacing_between_class`, caindo em `Relation => THICK` e injetando $\approx 3.65\text{ pt}$ ao redor de `$|x|$`.
- **Mecanismo**: Inclusão de `MathClass::Fence` na tabela de classes para eliminar o espaçamento binário/relação indesejado.

### §6. Propagação de Estilo em `Content::MathStyled` (`math/layout/mod.rs` — Secção 19)
- **Problema**: `MathStyled` ajustava `style.size` mas não sincronizava `math_style.math_size` nem `cramped`.
- **Mecanismo**: Atualização bidirecional do contexto matemático durante o layout interno.

### §7. Vírgula de Pontuação em Fallback de String (`eval/math.rs` — Secção 23)
- **Problema**: Fallback de chamadas injetava `Content::MathText(", ")` com espaço literal ASCII em vez de `MathClass::Punctuation`.
- **Mecanismo**: Emissão estruturada com classe de pontuação matemática nativa.

### §8. Descent de `integral.cont` / `integral.cont_gamma` (`equation.rs` — Secção 23)
- **Problema**: Métrica de descente da integral com contorno reportada em $4.30\text{ pt}$ contra $2.75\text{ pt}$ no Vanilla.
- **Mecanismo**: Verificação e aplicação das métricas reais de tinta/fonte para a variante fechada de integral com contorno.

### §9. Subscritos em Operadores Textuais Multi-Caractere (`attach.rs` — `log_a`, `log_2`, `lim`, `sin`, etc.)
- **Problema**: Operadores textuais como `log` com letras descendentes (`'g'`) somavam `base_descent = 2.409 pt` ao `drop_term`, rebaixando o subscrito em $1.75\text{ pt}$.
- **Mecanismo**: Identificação via `chars().count() > 1` aplicando `eff_base_descent = 0.0` para alinhamento uniforme na baseline textual.

### §10. Integral Verde com Estilização Aninhada (`attach.rs` — Secção 34 / Extended 34)
- **Problema**: Envelopamento em `Content::Styled` / `Content::MathStyled` ocultava operadores esticados (`integral`), impedindo a ativação do `drop_term`.
- **Mecanismo**: Unwrapping recursivo completo preservando a verificação de `MathLimitsOverride`.

### §11. Espaçamento Vertical de Linhas em `stack(dir: ttb)` (`stack.rs` / `helpers.rs` — Secção 43)
- **Problema**: Translação vertical calculada por altura de frame com acréscimo arbitrário de descida em caracteres sem descendente (`a`, `b`, `c`).
- **Mecanismo**: Avanço de baseline a baseline ($\Delta Y = \text{descent}_i + \text{spacing} + \text{ascent}_{i+1}$) com detecção real de caracteres descendentes.

---

## 2. Critérios de Aceitação e Verificação

1. **Testes Unitários e de Integração**:
   - `cargo test --workspace`: 100% PASS (5.958+ testes aprovados, 0 falhas).
   - `crystalline-lint .`: 0 violações type-level.
2. **Paridade em Documentos de Benchmark**:
   - Secções 7, 9, 19, 23, 26, 27, 28, 31, 32, 34, 35, 41 e 43 com convergência validada contra o Typst Vanilla.
3. **Recompilação e Sincronização**:
   - Todos os 47 arquivos em `.typ/` recompilados e sincronizados para `crystalline`, `oracle` e `vanilla`.
