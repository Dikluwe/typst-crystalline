# Passo 132A — Diagnóstico para materialização de `FontList`

**Série**: 132A (passo **micro-S** em L0; primeiro de dois
sub-passos para materializar tipo composto de `font` em L1).
**Precondição**: Passo 131B encerrado; 1069 total tests; zero
violations; 52 ADRs activas (ADR-0052 `IMPLEMENTADO`); 11
DEBTs abertos. `StyleDelta` não tem campo `font`.
**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional com vanilla).
- **ADR-0034** (diagnóstico obrigatório — este passo cumpre).
- **ADR-0036** (atomização progressiva).
- **ADR-0037** (coesão por domínio).
- **ADR-0038** — pode ganhar quinta nota conforme decisão do
  diagnóstico.

**Natureza**: passo L0. **Sem código**. **Sem testes**. Produz
documento de diagnóstico + ADR proposta.

---

## Contexto

Passo 131 estabeleceu precedente: propriedades com semântica
complexa no vanilla exigem materialização em L1 (tipo dedicado,
validação, paridade ADR-0033), não captura raw com divergência
diferida.

`text.font` é o caso mais complexo da lista DEBT-1. Vanilla
aceita 3 formas:

1. **String simples**: `#set text(font: "Arial")`.
2. **Array de famílias**: `#set text(font: ("Inria Serif", "Noto Sans Arabic"))`.
3. **Dict com coverage**: `#set text(font: ((name: "PT Sans", covers: regex("[0-9]")), "Libertinus Serif"))`.

Materializar `FontList` (ou equivalente) é pré-requisito para
capturar `font` sem violar ADR-0033.

Este passo cumpre ADR-0034 antes de escrever código no 132B.

---

## Contexto estratégico

Sub-passo 1 de 5 restantes para fechar DEBT-1 (roadmap revisto
após 131):

- **132A**: diagnóstico `FontList` (este).
- **132B**: materialização `FontList` + `StyleDelta.font`
  + arm com validação + substituição canary DEBT-50 para
  `hyphenate`.
- **133**: activar target `par` em `eval_set_rule`.
- **134**: migrar `leading` de `text` para `par`.
- **135**: fechar DEBT-1 no DEBT.md.

132B só pode ser enunciado depois de 132A encerrar e o
diagnóstico ser aprovado.

---

## Decisões já tomadas (entrada do passo)

1. **Paridade total**: aceitar as 3 formas do vanilla (string,
   array, dict com `covers`). Sem divergência semântica.
2. **Materialização em L1**: `FontList` (ou nome escolhido no
   diagnóstico) vive em L1.
3. **Erro hard em inválido**: consistente com Passo 131
   (`Lang`). Valores que não cabem nas 3 formas produzem Err.
4. **Novo canary DEBT-50**: `hyphenate` substitui `font` no
   132B. Diagnóstico regista como decisão; migração física
   acontece em 132B (atómica com a captura de `font`).
5. **Âmbito do diagnóstico**: só `FontList` + entidades
   necessárias (provavelmente `FontFamily` ou similar).
   **Não** inclui `FontInfo` do catálogo, `Font` runtime,
   shaping, fallback lookup — tudo isso são camadas futuras.

## Decisões diferidas (para o diagnóstico resolver)

6. **Nome do tipo em L1**: `FontList`, `FontFamilies`,
   `FontSelector`, conforme vanilla ou nome próprio.
7. **Forma interna**: decidir com base na leitura do vanilla —
   provavelmente `Vec<FontFamily>` onde `FontFamily` é enum
   `{ Plain(EcoString), Covered { name: EcoString, covers: ... } }`
   ou struct com `Option<...>`.
8. **`covers` — `regex` support**:
   - Vanilla usa `regex::Regex` (crate externa). Precisa
     verificar se `regex` está autorizado em L1 (ADR-0018
     lista). Se não, decisão arquitectural extra.
   - Alternativa: capturar `covers` como `EcoString` raw e
     validar na compilação do regex em layer superior.
     Divergência ADR-0033 — preferir não.
