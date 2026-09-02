# P1293 — recibo de reabertura contratual B: IC semântica do fragmento em `attach`

**Papel segregado:** autor contratual/L0 (`autor_contrato_p1293`)  
**Estado:** `L0_AUTHORED_AWAITING_LINEAGE_RESEAL_AND_REPLACEMENT_GATE`  
**Instante:** `2026-09-01T23:28:06-03:00`  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
**Working tree:** compartilhada e não commitada; este papel não atribui a si alterações fora da allowlist documental abaixo.

## 1. Entradas e segregação

| Entrada | SHA-256 verificado |
|---|---|
| manifesto antes desta reabertura | `ac0ff355c24d47f340a3df9d9f20932a2b5624462c5e47d5455d674da684f39d` |
| `p1293-attach-ic-residual-measurement-receipt.md` | `4564886ff14262755fe7163cdd6f6c02836bf4bc45d9264ef82ec9c8b0b8f987` |
| `p1293-implementation-receipt-b.md` | `c067ee6fb7238dc72862208fa18deec73b2fd9ea33c903838e2554daa9c9da6e` |
| selo serial equation-frame consumido/refutado | `c7d280e4ea8c9f5dd8342a5bfc92f9b782cc90aefff35dd3b2ba8df4350375f1` |
| bloco canônico do predecessor | `ca7df58c739158f6fc327dbf93be87dcd38d884186766195bcdaf5f4675e36ca` |
| consumer candidato `attach.rs`, somente hash | `1a660cf5b86860414d58654e945c35d1ce6b243585baa8ef3b0aa848b078a526` |

Foram lidos os ADRs 0107/0108/0127/0129, o L0 attach vigente e os recibos públicos pinados. Não foram lidos nem editados o patch candidato, código produtivo, testes, contrato protegido, oracle, RED/discriminação, ataques ou veredito. O arquivo de selo foi verificado somente por hash e permaneceu byte-idêntico.

## 2. Medição antes da decisão

O recibo independente mediu o vetor completo em `20,7614 × 9,7735pt` no vanilla e `20,5854 × 9,7735pt` no candidato: residual horizontal `−0,1760pt`. Base, `tl`, `bl` e `tr` coincidem; somente `br` desloca de `12,8458pt` para `12,6698pt`.

Refutadores de presença:

- sem `br`: `19,4755pt` bilateral;
- somente `tr`: `12,4377pt` bilateral;
- somente `br`: vanilla `13,7236pt`, candidato `13,5476pt`, delta `−0,1760pt`;
- `tr+br`: mesmo delta; se `tr` domina o máximo, o residual fica mascarado e reaparece quando `br` volta a dominar.

Matriz da base `Content::Text`, com `br` dominante:

| Base | IC da fonte | Vanilla | Candidato | Delta |
|---|---:|---:|---:|---:|
| `[x]` | `16du = 0,176pt` | `13,7236pt` | `13,5476pt` | `−0,1760pt` |
| `[f]` | `79du = 0,869pt` | `11,2816pt` | `10,4126pt` | `−0,8690pt` |
| `[R]` | `24du = 0,264pt` | `16,0116pt` | `15,7476pt` | `−0,2640pt` |
| `[A]` | `0` | `16,1656pt` | `16,1656pt` | `0` |
| `[1]` | `0` | `13,4156pt` | `13,4156pt` | `0` |

MathIdent/GlyphFragment `x/f/R` permanecem exatos bilateralmente (`14,9314`, `14,0294`, `16,9884pt`). A mudança de inline para block/display não neutraliza o TextItem, enquanto o display com átomos math permanece exato. Isso refuta compensação por modo, caractere, texto ou fonte e refuta apagar IC globalmente.

A fonte ratificada `math/scripts.rs:220-240` subtrai `base.italics_correction()` tanto de `br_x` quanto de `br_post`. `math/fragment/mod.rs:145-150,239-255` mostra que o valor pertence ao fragmento: GlyphFragment fornece IC tipográfica; FrameFragment/TextItem inicia IC semântica zero. A fórmula não diverge; o carrier causal fornecido a ela diverge.

