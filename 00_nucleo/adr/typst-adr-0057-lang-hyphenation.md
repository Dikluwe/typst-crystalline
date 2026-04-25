# ⚖️ ADR-0057: Lang hyphenation em L1 via crate `hypher`

**Status**: `IMPLEMENTADO`
**Validado**: Passo 144.E — 9 testes (5 unit `hyphenation` + 4 integração `lang_hyphenation_*`); zero violations.
**Data**: 2026-04-24
**Autor**: Humano + IA
**Diagnóstico prévio**: ausente — inventário condensado em §3
deste ADR (modelo "tudo-num-passo" registado nas notas
operacionais do Passo 144).

---

## Contexto

Gap 7 do DEBT-52 (lang hyphenation) era declarado opcional por
ADR-0054 (perfil observacional graded — "sem garantia de
shaping features"). DEBT-1 fechou no Passo 142 com `lang` em
scope-out total. Passo 144 reabre voluntariamente o gap por
priorização tipográfica: textos longos em PT/EN/ES/FR
beneficiam visivelmente de hyphenation (justificação produz
"rios" sem ela).

`Lang` em L1 é tipo semântico desde Passo 131B (ADR-0052):
`[u8; 3]` + length, ISO 639-1/2/3 ASCII, lowercase, Copy.
`as_str()` devolve o código sem padding. `TextStyle.lang`
captura o valor activo desde Passo 136 (Fase A DEBT-52). Falta
**consumer**.

Esta ADR autoriza a crate `hypher` em L1 e materializa o
consumer no algoritmo greedy de quebra de linha.

## Inventário condensado (cumpre espírito de ADR-0034)

Comparação `hypher` vs `hyphenation` (TeX hyph-utf8 patterns):

| Critério | `hypher` v0.1.7 | `hyphenation` v0.8 |
|----------|-----------------|--------------------|
| Pureza | `no_std`, sem `unsafe`, **zero deps** | depende de `bincode`, `pocket-resources` |
| Padrões | embebidos em compile-time como tries binários | embebidos OU loaded em runtime |
| Tamanho do binário | ~1.1 MiB para todas as linguagens (default) | ~3-5 MiB com todas as linguagens |
| Cobertura | 30+ idiomas (PT, EN, ES, FR, DE, IT, NL, …) | 33 idiomas (TeX hyph-utf8) |
| API principal | `pub fn hyphenate(word: &str, lang: Lang) -> Syllables<'_>` | `pub fn hyphenate(...) -> Standard` |
| Mapping | `Lang::from_iso(code: [u8; 2]) -> Option<Self>` | `Language` enum FromStr |
| Maturidade | Activa (autor: Laurenz, mesmo do Typst) | Estável, mais antiga |

**Decisão**: `hypher`.

Razões pela ordem dos critérios da spec 144.1.A.2:

1. **Pureza** — `hypher` é `no_std`, sem `unsafe`, sem deps;
   `hyphenation` requer `bincode`/`pocket-resources`. ADR-0029
   + ADR-0030 favorecem o primeiro.
2. **Tamanho** — diferença de 2-4 MiB é material em embedded
   contexts (e em CI artifact size).
3. **Cobertura** — ambos cobrem PT/EN/ES/FR/DE/IT (mínimo
   exigido).
4. **API** — `Lang::from_iso([u8; 2])` mapeia directamente do
   nosso `Lang::as_str().as_bytes()[..2]`. Limpo.
5. **Sem I/O** — `hypher` embebe via `include_bytes!`. Compila
   sem nenhum acesso a filesystem. Compatível com L1 sem
   trade-offs.

## Decisão

1. **Crate**: `hypher = "0.1"` (resolve para `0.1.7` no
   `Cargo.lock`). Default features (`alloc` + `full`) — todos
   os 30+ idiomas embebidos. Pode reduzir-se via opt-in
   (`default-features = false, features = ["english", "portuguese", ...]`)
   se a vinculação completa pesar.

2. **Localização**: **L1**. Adicionada a `[l1_allowed_external]`
   em `crystalline.toml`. Justificação: padrões são pure-data
   (bytes embebidos), API é função pura sem I/O nem estado
   global (ADR-0029, ADR-0030). Sem `unsafe` (`#![forbid(unsafe_code)]`
   no `hypher`). Cumpre ADR-0018.

3. **Pipeline de consumo**:
   ```
   layout_word(word)
     ↓ word não cabe E cursor_x > margin E style.lang.is_some()
     ↓
   hyphenation::hyphenate(word, &lang) → Vec<usize>  (chars)
     ↓ (greedy: maior prefixo que cabe vence)
     ↓
   FrameItem::Text { text: "<prefix>-", … } na linha actual
     ↓ flush_line
   layout_word(&rest)  (recursão)
   ```
   Wrap em `01_core/src/rules/layout/hyphenation.rs` mapeia
   `Lang::as_str().as_bytes()` → `[u8; 2]` →
   `hypher::Lang::from_iso(...)`.

4. **Política de fallback** (silent skip — consistente com
   ADR-0055 decisão 5 para fonts):
   - **Idioma com 3 letras** (ISO 639-2/3, ex: "por", "fil"):
     `hypher` só aceita 2-letras → `Vec::new()`.
   - **Idioma não suportado** pelo `hypher` (ex: "xx", "ja"):
     `from_iso` devolve `None` → `Vec::new()`.
   - **Palavra sem pontos de quebra** (uma sílaba):
     `Vec::new()`.
   - **Documento sem `lang`** (`style.lang == None`): condição
     `if let Some(lang) = ...` falha → fallback (`flush_line`
     directo, comportamento pré-144).

5. **Posição e forma do hífen no PDF**: hífen literal `-`
   (U+002D) inserido como sufixo do prefixo emitido. Sem soft
   hyphen Unicode (U+00AD); sem hífen contextual. Encoding
   final no PDF passa pelo mesmo path de qualquer outro
   `FrameItem::Text` (Helvetica fallback ou CIDFont via
   ADR-0055).

6. **Algoritmo de inserção**: dentro do branch greedy
   pré-existente (`layout_word` em `01_core/src/rules/layout/cursor.rs`),
   quando word não cabe E cursor não está na margem:
   - Iterar break_points da maior para a menor (`iter().rev()`).
   - Primeiro prefixo (com hífen) cuja `word_width <= available`
     vence: emit + flush + recursão com sufixo.
   - Se nenhum prefixo cabe: fall-through para `flush_line`
     pré-existente (palavra inteira para linha seguinte).

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| `hypher` ✓ | Pureza, tamanho, zero deps, API limpa | Padrões TeX são heurística — falsos positivos raros possíveis |
| `hyphenation` | Maturidade, padrões idênticos | Deps transitivas (`bincode`, etc); mais peso |
| Implementação própria (TeX patterns embedded manualmente) | Controle total | XL; dezenas de horas de extracção/empacotamento dos hyph-utf8 |
| Esperar por rustybuzz (DEBT-53 candidato) | Cobre shaping + hyphenation | XL; bloqueia gap 7 indefinidamente |
| Não materializar (continuar scope-out) | Zero risco | Justificação de parágrafos com "rios" persiste |

## Consequências

### Positivas

- **Gap 7 do DEBT-52 fechado** voluntariamente. `lang` muda
  de scope-out total para **parcialmente consumido**:
  hyphenation activo; shaping features (rustybuzz) continuam
  ausentes.
- **Justificação visualmente correcta** em PT/EN/ES/FR e 25+
  outros idiomas suportados pelo TeX hyph-utf8.
- **Zero impacto em documentos sem `lang`** (regressão
  garantida pelo teste `lang_hyphenation_sem_set_lang_*`).
- **Política de fallback limpa**: idiomas exóticos ou códigos
  3-letras silent skip — sem warnings nem erros, sem
  surpresa.

### Negativas

- **Crate nova autorizada em L1**. `hypher` adiciona ~1.1 MiB
  ao binário com all-languages default. Mitigação: opt-in
  por idioma se peso for problema (decisão futura, não bloqueia
  144).
- **Padrões TeX são heurística** — falsos positivos raros
  (palavras quebradas em pontos não-ideais) são possíveis.
  Aceitável dentro do perfil observacional graded de ADR-0054.
- **Sem hyphenation contextual** (ex: `co-operate` não-quebra
  como `coop-erate`). Out-of-scope; aceite.

### Neutras

- **`lang` agora é parcialmente consumido**: relatório 142
  §3 lista `lang` como scope-out — fica desactualizado mas é
  histórico imutável (Aviso sobre vocabulário em documentos
  históricos do README dos ADRs aplica-se).
- **DEBT-52 não reabre**. ADR-0054 declarou gap 7 opcional;
  este passo reduz superfície de scope-out residual sem
  contradizer perfil graded. Anotação de actualização à
  entrada de DEBT-52 em Secção 2 do `DEBT.md`.
- **Sem ADR de revisão de ADR-0054** — perfil graded
  permanece a regra; só a "superfície scope-out" diminui.

## Referências

- **ADR-0018** (critério de autorização externa) — invocada
  para justificar `hypher` em `[l1_allowed_external]`.
- **ADR-0029** (pureza física de L1) — `hypher` cumpre (sem
  I/O, padrões compile-time embebidos).
- **ADR-0030** (performance é domínio de L1) — padrões em
  RAM (via `include_bytes!`) são domínio L1 puro.
- **ADR-0033** (paridade funcional) — output observacional:
  pontos de quebra equivalentes ao vanilla para inputs de
  teste documentados.
- **ADR-0034** (diagnóstico obrigatório) — cumprimento via
  inventário condensado em §3 (modelo "tudo-num-passo").
- **ADR-0052** (Lang tipo semântico) — input do consumer.
- **ADR-0054** (critério fecho DEBT-1, perfil observacional
  graded) — gap 7 declarado opcional; este ADR reduz
  superfície sem invalidar o perfil.
- **DEBT-52** (rastreador, encerrado no Passo 142) — gap 7
  marcado materializado por anotação à entrada.
- **DEBT-53** (rustybuzz shaping, candidato XL) — `lang`
  continua **parcialmente** scope-out; shaping features
  (ligatures, kern, bidi) ficam fora deste ADR.
