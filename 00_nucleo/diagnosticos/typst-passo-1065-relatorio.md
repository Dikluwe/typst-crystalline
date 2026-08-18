# Relatório de Execução — Passo 1065 (Consolidado e Detalhado)

**Data**: 2026-08-17
**Passo**: 1065 — Anotação Formal dos 46 Casos `/ 2.0` (Classes 1A, 1B e 1C)
**Gate**: `ADR-0127` (Classificação: Modificação Documental em Massa / Homologada pelo Dono)
**Status**: CONCLUÍDO COM ÊXITO (Rastreabilidade integral da proveniência de `axis_pt` no §1, validação da sintaxe do linter no §2 e contagem diferencial de violações V21 no §3)

---

## 1. Resposta ao §1 — Proveniência do Termo `axis_pt` nos 11 Casos de 1C

Auditou-se a proveniência do termo aditivo nos 11 pontos da Classe 1C:
1. **Casos de Alinhamento Matemático ao Eixo (`01_core/src/compiler/math/layout/mod.rs:383, 873, 879, 880`)**:
   - O termo `axis_pt` provém de `self.constants.to_pt(self.constants.axis_height, size).val()` (linhas 382 e 867).
   - `axis_height` é lido diretamente da tabela padrão OpenType `MATH` (`MathConstants::axis_height`, especificação ISO/IEC 14496-22).
   - O cabeçalho da função em `mod.rs:377-380` já documenta: `/// O eixo matemático é axis_height (design units) acima da baseline. Após este ajuste, o centro vertical do box fica no eixo.`
   - Portanto, a parcela aditiva `axis_pt` já possui proveniência canônica de especificação formal OpenType; o `/ 2.0` anotado no P1065 isenta exclusivamente a centragem euclidiana do ponto médio da altura.
2. **Casos de Margem de Rodapé (`cursor.rs:475`, `mod.rs:1643, 1677`)**:
   - O termo aditivo `self.page_config.margin` provém da configuração de página (`PageConfig::margin`), definida pelo usuário ou padrão de página Typst (`#set page(margin: ...)`).
3. **Casos de Meia-Altura e Centro de Tinta (`assembly.rs:152`, `stretchy.rs:74, 99, 101`)**:
   - Expressões de semi-altura pura (`total_height / 2.0`, `height_pt / 2.0`, `(ink_up - ink_down) / 2.0`, `(ink_up + ink_down) / 2.0`) sem termo aditivo externo.

---

## 2. Resposta ao §2 — Validação da Convenção `// rationale:` contra o Linter

A mensagem oficial emitida pelo linter `crystalline-lint` para a regra V21 determina:
> `warning: Escalar contextual fixo: literal ... sem proveniência citada (adicione // ref:, // spec: ou // rationale:) [V21]`

Exemplos de proveniência já existentes na base de código:
- `01_core/src/compiler/layout/divider.rs:30`: `// ref: lab/typst-original/crates/typst-library/src/visualize/line.rs:20`
- `01_core/src/compiler/layout/quote.rs:39`: `// ref: lab/typst-original/crates/typst-library/src/model/quote.rs:80`
- `01_core/src/compiler/layout/raw.rs:108`: `// ref: lab/typst-original/crates/typst-library/src/text/raw.rs:360`

Para decisões de geometria euclidiana pura (onde não há arquivo externo do Vanilla a citar como "design arbitrário", mas sim uma dedução algébrica de ponto médio), a regra V21 aceita o prefixo canônico `// rationale: P1064 Classe 1A/1B/1C — ...`.

---

## 3. Resposta ao §3 — Contagem Diferencial de Violações V21 (Antes vs Depois)

Comparação executada via `crystalline-lint --checks v21 .`:

| Métrica | Pré-Anotação (Passo 1058/1064) | Pós-Anotação (Passo 1065) | Variação ($\Delta$) |
| :--- | :---: | :---: | :---: |
| **Violações de Centragem `/ 2.0` (Classes 1A, 1B, 1C)** | **46** | **0** | **-46** (100% eliminadas) |
| **Avisos V21 Restantes (Outras Categorias)** | 36 | 36 | 0 |
| **Total Geral de Avisos V21 Reportados** | **82** (46 centragem + 36 outras) | **36** | **-46** |

### Discriminação dos 36 Avisos V21 Restantes (Escopo de Passos Futuros):
1. **Multiplicação de Margem Simétrica (`2.0 * margin`)**: 12 ocorrências (Categoria 2 / Passo 1066).
2. **Escalar de Leading de Texto (`0.65`)**: 7 ocorrências.
3. **Escalares de Matemática e Tabela (`0.8`, `0.05`, `0.5`, `0.2`, `1.2`)**: 17 ocorrências.

---

## 4. Conclusão e Integridade do Workspace

- **Compilação e Testes**: `cargo test --workspace` com **5.942 testes aprovados (100% PASS)**.
- **Linter**: `crystalline-lint .` com **0 erros**.