9. **Nomes especiais do vanilla** (ex: `"latin-in-cjk"` como
   `covers`): enumerar variantes especiais.
10. **Localização em L1**:
    - `01_core/src/entities/font_list.rs` (novo).
    - `01_core/src/entities/font_book.rs` (existente com
      `FontWeight`, mas domínio é catálogo, não selecção).
    - Ficheiro próprio é provável. Decidir no diagnóstico.

---

## Sub-passos

### 132A.1 — Inventário vanilla

**Leitura de `lab/typst-original/`**:

1. Localizar tipo de `font`:
   - `grep -rn "pub font:" lab/typst-original/crates/typst-library/src/text/`
   - Candidato: `TextElem.font: FontList` (ou similar).
2. Ler ficheiro principal de `FontList`:
   - Declaração, campos, derives.
   - Impl blocos, métodos públicos.
   - `impl FromValue` / `impl IntoValue` / `cast!`.
   - Constantes pré-definidas (se algumas).
3. Ler entidade secundária (`FontFamily` ou equivalente):
   - Se é enum ou struct.
   - Variantes / campos.
   - Como representa coverage.
4. Registar todas as crates referenciadas em imports do
   ficheiro (especial atenção a `regex`).

### 132A.2 — Inventário cristalino

1. `grep -rn "FontList\|FontFamily\|FontSelector" 01_core/src/`
   — confirmar que tipo não existe ainda.
2. Listar tipos tipográficos já em L1:
   - `FontWeight` (`entities/font_book.rs`).
   - `Length`, `Color`, outros.
3. Verificar `01_core/Cargo.toml`:
   - Crates já autorizadas em `[l1_allowed_external]`.
   - Se `regex` está autorizada. Se não, verificar se há
     outra crate regex-like autorizada.
   - Se nenhuma crate regex autorizada: decisão arquitectural
     extra a documentar.
4. Confirmar `EcoString` já é `Value::Str` (ADR-0024 —
   pré-requisito do Passo 131B, continua válido).

### 132A.3 — Inventário pool DEBT-49 e canary

**Objectivo**: confirmar estado actual do pool para saber o que
fica após 132B captura `font`.

1. `grep -n "font\|lang\|alignment\|stroke" 03_infra/src/integration_tests.rs`
   — estado do pool actual.
2. Registar:
   - Testes que usam `font` como propriedade desconhecida
     (serão afectados em 132B).
   - Testes que usam outras propriedades.
3. Confirmar que `hyphenate` não aparece em nenhum teste L3
   actualmente — se aparece, o canary novo não é livre de
   colisão e precisa de escolha alternativa.

### 132A.4 — Inventário testes L1 com canary actual

1. `grep -n "font_canary\|#set text(font:" 01_core/src/rules/eval/tests.rs`
   — todos os testes que usam `font` como canary.
2. Registar lista de testes afectados em 132B:
   - Testes que testam captura positiva de outras props e
     usam `font` como canary no mesmo ficheiro.
   - Testes dedicados ao canary.
3. Estimar impacto da migração de canary em número de testes.

### 132A.5 — Escrever diagnóstico

Ficheiro:
`00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`.

**Template dos 7 itens mínimos (ADR-0034)**:

