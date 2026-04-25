# Passo 144 — Lang hyphenation (gap 7 DEBT-52; ADR-0057 + consumer)

**Série**: 144 (passo **L1 + L3 + ADR**; reabertura parcial
do trabalho opcional de DEBT-52 pós-fecho de DEBT-1).
**Precondição**: Passo 145 encerrado; cabeçalhos dos ADRs
0017, 0027, 0028 e 0038–0051 uniformizados; 56 ADRs no índice
canónico; 10 DEBTs abertos; DEBT-1 e DEBT-52 fechados.

**Numeração**: 144 estava reservado para este passo (gap 7
hyphenation) desde o relatório 142 §11 e formalmente
re-confirmado no relatório 143 §9.

**Modo**: **tudo-num-passo** (decisão tua). ADR de
autorização de crate + materialização do consumer numa única
unidade. **Sem diagnóstico prévio formal** — o inventário
empírico fica condensado nos sub-passos 144.1 e 144.2 e os
seus resultados informam directamente as decisões da ADR e do
código.

**ADR criada**: **ADR-0057** — "Hyphenation em L1 via crate
`hyphenation` ou `hypher`". Status final dependente do
inventário (provável `IMPLEMENTADO` se materialização
concluir; `PROPOSTO` se algum bloqueio surgir). Número 0057
escolhido por ser o seguinte a ADR-0055 (0056 está
reservado pelas notas operacionais do 142 e 145 para
"subsetting de fonts" — candidato distinto).

**Natureza**: passo **substantivo**. Toca:
- L1 (`01_core/`): possível accessor novo em `Lang` se for
  necessário expor códigos ISO; consumer hyphenation no
  pipeline de quebra de linha.
- L3 (`03_infra/`): se a crate exigir carregamento de padrões
  do filesystem ou descoberta de recursos.
- L0 (prompts): spec do consumer.
- ADR (`00_nucleo/adr/`): ADR-0057.
- `DEBT.md`: gap 7 marcado `[x]`.
- `Cargo.toml`: dependência nova autorizada.

**ADRs aplicáveis**:
- **ADR-0034** (diagnóstico obrigatório para tipos vanilla) —
  hyphenation **não é tipo vanilla**, é mecanismo. ADR-0034
  não obriga estritamente, mas o **espírito** da regra
  (inventário antes de decisão) aplica-se. Cumprido por
  144.1+144.2 condensados.
- **ADR-0029** (pureza física de L1) — se a crate de
  hyphenation acede ao filesystem para carregar padrões, a
  parte de I/O fica em L3; só dados puros (padrões já em
  memória) atravessam para L1.
- **ADR-0030** (performance é domínio de L1) — padrões
  pré-compilados em RAM são domínio L1 puro (análogo a `Arc`
  em struct de domínio).
- **ADR-0033** (paridade funcional) — output observacional:
  pontos de quebra de linha equivalentes ao vanilla para
  inputs de teste documentados.
- **ADR-0054** (perfil observacional graded) — gap 7 estava
  scope-out pelo perfil; este passo materializa-o
  voluntariamente. **Não invalida** ADR-0054; apenas reduz a
  superfície de scope-out residual.
- **ADR-0018** (critério de autorização externa) — ADR-0057
  invoca-a para justificar autorização da crate escolhida.

---

## Contexto

DEBT-52 fechou em 142 com 6/8 gaps materializados e 2/8
explicitamente scope-out (gap 7 lang hyphenation; gap 8 font
dict). Relatório 142 §11 listou ambos como candidatos
futuros não-DEBT.

Decisão tua de Abr 2026 prioriza gap 7. Razão pragmática
provável (não confirmada): textos longos em PT/EN/ES/FR
beneficiam visivelmente de hyphenation; sem ela, justificação
de parágrafos produz "rios" de espaço branco quando palavras
longas não cabem na linha.

**Estado actual** do consumer `lang` segundo o relatório 142
§3:

> `lang` | `Option<Lang>` | (sem consumer activo) | — |
> **scope-out** (perfil observacional graded; §4)

`Lang` é tipo semântico desde Passo 131B (ADR-0052). Captura
+ validação + tipo: presente. Consumer: ausente.

