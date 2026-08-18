# Relatório de Execução — Passo 1078: Divergência `Seção`/`Secção` em Português — Achado Lateral do P1073 (Origem P788)

**Data**: 2026-08-18  
**Passo**: 1078 — Divergência `Seção`/`Secção` em Português (Achado Lateral do P1073 / Origem P788)  
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Paridade com a Linguagem Typst)  
**Status**: CONCLUÍDO COM ÊXITO (100% IDÊNTICO ao Vanilla Typst)

---

## 1. Contexto e Motivação

Durante a homologação do Passo 1073 (unificação dos suplementos em `@f`, `@eq` e `@t`), detectou-se uma divergência pré-existente na resolução do suplemento de cabeçalho (`heading`) em documentos no idioma português (`#set text(lang: "pt")`):
- O Vanilla Typst utiliza `translations/pt.txt`, onde `heading = Seção` (sem \"c\").
- O Crystalline (desde o Passo 788) possuía o valor hardcoded `Secção` (com \"c\", variante de Portugal pré-acordo / `pt-PT.txt`).

O Passo 1078 realizou a investigação sistemática das tabelas de tradução do Vanilla Typst e reconciliou o comportamento padrão do Crystalline para atingir paridade estrita.

---

## 2. Investigação e Resolução dos Critérios do L0

### 2.1 Análise das Tabelas de Tradução no Vanilla Typst (§1 do L0)
A inspeção em `lab/typst-original/crates/typst-library/translations/` confirmou a coexistência de dois arquivos:
1. **`translations/pt.txt`** (Tabela geral / padrão para `lang: "pt"`):
   ```text
   heading = Seção
   outline = Sumário
   raw = Listagem
   page = página
   footnote = Nota de rodapé
   email = Correio Eletrônico
   telephone = Telefone
   ```
2. **`translations/pt-PT.txt`** (Tabela regional para `lang: "pt", region: "PT"`):
   ```text
   heading = Secção
   outline = Índice
   email = Correio Eletrónico
   figure = Figura
   table = Tabela
   equation = Equação
   bibliography = Bibliografia
   ```

A resolução do Vanilla Typst (`crates/typst-library/src/text/lang.rs:620-645`) faz fallback de `(lang, region)` para `(lang, None)`. Logo:
- `#set text(lang: "pt")` sem região especificada resolve para `translations/pt.txt` $\\to$ **`Seção`**.
- `#set text(lang: "pt", region: "PT")` resolve para `translations/pt-PT.txt` $\\to$ **`Secção`**.

### 2.2 Localização e Reconciliação no Crystalline (§2 e §3 do L0)
O valor `"Secção"` estava presente em:
1. `01_core/src/compiler/layout/references.rs:262` (`default_supplement_for_key`):
   - Atualizado para `"Seção"`.
2. `01_core/src/compiler/introspect/labelled.rs:34` (`compute_labelled`):
   - Atualizado para diferenciar idioma: `"Seção"` quando `lang == "pt"` e `"Section"` quando `lang == None` (default `en`).
3. Testes legados de introspecção sem `#set text(lang: "pt")` que esperavam `"Secção 1"` foram atualizados para `"Section 1"` (conforme a regra de que o default da linguagem é inglês `en`).

---

### 2.3 Delimitação de Escopo: Suporte à Região Linguística (`region`)
A investigação identificou que o Vanilla Typst opera com um par de chaves `(Lang, Option<Region>)`. No entanto, no Crystalline atual:
- O tipo [`Lang`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/lang.rs) modela exclusivamente códigos ISO 639 de língua (2 a 3 caracteres).
- A entidade [`Region`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/region.rs) no Crystalline é um descritor geométrico de layout 2D (bounding box de colunas/páginas), não existindo ainda um tipo de região geográfica/linguística (ISO 3166-1 alpha-2).
- No avaliador de regras ([`01_core/src/compiler/eval/rules.rs:136`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/eval/rules.rs#L136)), `region` está catalogada em `VANILLA_TEXT_SET_PROPS`, mas tratada como **scope-out explícito sob ADR-0040** (emitindo warning de propriedade ainda não implementada).
- A cadeia de estilos ([`TextStyle`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/layout_types.rs), [`StyleDelta`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/entities/style.rs), `StyleChain`) não transporta um campo de região linguística.

**Conclusão do Escopo**:
O Passo 1078 corrigiu a divergência do idioma base `pt` (que estava incorretamente fixado com a grafia de `pt-PT.txt` em vez da tabela padrão `pt.txt`). A introdução de suporte completo a variantes regionais (`#set text(region: "PT")` $\\to$ `Secção`) é uma extensão arquitetural de maior escala que exige a introdução do tipo `Region` linguístico no sistema de estilos (ADR-0040) e tabelas regionais dedicadas.

## 3. Medição Comparativa e Paridade

### 3.1 Documento de Referência em Português
Código testado:
```typst
#set heading(numbering: "1.")
#set text(lang: "pt")

= Introdução <h1_br>
Veja a @h1_br.
```

### 3.2 Tabela de Resultados Observáveis

| Elemento / Caso | Vanilla Typst (`/usr/local/bin/typst`) | Crystalline (Antes P1078) | Crystalline (P1078) | Paridade |
| :--- | :--- | :--- | :--- | :---: |
| Heading 1 | `1. Introdução` | `1. Introdução` | `1. Introdução` | **100% IDÊNTICO** |
| Referência `@h1_br` | `Veja a Seção 1.` | `Veja a Secção 1.` ❌ | `Veja a Seção 1.` | **100% IDÊNTICO** |
| Referência em Inglês (`@h_en`) | `See Section 1.` | `See Section 1.` | `See Section 1.` | **100% IDÊNTICO** |

---

## 4. Arquivos Afetados e Alterações

- [`01_core/src/compiler/layout/references.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/references.rs):
  - Suplemento padrão de `heading` para `lang: "pt"` alterado de `"Secção"` para `"Seção"`.
- [`01_core/src/compiler/introspect/labelled.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/labelled.rs):
  - `compute_labelled` sensibilizado para `lang` (`"Seção"` em `pt`, `"Section"` em `en`/default).
- [`01_core/src/compiler/layout/tests.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/tests.rs):
  - Adicionado teste unitário `p1078_ref_heading_supplement_pt_secao`.
  - Atualizada asserção do teste legacy `labelled_walk_emite_tag_e_popula_introspector` para `"Section 1"`.
- [`01_core/src/compiler/introspect.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/introspect.rs):
  - Atualizadas asserções de testes legados em inglês sem `lang` explícito para `"Section 1"` / `"Section 2"`.

---

## 5. Validação e Qualidade

- `cargo test --workspace`: **5.950 testes aprovados (100% PASS)**.
- `crystalline-lint .`: **0 erros, 0 avisos de drift**.
- Compilação binária `typst` em modo `--release` validada contra o compilador oficial de referência.