```markdown
# Diagnóstico de `FontList` — Passo 132A

**Data**: 2026-MM-DD
**ADR alvo**: ADR-NNNN (a criar em 132A.6) — "Font como tipo
composto em L1".
**Motivação**: `text.font` é propriedade da lista DEBT-1 pendente.
Vanilla aceita 3 formas (string/array/dict com covers).
Materializar tipo composto em L1 para paridade ADR-0033, aplicando
precedente 131B.

---

## 1. Localização no vanilla

- Ficheiro principal: `lab/typst-original/crates/typst-library/src/text/...`
  (confirmar).
- Linhas da declaração de `FontList`: [X:Y].
- Declaração de `FontFamily` (ou equivalente): [X:Y].
- `TextElem.font` em `text/mod.rs` linha [X].

## 2. Campos / variantes

- `FontList`: declaração exacta + derives.
- `FontFamily` (ou equivalente): enum? struct? Variantes e payloads.
- Semântica da lista: prioridade? fallback? ordem.

## 3. Operadores / métodos

- Métodos públicos de `FontList`.
- Iteração sobre famílias.
- `impl FromValue` / cast! — copiar conteúdo.
- `Display` / `Debug`.

## 4. Dependências

- Crates externas:
  - `ecow::EcoString` — confirmado autorizado.
  - `regex::Regex` (para `covers`) — **verificar autorização**.
  - Outras?
- Tipos internos vanilla referenciados.
- Decisão: se `regex` não está autorizada, opção A (adicionar à
  allowlist + ADR) vs opção B (tipo próprio de coverage).

## 5. Semântica

- **Forma string simples**: mapeamento para `FontList` de 1
  elemento com covers=None.
- **Forma array**: lista de famílias com prioridade.
- **Forma dict com covers**: família com regex filter sobre
  codepoints.
- **Casos especiais**: `covers: "latin-in-cjk"` é string especial
  reconhecida como keyword?
- **Fallback chain**: semântica do array (primeira família que
  cobre o glyph, última opção se nenhuma cobre).

## 6. Mensagens de erro

- Forma exacta das mensagens do vanilla para cada falha:
  - Valor não é string nem array nem dict.
  - Dict sem `name`.
  - Dict com `covers` inválido.
- Hints.

## 7. Divergências propostas para L1

- **Forma interna**: decisão sobre struct/enum.
- **`covers` representation**:
  - Replicar `regex::Regex` se autorizado.
  - Ou tipo dedicado (ex: `enum Coverage { Regex(...), Keyword(...), None }`).
- **Keywords especiais**: `"latin-in-cjk"`, etc. — enumerar.
- **Erro hard em inválido**: alinhado Passo 131 precedente.
- **Constantes**: provavelmente nenhuma (nomes de fonte variam).
- **Localização**:
  - `01_core/src/entities/font_list.rs` (novo, recomendado).
  - Razão: coesão por domínio (ADR-0037). `font_book.rs`
    tem `FontWeight` que é variante tipográfica; `FontList` é
    selector de fonte — domínios diferentes.

---

## Itens adicionais

### Impacto em call-sites

- `01_core/src/entities/style_chain.rs`:
  - Adicionar `use crate::entities::font_list::FontList;`.
  - `StyleDelta.font: Option<FontList>` — novo campo.
  - `StyleDelta::empty()` ganha `font: None`.

- `01_core/src/rules/eval/rules.rs`:
  - Adicionar arm `"font"` com validação via construtor de
    `FontList`.
  - Arm emite Err hard em inválido (precedente 131B).

- Ficheiros com canary DEBT-50:
  - Lista concreta em 132A.4.
  - Cada teste renomeado para `hyphenate_canary_passo_132b`
    e input alterado de `#set text(font: "X")` para
    `#set text(hyphenate: true)` (ou outro valor legal que
    o parser aceite como sintaxe).

- `03_infra/src/integration_tests.rs`:
  - Tests DEBT-49 rotação: `font` deixa o pool; substituto a
    decidir em 132B (candidato: `justify`, `first-line-indent`,
    ou `dir`).

### Plano de teste para 132B

**Unit tests em `entities/font_list.rs`** (esperados 15-20):

- `font_list_single_string_aceita`: `"Arial"` → FontList de 1
  elemento.
- `font_list_array_aceita_multiple`.
- `font_list_array_vazio_devolve_erro` (se vanilla assim).
- `font_list_dict_com_name_aceita`.
- `font_list_dict_com_covers_regex_aceita`.
- `font_list_dict_com_covers_keyword_aceita` ("latin-in-cjk").
- `font_list_dict_sem_name_devolve_erro`.
- `font_list_dict_com_covers_invalido_devolve_erro`.
- Display tests.

**Integration tests em `rules/eval/tests.rs`** (esperados 3-4):

- `eval_set_text_font_string_simples_passo_132b`.
- `eval_set_text_font_array_passo_132b`.
- `eval_set_text_font_dict_passo_132b`.
- `eval_set_text_font_invalido_emite_erro_passo_132b`.