**Pontos de integração antecipados** (a confirmar em 144.2):
- `flush_line` em `03_infra/` — onde leading foi materializado
  (Passo 138). Quebra de linha é vizinha conceitual.
- Algoritmo de quebra de linha (greedy ou Knuth-Plass?). Em
  cristalino, segundo passos anteriores (137 word_width
  + tracking), o algoritmo é greedy simples. Hyphenation em
  greedy é mais simples que em Knuth-Plass — bom sinal.

**Crates candidatas** (a comparar empiricamente em 144.1):
- **`hyphenation`** (crate atom-hyphenation) — padrões TeX
  pré-compilados como dados estáticos; ~4MB de padrões
  embebidos; runtime O(n) via Liang's algorithm; `no_std`
  capable.
- **`hypher`** — alternativa mais recente; foco em ser
  pequena; padrões TeX igualmente; API mais minimalista.

A escolha empírica, dado que cristalino prefere
"compromisso minimalista quando paridade observacional o
permita" (ADR-0054 perfil graded), provavelmente recai em
**`hypher`** — confirmar em 144.1.

---

## Objectivo

Ao fim do passo:

1. **Inventário empírico** das duas crates registado no
   relatório:
   - Tamanho de cada (lib + padrões), deps transitivas,
     features.
   - API: como se obtém pontos de quebra de uma palavra dado
     um idioma.
   - Suporte a idiomas (PT/EN/ES/FR no mínimo; idealmente
     conjunto coberto pelo TeX hyph-utf8).
   - I/O: padrões são embebidos (compile-time include) ou
     carregados em runtime?

2. **ADR-0057** criada com `Status: IMPLEMENTADO` (ou
   `PROPOSTO` se materialização ficar incompleta) com:
   - Decisão de crate (`hypher` ou `hyphenation`).
   - Localização das deps (L1 se padrões são pure-data e
     embebidos; L3 se há I/O).
   - Pipeline de consumo (`Lang` → padrões → pontos de
     quebra).
   - Política de fallback (idioma sem padrões → sem
     hyphenation; documento sem `lang` → sem hyphenation).

3. **Crate autorizada** em `Cargo.toml` do workspace e nos
   crates relevantes (`01_core/Cargo.toml` e/ou
   `03_infra/Cargo.toml` consoante ADR-0057). `crystalline.toml`
   actualizado se a crate fica em L1.

4. **Consumer hyphenation** materializado no pipeline de
   quebra de linha:
   - Função pura em L1 (ou L3 se I/O é necessário): dado
     `(palavra, lang)`, devolve pontos de quebra.
   - Integração no algoritmo greedy de quebra de linha:
     quando uma palavra não cabe, tenta inserir hífen num
     ponto de quebra autorizado por hyphenation.
   - Output PDF reflecte: hífen literal "-" inserido na
     posição de quebra; resto da palavra na linha seguinte.

5. **Tests**:
   - Unitários: pontos de quebra para palavras concretas em
     PT/EN (ex: "exemplo" → ["e-xem-plo"]; "hyphenation" →
     ["hy-phen-ation"]).
   - Integração L3: documento com `#set text(lang: "pt")` +
     palavra longa em coluna estreita → PDF com hífen
     visível na quebra.
   - Regressão: documento sem `#set text(lang: ...)` →
     comportamento inalterado (sem hyphenation, como antes).

6. **DEBT-52 gap 7 marcado `[x]`** em `DEBT.md` — secção 2
   ("encerrados"), na entrada do DEBT-52. Reabertura formal
   de DEBT-52 **não acontece** — o DEBT continua encerrado;
   adiciona-se actualização ao histórico análoga às
   actualizações de DEBT-1 (Passos 140B/141/142).

7. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-144-relatorio.md`.

8. **README dos ADRs** actualizado com ADR-0057 na tabela
   "Estado por ADR" + entrada na secção "Passos-chave da
   história dos ADRs". (Análogo ao tratamento dado em
   passos administrativos anteriores.)

Este passo **não**:

- Toca DEBT-1 (encerrado). Apenas anota actualização ao
  histórico de DEBT-52.
- Implementa Knuth-Plass ou outro algoritmo de quebra de
  linha sofisticado. Continua greedy.
- Implementa shaping features sensíveis a script (rustybuzz).
  `lang` continua **parcialmente** scope-out: o componente
  hyphenation ganha consumer; o componente shaping
  permanece sem consumer (DEBT-53 candidato XL futuro).
- Materializa gap 8 (font dict).
- Cria ADR-0055bis (variant-aware) ou ADR-0056 (subsetting).

---

## Decisões já tomadas

1. **Tudo-num-passo**, não diagnóstico-primeiro. Compromisso
   conhecido: menos formalização prévia, mas o inventário
   condensado de 144.1 + 144.2 cumpre o espírito de ADR-0034.
2. **ADR-0057 escolhida como número** (não 0056 — reservado
   informalmente para subsetting de fonts conforme notas
   operacionais 142/145).
3. **Reabertura de DEBT-52 não acontece**. Histórico de
   DEBT-52 (já em secção "encerrados" de `DEBT.md`) ganha
   actualização análoga às de DEBT-1. Inventário continua
   coerente.
4. **Sem ADR de revisão de ADR-0054**. ADR-0054 declarou
   scope-out por perfil graded; este passo materializa
   voluntariamente um gap declarado opcional. Não há
   contradição com ADR-0054 que justifique revisão.

## Decisões diferidas (resolvidas neste passo)

5. **Crate escolhida** (`hypher` vs `hyphenation` vs outra):
   resolvida em 144.1 com base em: tamanho, deps, API,
   cobertura de idiomas, I/O.
6. **Localização da crate** (L1 vs L3): se padrões são
   compile-time embebidos e a API é pure-fn, fica em L1
   (ADR-0029 + ADR-0030 permitem). Se requer carregamento
   runtime, fica em L3.
7. **Política para idiomas sem padrões**: silent skip
   (sem hyphenation) ou warning? Decisão: **silent skip**
   por consistência com a política de fallback de fonts
   (silent drop em 140B). Revisitar se utilizadores
   reportam confusão.
8. **Política para documento sem `lang`**: silent skip.
   Sem `lang` declarado, hyphenation não é aplicado.
9. **Posição do hífen no PDF**: hífen literal "-" inserido
   antes da quebra. Não há hífen Unicode discreto (`\u{00AD}`
   soft hyphen) no PDF — apenas o "-" literal.
10. **Algoritmo de inserção**: na quebra greedy, quando
    palavra não cabe, **tentar inserir** hífen no último
    ponto de quebra `≤ espaço-disponível`. Se nenhum ponto
    de quebra cabe, palavra inteira passa para linha
    seguinte (comportamento actual preservado).

---

## Escopo

**Dentro**:

- Inventário empírico das duas crates.
- ADR-0057 escrita.
- Edição de `Cargo.toml` (workspace + crates relevantes).
- Edição de `crystalline.toml` se crate fica em L1.
- Função pura `hyphenate(word, lang) -> Vec<usize>` (índices
  de pontos de quebra) em L1 ou L3 conforme ADR-0057.
- Modificação no algoritmo de quebra de linha
  (`flush_line` ou função adjacente) para consumir o
  resultado.
- Tests unitários e de integração.
- Edição de prompts L0 do pipeline afectado.
- Actualização de hash L0/L3.
- Actualização de `DEBT.md` (DEBT-52 secção 2, gap 7
  marcado).
- Actualização do README dos ADRs (tabela + passos-chave).

**Fora**:

- Knuth-Plass ou Best-fit linebreaking.
- Shaping features (rustybuzz).
- Soft hyphen Unicode (`\u{00AD}`) no input do utilizador.
- Hyphenation contextual (proibições por contexto, ex:
  "co-operate" não deve quebrar como "coop-erate").
- Variantes de hyphenation por preferência tipográfica
  (mínimo de letras antes/depois do hífen, etc.) — usar
  defaults da crate.
- Gap 8 (font dict).
- Outras ADRs candidatas (0055bis, 0056).

---

## Sub-passos

### 144.1 — Inventário das crates candidatas

**A.1.1 — Comparar `hypher` vs `hyphenation`**:

```
cargo search hyphenation
cargo search hypher
```

Para cada candidata, registar:
- Versão estável actual.
- Deps transitivas (`cargo tree --package <crate>`).
- Tamanho dos padrões (search nos READMEs / docs.rs).
- Idiomas suportados (PT, EN, ES, FR mínimo).
- API: assinatura de função principal.
- Feature flags relevantes (ex: `hyphenation/embed_all` vs
  `embed_en_us`).
- I/O: padrões embebidos via `include_bytes!` ou loaded em
  runtime?

**A.1.2 — Decisão**:

Escolha justificada no relatório. Critérios em ordem:
1. Pureza (sem I/O > com I/O).
2. Tamanho (menor > maior).
3. Cobertura de idiomas suficiente (PT/EN/ES/FR mínimo).
4. API simples.

Esperado (a confirmar): `hypher` vence em pureza e
tamanho. `hyphenation` vence em maturidade.

### 144.2 — Inventário do ponto de integração

**A.2.1 — Localizar quebra de linha actual**:

```
grep -rn "flush_line\|line_break\|linebreak" 01_core/src/ 03_infra/src/
```

Confirmar:
- Função(ões) que executam a quebra.
- Se algoritmo é greedy puro ou tem alguma sofisticação.
- Onde `Lang` está acessível (StyleChain? TextStyle? Frame?).

**A.2.2 — Localizar emit do glyph "-"**:

`grep -rn "0x2D\|hyphen\|'-'" 03_infra/src/export.rs`.

Verificar como hífen seria emitido se for literal `-`. PDF
deve aceitar como qualquer outro glyph.

**A.2.3 — Confirmar `Lang` API**:

```
view 01_core/src/entities/lang.rs
```

(ou caminho real, descobrir em A.2.1). Confirmar que
`Lang::as_iso_code()` ou similar existe — se não, **adicionar
accessor puro** em L1 sem lógica nova.

### 144.3 — Escrever ADR-0057

Ficheiro: `00_nucleo/adr/typst-adr-0057-lang-hyphenation.md`.

Cabeçalho canónico (P145 conformidade):

```markdown
# ⚖️ ADR-0057: Lang hyphenation em L1 via crate `<escolhida>`

