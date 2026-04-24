# ⚖️ ADR-0052: Lang como tipo semântico em L1

**Status**: `IMPLEMENTADO`
**Data**: 2026-04-24
**Materializado em**: Passo 131B (2026-04-24)
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`](../diagnosticos/diagnostico-lang-passo-131a.md)

---

## Contexto

O Passo 130 capturou `#set text(lang: "pt")` como
`Option<EcoString>` raw em `StyleDelta`, sem validação. Revisão
subsequente classificou o comportamento como **divergência
semântica activa** face ao vanilla: input
`#set text(lang: "xx-invalid-nonsense")` passa silencioso no
cristalino, erra hard no vanilla. Viola ADR-0033 (paridade
funcional).

Decisão tomada: materializar `Lang` como tipo semântico em L1
com validação e erro-hard em inputs inválidos, obtendo paridade
total com vanilla.

ADR-0034 (diagnóstico obrigatório para materialização de tipos
vanilla) obriga a produzir documento estruturado antes de
escrever código. O diagnóstico vive em
`00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md` e cobre
os 7 itens mínimos.

## Decisão

1. **Materializar `Lang` em L1** como réplica exacta do vanilla:
   ```rust
   #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
   pub struct Lang([u8; 3], u8);
   ```
   - Newtype compacto: 3 bytes ASCII (padded com `b' '` se
     2-letter) + length discriminator (2 ou 3).
   - `Copy` — pass-by-value livre de alocação.

2. **Parser BCP 47 via `impl FromStr`** — idêntico ao vanilla:
   - Aceita 2 ou 3 letras ASCII.
   - Normaliza para lowercase.
   - Erro literal: `"expected two or three letter language code (ISO 639-1/2/3)"`.

3. **Erro hard em valor inválido** (paridade vanilla).
   Arm `"lang"` em `eval_set_text` introduz primeiro caso de
   `SourceResult::Err` dentro do loop de argumentos (até hoje,
   só warnings).

4. **`StyleDelta.lang: Option<Lang>`** substitui
   `Option<EcoString>`. Campo continua a ser "absence =
   herdado" (semântica StyleChain preservada).

5. **Localização**: `01_core/src/entities/lang.rs` — ficheiro
   novo dedicado ao domínio *identificadores de língua*. ADR-0037
   (coesão por domínio). Futuras adições (`Region`,
   `WritingScript`) vivem no mesmo módulo.

6. **Constantes mínimas**: apenas `Lang::ENGLISH`. Vanilla tem
   ~260 constantes; cristalino materializa on-demand quando
   consumer (shaping/hyphenation/translations) precisar.

7. **`Dir` e hint "put region in region parameter"** **deferidos**
   — exigem tipos adjacentes (`Dir`, `Region`) não materializados.
   Mensagem de erro fica simples neste passo; hint pode chegar
   quando `Region` chegar.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **Manter `Option<EcoString>` raw** | XS, zero refactor | Viola ADR-0033 activamente; divergência semântica observável |
| **Validar BCP 47 inline sem tipo** | S, parser directo no arm | Dispersa lógica de validação; dívida estrutural; sem reuso em call-sites futuros |
| **Materializar `Lang` fiel ao vanilla** ✓ | Paridade total; coesão por domínio; base para `Region` futuro | S, exige migração de `StyleDelta.lang` + testes |
| **Crate externa `unic-langid`** | Correctness by construction; RFC 5646 full | Não autorizada em `[l1_allowed_external]`; over-engineering para subset ISO 639 que vanilla usa |
| **Enum com 260 variantes** | Exhaustive match | 260 linhas de boilerplate; `Lang::from_str` teria de match 260 strings |

**Escolha**: materializar como réplica fiel do vanilla
(struct compacto `([u8; 3], u8)` com `FromStr`). Padrão
vanilla é sensato para este domínio — cristalino espelha.

## Consequências

### Positivas

- **ADR-0033 satisfeito**: inputs inválidos erram como no
  vanilla.
- **Base para consumer futuro**: shaping engine pode usar
  `Lang` para escolher fonte/hyphenation por língua.
- **Base para extensões**: `Region`, `WritingScript`, `Locale`
  seguem pattern no mesmo ficheiro.
- **Precedente para outros tipos com validação**: `font-size`
  (bounds check), `stroke.width` (positive), etc. podem
  seguir estrutura `struct + FromStr + erro hard no arm`.
- **DEBT-1 subset `lang` fica "paridade vanilla"** em vez
  de "captura temporal" — qualidade acima dos 126-130.