**Canary substituição**:
- `eval_set_text_hyphenate_canary_passo_132b` (substitui
  todos os testes `font_canary_passo_*`).

### Plano de migração de canary

- Identificar N testes actuais com `font_canary_passo_XXX` em
  `rules/eval/tests.rs`.
- Cada um: renomear para `hyphenate_canary_passo_132b` e alterar
  input `#set text(font: "X")` → `#set text(hyphenate: true)`
  (ou sintaxe equivalente — confirmar tipo de `hyphenate` no
  vanilla: `bool`, `auto`, `none`, ou outro).
- Assertion: mensagem de warning contém `'hyphenate'` em vez de
  `'font'`.

### Plano de migração DEBT-49 L3

Post-132B: pool `font/alignment/stroke` → `???/alignment/stroke`.
Candidato substituto para slot de `font`:
- `justify` — bool, sintaxe simples.
- `first-line-indent` — length, sintaxe comum.
- `dir` — direction, menos comum.
Decidir em 132B com base no que cabe cleanly no input de teste.
```

### 132A.6 — Escrever ADR proposta

Ficheiro:
`00_nucleo/adr/typst-adr-NNNN-font-tipo-composto.md`.

**Primeiro**: determinar número. Após 131B, ADR-0052 é a última
atribuída e está `IMPLEMENTADO`. Próximo candidato: **ADR-0053**.
Confirmar com `ls 00_nucleo/adr/typst-adr-*.md | sort | tail -5`.

Template (seguindo estrutura das ADRs recentes, especialmente
ADR-0052):

```markdown
# ⚖️ ADR-NNNN: Font como tipo composto em L1

**Status**: `PROPOSTO`
**Data**: 2026-MM-DD
**Diagnóstico prévio**:
`00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`

---

## Contexto

`text.font` é a propriedade residual mais complexa da lista
DEBT-1. Vanilla aceita string simples, array, e dict com
coverage (regex ou keyword). Captura raw como `EcoString`
violaria ADR-0033 no mesmo espírito do Passo 130 (lang),
resolvido no 131. Este ADR propõe materialização análoga à
do `Lang` (ADR-0052) para `font`.

## Decisão

1. Materializar `FontList` em L1 com forma <a definir no
   diagnóstico>.
2. Materializar `FontFamily` (ou equivalente) como auxiliar.
3. Parse e validação equivalente ao vanilla.
4. Erro hard em valor inválido (paridade 131B).
5. `StyleDelta.font: Option<FontList>` — novo campo.
6. Localização: <ficheiro a definir>.
7. `covers`: replicar como vanilla (`regex::Regex` se autorizada)
   ou tipo dedicado se não.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Capturar só string simples | XS | Viola ADR-0033; divergência activa |
| Materializar `FontList` completo | Paridade total | S-M, tipo composto |
| Tipo simplificado sem covers | S | Perde funcionalidade, divergência parcial |

## Consequências

**Positivas**:
- ADR-0033 satisfeito para toda a lista DEBT-1.
- Base para consumer futuro (font resolution, shaping).
- Precedente 131B reaplicado — disciplina validada.

**Negativas**:
- Custo de migração (132B = S-M, maior que 131B).
- Se `regex` precisa autorização nova, ADR complementar.

**Neutras**:
- `StyleDelta` ganha campo complexo (primeira vez com tipo
  composto agregando outros tipos).

## Referências