**Status**: `IMPLEMENTADO`
**Data**: 2026-04-24
**Validado**: Passo 144.
**Diagnóstico prévio**: ausente — inventário condensado em §3
deste ADR (modelo "tudo-num-passo").
```

Secções:
- **Contexto** — gap 7 DEBT-52 scope-out reaberto
  voluntariamente; ADR-0054 perfil graded preservado.
- **Inventário condensado** — tabela `hypher` vs
  `hyphenation` (cumpre espírito de ADR-0034).
- **Decisão** — 6 itens análogos à estrutura de ADR-0055:
  1. Crate escolhida.
  2. Localização (L1 ou L3).
  3. Pipeline de consumo.
  4. Política de fallback (idioma sem padrões / documento
     sem `lang`).
  5. Posição e forma do hífen no PDF.
  6. Algoritmo de inserção.
- **Alternativas consideradas** — tabela com `hypher`,
  `hyphenation`, e "implementação própria via tabela TeX
  embebida manualmente" (XL, descartado).
- **Consequências** — positivas (gap 7 fechado, justificação
  visualmente correcta), negativas (deps nova autorizada;
  binário maior em ~Xkb), neutras (`lang` muda de scope-out
  parcial para parcialmente-consumed: hyphenation OK,
  shaping ainda não).
- **Referências** — ADR-0018, 0029, 0030, 0033, 0034, 0052,
  0054.

### 144.4 — Autorizar crate

**A.4.1 — `Cargo.toml` workspace**:

```diff
[workspace.dependencies]
+ <crate-escolhida> = "X.Y"
```

**A.4.2 — Crate consumidora**:

```diff
[dependencies]
+ <crate-escolhida> = { workspace = true }
```

Em `01_core/Cargo.toml` (se L1) ou `03_infra/Cargo.toml` (se
L3) — conforme ADR-0057.

**A.4.3 — `crystalline.toml`** (se L1):

```diff
[l1_allowed_external]
+ <crate-escolhida> = "Hyphenation pure-data via padrões embebidos (ADR-0057)."
```

### 144.5 — Função `hyphenate`

**A.5.1 — Forma**:

```rust
/// Devolve índices (em chars, não bytes) de pontos de quebra
/// na palavra para o idioma dado. Ex: "exemplo" em "pt" →
/// vec![1, 4]  // e-xem-plo.
///
/// Devolve vec vazio se idioma sem padrões ou palavra
/// demasiado curta para hyphenation (default da crate).
pub fn hyphenate(word: &str, lang: &Lang) -> Vec<usize>;
```

Implementação concreta depende da crate escolhida — mapear
`Lang` → enum/string da crate, chamar API, transformar
output em `Vec<usize>` (canonicalização).

**A.5.2 — Tests unitários**:

- `hyphenate_palavra_pt_devolve_pontos_correctos`:
  "exemplo" em "pt" → `vec![1, 4]` (ou o que a crate
  produzir; ajustar assert ao output real e documentar).
- `hyphenate_palavra_en_devolve_pontos_correctos`:
  "hyphenation" em "en" → pontos de quebra esperados.
- `hyphenate_idioma_sem_padroes_devolve_vazio`: idioma
  improvável (ex: "xx") → `vec![]`.
- `hyphenate_palavra_curta_devolve_vazio`: "ao" em "pt" →
  `vec![]` (default min-letters).

### 144.6 — Integração no algoritmo de quebra de linha

**A.6.1 — Modificar `flush_line` (ou função identificada
em A.2.1)**:

Pseudocódigo:

```
fn try_fit_word_with_hyphenation(word, lang, available_width):
    if let break_points = hyphenate(word, lang):
        for &point in break_points.iter().rev():
            let prefix = &word[..point];
            let prefix_with_hyphen = format!("{}-", prefix);
            if width(prefix_with_hyphen) <= available_width:
                return Some((prefix_with_hyphen, &word[point..]));
    None