Inferência: `TextStyle.math_text_item`, proveniência já confirmada, distingue internamente o FrameFragment textual sem novo payload. Refutadores: TextItem não satisfazer `delta=−IC` quando `br` domina; `A/1` divergirem; MathIdent deixar de coincidir; vanilla aplicar IC tipográfica a TextItem; ou sem-`br`/somente-`tr` manterem delta. Qualquer refutador bloqueia.

## 3. Decisão L0 e owner 1:1

O L0 mantém a fórmula vanilla:

```text
br_ic = semantic_italics_correction(base_fragment)
br_kern = math_kern(BR) - br_ic
br_post = present_markup(br)
  ? SpaceAfterScript + width(br) + br_kern
  : 0
br_x = pre_width + width(base) + br_kern
```

`FrameItem::Text` com `style.math_text_item=true` representa TextItem/FrameFragment e fornece IC semântica zero, independentemente da IC do último caractere. GlyphFragment, MathIdent, NumberItem e rotas não-TextItem conservam a IC métrica normal. É proibida heurística por texto, char, fonte, largura, tinta ou último item, bem como campo/variant/signature/trait/constante/fase novos.

`tr`, markup vazio, None/`Content::Empty`, equation Group, ssty, K/W/RR, displays, máximos, kerns, shifts, cramped, limits, alinhamento e demais fórmulas permanecem inalterados.

Ownership permanece exclusivamente 1:1:

```text
00_nucleo/prompts/compiler/math/layout/attach.md
  → 01_core/src/compiler/math/layout/attach.rs
```

Nenhum owner 1:N ou Núcleo novo foi criado. Equation, shaper, font metrics, entities e todos os outros consumers ficam fora da autorização.

## 4. Hashes L0 e consumer intocado

| Artefato | SHA-256 antes | SHA-256 depois |
|---|---|---|
| `00_nucleo/prompts/compiler/math/layout/attach.md` | `fc189ea4f52436ca9c1adf52ba6767e3a2ff241211fb6edf4de9e4632cae6ff9` | `45829d6aeec2f309984c02d9e1c028d275f65d764543fca223802ed3ab26608d` |
| `01_core/src/compiler/math/layout/attach.rs` | `1a660cf5b86860414d58654e945c35d1ce6b243585baa8ef3b0aa848b078a526` | `1a660cf5b86860414d58654e945c35d1ce6b243585baa8ef3b0aa848b078a526` |

O header do consumer não foi alterado; `--fix-hashes` não foi executado.

## 5. Classificação

- **ADR-0107:** posição de `br` e extensão da fórmula são geometria de linguagem; fragmento/carrier/métrica são mecânica.
- **ADR-0108:** decisão posterior às matrizes `x/f/R/A/1`, MathIdent, sem-`br` e somente-`tr`, com inferência e refutadores explícitos.
- **ADR-0127:** fluxo contínuo interno. Usa o campo público já confirmado, sem ampliar API, default, fase ou compatibilidade; não requer novo gate humano.
- **ADR-0129:** um L0 e um consumer produtivo, relação 1:1.

Contrato/oracle continuam protegidos e unchanged. `Unknown` permanece bloqueante; o lote B não está aprovado.

## 6. Invalidação e gates pré-resselo

O selo serial `c7d280e4ea8c9f5dd8342a5bfc92f9b782cc90aefff35dd3b2ba8df4350375f1`, bloco `ca7df58c…`, foi consumido pelo candidato refutado e fica histórico sem autorização de novas escritas. Seu arquivo não foi editado.

```text
crystalline-lint --checks V15,V26 --fail-on warning .
✓ No violations found

crystalline-lint --fix-hashes --dry-run .
Would fix ./01_core/src/compiler/math/layout/attach.rs ...
old=7be814c8 hash-a=11e7a221 hash-b=139b9bac
```

V15/V26 estão verdes e o dry-run aponta **exatamente `attach.rs`**. Nenhuma escrita de lineage ocorreu.

## 7. Próxima transição

O coordenador pode ressellar mecanicamente somente o header de `attach.rs`, revalidar V5/V15/V26 e obter gate discriminatório/seal substituto. Allowlist proposta para o futuro seal:

- `01_core/src/compiler/math/layout/attach.rs`;
- `00_nucleo/diagnosticos/p1293-implementation-receipt-b.md`.

Até o seal substituto, todo produto permanece congelado; lotes C/D continuam proibidos.
