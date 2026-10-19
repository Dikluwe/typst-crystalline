# Relatório de Execução — Passo 1071: Recuperação e Triagem dos 6 Achados Média/Baixa do P1031

**Data**: 2026-08-18
**Passo**: 1071 — Recuperar e Escrever os 6 Achados Média/Baixa do P1031
**Gate**: Nenhum (Passo preparatório de levantamento e auditoria factual — sem alterações de código ou escrita de L0s de correção neste passo)
**Status**: CONCLUÍDO (Relatório P1031 obtido diretamente do repositório, 6 achados de prioridade Média/Baixa isolados, auditados no código atual e confirmados como pendentes)

---

## 1. Passo 0 — Obtenção do Documento Fonte Real

O documento de referência [`00_nucleo/diagnosticos/typst-passo-1031-relatorio.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/typst-passo-1031-relatorio.md) encontra-se disponível no repositório.

A seção *"Achados escalados (12) — nenhum implementado aqui"* (linhas 70–95) cataloga os 12 achados reais de divergência:
* **Achados #1 a #6 (Prioridade Alta)**: Divergências críticas de layout/contexto/numeração (show rules, `bibliography.style`, argumentos `label`, heading levels no corpo, `figure.numbering`, língua default) — desdobrados e tratados em passos dedicados (P1034–P1037).
* **Achados #7 a #12 (Prioridade Média/Baixa)**: As 6 divergências residuais catalogadas para este passo.

---

## 2. Passo 1 — Inventário, Auditoria e Estado Atual dos 6 Achados (Média/Baixa)

Todos os 6 achados foram verificados contra o código-fonte ativo em `01_core/` para determinar se algum foi resolvido incidentalmente por passos intermediários:

| # (P1031) | Achado e Descrição Técnica | Prioridade Original | Arquivo / Linha no Código Atual | Evidência de Código Atual | Estado Atual |
| :-: | :--- | :---: | :--- | :--- | :---: |
| **7** | **Supplements de `ref` divergem**<br>O vanilla resolve pelo `LocalName` do elemento (`Figure 1`, `Equation 1`). O cristalino usa abreviação fixa `"Fig. "`. | **Média** | `01_core/src/compiler/layout/references.rs:242`<br>`01_core/src/compiler/layout/references.rs:269` | `.unwrap_or_else(\|\| "Fig. ".to_string())`<br>`CounterKey::Str(s) if s.starts_with("figure:") => Some(Content::text("Fig."))` | **PENDENTE** |
| **8** | **Default de `body_indent` é `0pt` em vez de `0.5em`**<br>Em `list` e `enum`, o vanilla recua o corpo em `0.5em` por padrão (`• Um`). O cristalino usa `0.0pt` (`•Um`). | **Média** | `01_core/src/compiler/layout/enum_item.rs:53`<br>`01_core/src/compiler/layout/list_item.rs:52` | `e.body_indent.unwrap_or(Length::pt(0.0))` | **PENDENTE** |
| **9** | **Heading força `italic=false` em vez de herdar**<br>O `show_set` do vanilla nunca reseta `TextElem::style`. O cristalino zera explicitamente `italic: false` no `TextStyle`. | **Média** | `01_core/src/compiler/layout/heading.rs:70` | `layouter.style = TextStyle { bold: true, italic: false, ... }` | **PENDENTE** |
| **10** | **Grupo de captura regex não participante devolve `""` em vez de `none`**<br>`"ab".match(regex("a(x)?(b)"))` devolve `(none, "b")` no vanilla e `("", "b")` no cristalino. | **Média** | `01_core/src/entities/regex.rs:65`<br>`01_core/src/entities/regex.rs:85` | `captures.push(caps.get(i).map(\|g\| g.as_str().to_string()).unwrap_or_default())` | **PENDENTE** |
| **11** | **`color.mix` em sRGB difere em 1 unidade no canal verde**<br>Arredondamento na conversão sRGB (`#805a88` no vanilla vs `#805b88` no cristalino). | **Baixa** | `01_core/src/entities/color.rs:671-678` | Interpolação linear de componentes em `mix()` com conversão `to_srgb()` | **PENDENTE** |
| **12** | **`len` global é extensão do cristalino**<br>Conta codepoints em vez de bytes. Na linguagem Typst oficial, `len` não existe no escopo global (é o método `.len()` que opera em bytes). | **Baixa** | `01_core/src/compiler/eval/mod.rs:1471`<br>`01_core/src/compiler/stdlib/foundations/len.rs:19` | `scope.define("len", Value::Func(Func::native("len", native_len)))` | **PENDENTE** |

---

## 3. Passo 2 — Planejamento para a Criação dos Prompts L0 Individuais

Nenhum L0 foi escrito neste passo, conforme o critério de conclusão. No próximo passo, cada um dos 6 achados receberá um arquivo de especificação L0 individual em `00_nucleo/prompts/` (ou `00_nucleo/materialization/`) com:
1. **Medição diferencial exata** contra o compilador Typst Vanilla (`lab/typst-original/`).
2. **Gate `ADR-0127`** (pois todos afetam a saída de layout ou semântica por defeito da linguagem).
3. **Plano de verificação e testes unitários/diferenciais**.

### Ordem Sugerida de Prioridade para os Próximos Passos:
1. **Bloco Média (Comportamento Visível / Layout)**:
   * **P1072 (Achado #8)**: Corrigir default de `body_indent` para `0.5em` em listas e enumerações.
   * **P1073 (Achado #7)**: Implementar supplements canônicos de `ref` via `LocalName` (`Figure`, `Equation`).
   * **P1074 (Achado #9)**: Herdar `italic` no layout de `Heading` (remover `italic: false` forçado).
   * **P1075 (Achado #10)**: Retornar `None` para grupos de captura regex não participantes.
2. **Bloco Baixa (Ajustes de Precisão / Extensões)**:
   * **P1076 (Achado #11)**: Ajustar precisão de arredondamento sRGB em `color.mix`.
   * **P1077 (Achado #12)**: Harmonizar / deprecar a função global `len` versus o método `str.len()`.

---

## 4. Conclusão e Validação

* O relatório do P1031 foi lido diretamente da fonte factual sem reconstrução por memória.
* Os 6 achados de Média/Baixa foram isolados, auditados e confirmados como 100% pendentes no código atual.
* Nenhum arquivo de código foi alterado.
* `crystalline-lint .`: APROVADO (0 erros, 0 avisos).
* `cargo test --workspace`: APROVADO (5.942 testes, 100% PASS).