```

Quando `try_fit_word_with_hyphenation` devolve `Some`,
inserir prefixo+hífen na linha actual; continuar com sufixo
na linha seguinte.

Quando devolve `None`, comportamento actual preservado:
palavra inteira passa para linha seguinte.

**A.6.2 — Tests de integração L3**:

- `lang_hyphenation_pt_palavra_longa_quebra_com_hifen`:
  documento com `#set text(lang: "pt")` + palavra longa em
  coluna estreita → PDF contém hífen na posição esperada.
- `lang_hyphenation_en_paragrafo_largo_reduz_rios`: mesmo
  que o anterior em EN; verificar via inspecção visual ou
  por contagem de hífenes inseridos.
- `lang_hyphenation_sem_set_lang_comportamento_inalterado`:
  documento sem `#set text(lang: ...)` → sem hífenes
  inseridos. Regressão.
- `lang_hyphenation_idioma_sem_padroes_silent_skip`:
  documento com `lang: "xx"` (improvável) → sem hífenes
  inseridos; sem warning; sem erro.

### 144.7 — Edição L0

Prompts a tocar (a confirmar em 144.2):
- `prompts/infra/pipeline.md` (se quebra de linha vive em
  L3) ou
- `prompts/core/layout.md` (se vive em L1).

Adicionar secção descrevendo:
- `hyphenate(word, lang) → Vec<usize>` como helper puro.
- Integração no fluxo de quebra de linha.
- Política de fallback (silent skip).
- Referência a ADR-0057.

Recalcular hashes; actualizar headers de ficheiros consumers.

### 144.8 — Actualizar `DEBT.md`

Em `DEBT.md` Secção 2 (encerrados), localizar entrada
DEBT-52. Adicionar actualização:

```diff
+ ### Actualização Passo 144 — Consumer `lang` hyphenation
+
+ - [x] Gap 7 (lang hyphenation) materializado pós-fecho de
+   DEBT-1. ADR-0057 autoriza crate `<escolhida>` em
+   `<L1|L3>`. `hyphenate(word, lang)` invocada no algoritmo
+   greedy de quebra de linha. **Não reabre** DEBT-1 nem
+   DEBT-52: ADR-0054 declarou gap 7 opcional; este passo
+   reduz superfície de scope-out por priorização tua, sem
+   contradizer perfil observacional graded.
```

Contagem de DEBTs abertos: **inalterada** (10) — DEBT-52
permanece encerrado. Apenas anotação ao histórico.

### 144.9 — Actualizar README dos ADRs