### Negativas

- **Custo migração**: 131B ≈ S (1-2h).
- **Mais 1 tipo em L1 a manter**: ~50-80 linhas.
- **Introduz Err no loop de args de `eval_set_rule`**: primeiro
  arm a divergir do padrão "warnings só". Requer atenção na
  propagação do `SourceResult::Err`.
- **Teste existente `eval_set_text_lang_bcp47_composto_passo_130`
  inverte semântica**: `"en-GB"` deixa de ser silent e vira
  erro. Teste actualizado no 131B como canary de paridade.

### Neutras

- **`StyleDelta` permanece com mesmo número de campos**, só
  muda tipo de 1 campo.
- **ADR-0038 ganha quarta nota** (recomendação): pequena,
  apontando que `lang` deixa de ser subset DEBT-1 XS porque
  tem validação própria (ADR-0052 absorve).
- **Constantes on-demand**: não há degradação; simplesmente
  ninguém precisa delas ainda.

## Plano de materialização (para 131B)

1. Criar `01_core/src/entities/lang.rs`:
   - `struct Lang([u8; 3], u8)` + derives.
   - `impl Lang`: `ENGLISH` const + `as_str()`.
   - `impl FromStr for Lang`: parser com mensagem vanilla.
   - 8-10 unit tests (válidos, inválidos, casos edge).
2. Actualizar `01_core/src/entities/mod.rs`: `pub mod lang;`.
3. Alterar `StyleDelta.lang`:
   - `Option<EcoString>` → `Option<Lang>`.
   - Remover `use ecow::EcoString;` se fica não usado.
   - Adicionar `use crate::entities::lang::Lang;`.
4. Alterar arm `"lang"` em `rules/eval/rules.rs`:
   - Parse via `Lang::from_str`.
   - `Err` propaga como `SourceResult::Err` com span do arg.
5. Adaptar 3 testes L1 do Passo 130:
   - `eval_set_text_lang_passo_130`: continua a passar (input
     válido).
   - `eval_set_text_lang_bcp47_composto_passo_130`: **inverter
     assertion** — `"en-GB"` agora erra hard.
   - `eval_set_text_font_canary_passo_130`: inalterado.
6. Adicionar teste novo:
   - `eval_set_text_lang_invalido_emite_erro_hard_passo_131b`
     com `"xx-nonsense"` → `Err`.
7. ADR-0038 quarta nota (ver diagnóstico item ADR-0038).
8. Transição ADR-0052 → `IMPLEMENTADO` ao encerrar 131B.

## Referências

- **ADR-0033** (paridade funcional) — violação a resolver.
- **ADR-0034** (diagnóstico obrigatório) — cumprido em 131A.3.
- **ADR-0036** (atomização progressiva) — dependências
  declaradas; `Lang` isolado em módulo próprio.
- **ADR-0037** (coesão por domínio) — `lang.rs` dedicado.
- **ADR-0038** (Style/StyleChain L1) — nota futura sobre
  separação de `lang` do pattern XS.
- **Passo 130** relatório — captura original como `EcoString`
  + identificação da divergência.
- **Vanilla**: `lab/typst-original/crates/typst-library/src/text/lang.rs:154-516`.

---

## Estado final (Passo 131B encerrado)

- **Ficheiro materializado**: `01_core/src/entities/lang.rs` —
  ~125 linhas (54 de código + 71 de tests).
- **11 unit tests** todos verdes.
- **Erro hard confirmado** em 2 novos integration tests:
  `eval_set_text_lang_composto_emite_erro_passo_131b`
  (`"en-GB"` → Err) e
  `eval_set_text_lang_invalido_emite_erro_hard_passo_131b`
  (`"xxxx"` → Err).
- **Teste canary DEBT-50** renomeado para
  `eval_set_text_font_canary_passo_131b` — sexta iteração
  sem regressão.
- **L1 tests**: 826 → 838 (+12).
- **Workspace total**: 1057 → 1069.
- **ADR-0038 quarta nota** adicionada explicando que `lang`
  não segue pattern DEBT-1 XS.
- **`StyleDelta.lang`** agora `Option<Lang>` — tamanho em
  bytes reduzido vs `Option<EcoString>` (ADR-0030 friendly).
- **Zero crates novas**.
- **Precedente estabelecido**: primeiro `return Err` em loop
  de argumentos de `eval_set_rule`. Disponível para reuso
  em futuras propriedades com paridade vanilla.