- ADR-0033 (paridade).
- ADR-0034 (diagnóstico).
- ADR-0052 (precedente `Lang`).
- Passo 131B (materialização análoga, relatório).
- Passo 132A (este diagnóstico).
```

---

## Verificação

1. Diagnóstico criado com 7 itens mínimos preenchidos com factos
   do vanilla.
2. ADR-NNNN criada com `Status: PROPOSTO`.
3. Decisão sobre `regex` tomada (autorizada vs tipo dedicado) e
   documentada no diagnóstico.
4. Lista concreta de testes L1 afectados pela migração de canary
   (132A.4).
5. Estado do pool DEBT-49 L3 documentado com substituto
   proposto (132A.3).
6. Nenhum ficheiro em `01_core/src/`, `02_shell/src/`,
   `03_infra/src/`, `04_wiring/src/` tocado.
7. `cargo test --workspace` continua em 1069, 6 ignorados.
8. `crystalline-lint` zero violations.

---

## Critério de conclusão

1. `diagnostico-font-list-passo-132a.md` existe e tem todos os
   7 itens do ADR-0034 preenchidos com dados reais.
2. `typst-adr-NNNN-font-tipo-composto.md` existe com
   `Status: PROPOSTO`.
3. Diagnóstico referencia a ADR (`ADR alvo:`).
4. ADR referencia o diagnóstico (`Diagnóstico prévio:`).
5. ADR lista alternativas consideradas com prós/contras.
6. Decisão sobre `regex` (autorizada ou tipo dedicado) tem
   secção dedicada no diagnóstico.
7. Lista de testes L1 canary identificada (nomes e localizações).
8. Substituto DEBT-49 L3 proposto.
9. Zero ficheiros de código tocados.
10. `cargo test --workspace` inalterado (1069, 6 ignorados).
11. `crystalline-lint` zero violations.
12. Relatório 132A.K escrito.

---

## O que pode sair errado

- **`regex` não está em `[l1_allowed_external]`**: decisão
  extra. Opções:
  - Adicionar com ADR complementar (precedente: `ecow`,
    `rustc-hash`, `indexmap`).
  - Tipo dedicado `Coverage` que não usa regex (perde
    funcionalidade exacta vanilla).
  - Deferir `covers` suporte para passo futuro e capturar só
    forma string + array sem covers.
  Diagnóstico recomenda uma das três; 132B executa.

- **`FontList` vanilla tem métodos complexos** (ex: iteração,
  lookup integrado com `FontBook`): replicar todos excede S.
  Diagnóstico registra subset mínimo para 132B; restantes
  ficam como candidatos futuros (ligados a consumer).

- **Keywords especiais de covers** (`"latin-in-cjk"`, etc.)
  são mais do que 2-3: mapeamento pode ser longo. Diagnóstico
  regista lista completa; 132B implementa tabela.

- **`TextElem.font` não é `FontList` directamente mas outro
  tipo agregador**: ajustar nome do tipo a materializar.

- **Número de ADR incorrecto**: se 132A assume 0053 e real é
  outro, executor corrige ao criar.

- **Pool DEBT-49 L3 não tem substituto óbvio**: se `alignment`
  e `stroke` são os únicos disponíveis e `font` saindo reduz
  pool para 2, pode precisar de substituto vindo de lista
  mais distante (ex: `dir`, `region`). Diagnóstico propõe.

---

## Notas operacionais

- Segunda aplicação do padrão "diagnóstico separado +
  materialização" após Passo 131. Se sai limpo, valida o padrão
  como disciplina reusável para futuras materializações
  (`Region`, `Dir`, `Stroke`, etc.).

- **`FontList` é tipo agregador**: precedente 131B foi tipo
  "folha" (`Lang` não contém outros tipos L1). `FontList`
  provavelmente contém `FontFamily` ou similar. Primeira
  materialização composta — diagnóstico atenção especial a
  **quantos tipos novos** são necessários.

- Se diagnóstico revela que materializar `FontList` exige mais
  de 1 tipo novo em L1 (ex: `FontList` + `FontFamily` +
  `Coverage`), **parar e reportar**. 132B pode precisar de
  ser desdobrado em sub-passos adicionais (132B.1 tipos,
  132B.2 captura, etc.).

- **Canary migration é parte explícita do 132B**: diagnóstico
  lista testes afectados para executor do 132B seguir. Lista
  incompleta = janela sem canary depois da captura, o que
  invalida o propósito do canary.

- **Pattern "só diagnóstico" está a ganhar consistência**:
  131A e 132A ambos L0-puros com diagnóstico + ADR proposta.
  Se 132A sai limpo, formalizar no ADR-0034 como sub-padrão
  reusável é candidato futuro.