Em `00_nucleo/adr/README.md`:

- Tabela "Estado por ADR" — linha nova para ADR-0057.
- Distribuição de status — recálculo: `IMPLEMENTADO` 17 → 18
  (ou ajustado conforme estado final da ADR).
- Total: 56 → 57 ADRs.
- "Passos-chave da história dos ADRs" — entrada nova:

```markdown
- **Passo 144** — Lang hyphenation (gap 7 DEBT-52
  reaberto pós-fecho). ADR-0057 autoriza crate
  `<escolhida>`. Consumer integrado no algoritmo greedy de
  quebra de linha. Reduz superfície de scope-out de
  ADR-0054 sem invalidá-la.
```

### 144.10 — Verificação final + relatório

Comandos de verificação:

```bash
cargo test --workspace --lib
cargo test --workspace --tests  # se integration tests separados
crystalline-lint .
```

Relatório em `materialization/typst-passo-144-relatorio.md`
com secções:

1. Sumário executivo.
2. Inventário das crates (resultado de 144.1).
3. Inventário do ponto de integração (resultado de 144.2).
4. Decisão final (crate escolhida + localização).
5. ADR-0057 produzida.
6. Diff resumido em `Cargo.toml` e `crystalline.toml`.
7. Função `hyphenate` (assinatura + localização).
8. Modificação no algoritmo de quebra de linha.
9. Tests adicionados (números + nomes).
10. Edições L0 + hash propagado.
11. DEBT-52 actualizado.
12. README dos ADRs actualizado.
13. Limitações registadas:
    - Algoritmo permanece greedy (não Knuth-Plass).
    - Sem hyphenation contextual.
    - Sem soft-hyphen Unicode no input.
    - Shaping features (rustybuzz) continuam ausentes —
      `lang` é agora **parcialmente** consumido (hyphenation)
      mas não totalmente.
14. Verificação final.

---

## Verificação

1. ✅ Inventário das crates registado.
2. ✅ ADR-0057 criada com `IMPLEMENTADO` (ou `PROPOSTO` se
   bloqueio surgir).
3. ✅ Crate autorizada em `Cargo.toml` + `crystalline.toml`
   se aplicável.
4. ✅ `hyphenate` implementada + 4 tests unitários.
5. ✅ Quebra de linha modificada para consumir hyphenation.
6. ✅ 4 tests de integração L3 (3 cenários reais + 1
   regressão).
7. ✅ DEBT-52 secção 2 com actualização.
8. ✅ README dos ADRs com ADR-0057 + entrada P144.
9. ✅ L0 actualizado; hash propagado.
10. ✅ `cargo test --workspace`: 1095 → 1095+8 = 1103 (+8
    novos: 4 unit + 4 integração).
11. ✅ `crystalline-lint .`: zero violations.
12. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Hyphenation funcional para PT/EN no mínimo (extensível
   por escolha de feature flags da crate).
2. Output PDF observacionalmente correcto: hífenes
   inseridos em pontos válidos.
3. Sem regressão: documentos sem `lang` ou com `lang`
   improvável continuam idênticos ao pré-144.
4. ADR-0057 documentada e referenciada.
5. README dos ADRs e DEBT-52 actualizados.
6. Tests verdes; lint zero.
7. Limitações registadas (shaping ainda ausente).

---

## O que pode sair errado

- **Crate escolhida tem deps transitivas pesadas**: se
  inventário 144.1 revelar que `hyphenation` puxa 10+ deps
  transitivas e `hypher` puxa 0–1, a escolha é trivialmente
  `hypher`. Se ambas puxam pesado, registar e reconsiderar
  — `crystalline.toml` regula superfície.

- **API da crate exige `Lang` como string em vez de tipo**:
  necessário mapear `Lang` (tipo semântico ADR-0052) para
  `&str` ou enum da crate. Helper de conversão em L1.

- **Crate exige carregamento runtime de padrões**: I/O é
  problema em L1 (ADR-0029). Se acontecer, mover toda a
  hyphenation para L3, expor função pura via trait do
  contrato. ADR-0057 reflecte localização final.

- **`Lang` não tem accessor para código ISO**: adicionar
  accessor puro em L1 (ex: `Lang::iso_code() -> &str`). Sem
  lógica nova; só leitura. Justifica nota no relatório.

- **Algoritmo de quebra de linha é mais sofisticado que
  greedy**: integração pode ser mais complexa que o
  pseudocódigo de A.6.1. Se for o caso, registar o algoritmo
  real no inventário 144.2 e adaptar. Se complexidade
  ultrapassa "modificação cirúrgica", **pausar** e reverter
  para abordagem diagnóstico-primeiro (criar 144A com
  diagnóstico; 144B materializar).

- **Hyphenation produz pontos de quebra absurdos para
  algumas palavras**: as crates baseadas em padrões TeX são
  conservadoras, mas excepções existem. Documentar no
  relatório casos detectados; aceitar se forem raros (perfil
  graded).

- **Padrões PT brasileiros vs europeus diferentes**: TeX
  hyph-utf8 trata-os como locales separados (`pt-BR` vs
  `pt-PT`). `Lang` em cristalino captura `pt` simples? Se
  sim, decidir default (`pt-BR` é mais comum globalmente).
  Documentar e oferecer flag de feature da crate para
  escolha.

- **PDF emite hífen como glyph diferente do esperado**: se
  font não tem glyph para `-` (improvável mas possível com
  fonts decorativas), fallback a CIDFont assegura cobertura.
  Confirmar em testes.

- **Conflito de localização L1 vs L3 quando crate é
  borderline**: alguns crates de hyphenation usam `OnceLock`
  global para cache de padrões — incompatível com L1
  (ADR-0029 + ADR-0032). Se detectado, forçar L3.

- **`crystalline.toml` rejeita crate por critério não óbvio**:
  ADR-0018 estabelece "não viola pureza funcional". Crates
  que internamente usam unsafe ou allocação não-determinista
  podem falhar critério. Se acontecer, ADR-0057 documenta a
  excepção; se a excepção é injustificável, reconsiderar
  crate.

- **Tests de integração com hífen são frágeis a inspecção
  textual**: PDF binário pode codificar `-` de várias
  formas. Asserts devem usar parser de PDF ou marker textual
  resiliente. Padrão do 140B/141 (probe + early-return)
  aplica-se.

---

## Notas operacionais

- **`lang` muda de scope-out total para parcial**. Após
  144, hyphenation existe; shaping features (rustybuzz)
  continuam ausentes. Relatório 142 §3 fica
  desactualizado quanto à descrição do estado de `lang` —
  mas é histórico imutável (Convenção do README dos ADRs).
  O relatório 144 declara o novo estado actual.

- **DEBT-52 não reabre**. ADR-0054 já declarou gap 7
  opcional. Este passo é "extensão voluntária" — análogo a
  como uma divisão poderia opcionalmente implementar uma
  feature de scope-out sem invalidar a decisão original.

- **Sem ADR de revisão de ADR-0054**. ADR-0054 não muda;
  apenas a "superfície de scope-out residual" diminui.

- **Modelo "tudo-num-passo" registado**. Se voltar a ser
  usado no futuro, é precedente. Inventário condensado em
  144.1+144.2 cumpre espírito de ADR-0034 mas não a sua
  letra. Se a frequência aumentar, considerar ADR formal
  sobre quando "tudo-num-passo" é aceitável vs quando
  exige diagnóstico-primeiro.

- **ADR-0057 é a primeira ADR criada pós-uniformização do
  P145**. Cabeçalho deve ser canónico desde o início:
  `**Status**: \`<valor>\``, título com `⚖️`, campo
  `**Validado**:` se relevante. Relatório 145 estabeleceu
  o padrão; 144 cumpre-o desde a primeira linha.

- **Após 144**: candidatos remanescentes não-bloqueantes
  são Passo 142A (multi-font), ADR-0055bis (variant-aware),
  ADR-0054bis (regex/font dict), DEBT-53 (rustybuzz). A
  prioridade entre eles continua decisão tua.

- **Conflito potencial com 142A**: se multi-font for
  priorizado pouco depois deste, há decisão tipográfica
  sobre como mapear `lang` para fonts diferentes (ex:
  documento bilíngue PT/JA com hyphenation só em PT). Não
  endereçado neste passo; nota para 142A.
