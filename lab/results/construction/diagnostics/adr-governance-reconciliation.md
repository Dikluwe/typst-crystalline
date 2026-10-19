# ADR governance and ADR-0130 reconciliation

## Checkpoint

HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; uncommitted working tree.
Pre-edit inspection: `2026-09-26T20:29:27.383786-03:00`.
Existing edits, deleted/archive ADRs and unrelated implementation work were
preserved. No prompt, Rust file, archive original or ADR filename was changed.

## Decisions

- Canonical English primary status values are translations, not promotions.
- Prose uses full repository-relative `00_nucleo/prompts/...` identity,
  consistently in README, template, ORDER and ADR-0130.
- Rust `@prompt` repository-relative syntax remains unchanged.
- Step 1060 inventory/history was moved verbatim to the ADR-specific diagnostic;
  historical count/list discrepancies are explicitly non-normative.
- Remaining legacy prose is not declared translated or globally remediated.

## Pre-edit primary metadata and mappings (display whitespace normalized)

ADR-0130: `in force — decision originally approved in Step 1062` → `ACCEPTED`;
its approval remains explicit in the ADR and historical diagnostic.

### `00_nucleo/adr/typst-adr-0001-estrategia-migracao.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0005-packagespec-world.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0006-typst-timing.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0008-defer-inline.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0009-math-class.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0010-unicode-ident.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0011-unicode-math-class.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0012-unicode-script.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0013-unicode-segmentation.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0014-unscanny.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0015-ecow.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0016-lazyhash.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0017-adiamento-eval-typst-library.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0018-rustc-hash.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0020-fontdb.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0021-datetime.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0022-fontbook.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0023-indexmap.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0024-ecow-value.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0025-int-eq-float.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0026-R1-content-arc.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0026-content-divergencia.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0031-early-hashing-source.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0039-frameitem-style.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0040-activacao-set-rule.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0041-activacao-show-rule.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0042-sink-materializado.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0043-canal-sink-saida.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0044-engine-agregador.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR COM ADENDO P1139`
```

### `00_nucleo/adr/typst-adr-0046-cli-minima.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0047-cli-clap.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0049-cli-em-l2.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0050-formatter-em-l2.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0051-flags-funcionais.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0053-font-tipo-composto.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0057-lang-hyphenation.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0058-value-type-simplificado.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0059-args-tipo-separado.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO` (promovido em P418, 2026-06-23)
```

### `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md` → `PROPOSED`

```text
**Status**: `PROPOSTO`
```

### `00_nucleo/adr/typst-adr-0068-location-aware-layouter.md` → `ACCEPTED`

```text
**Status**: `ACEITE`
```

### `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md` → `ACCEPTED`

```text
**Status**: `ACEITE`
```

### `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md` → `ACCEPTED`

```text
**Estado**: **ACEITE** (P190I — M6 fechado).
```

### `00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md` → `ACCEPTED`

```text
**Estado**: **ACEITE** (P191C — validação empírica completa após 2 helpers migrados + walk gate migrado).
```

### `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md` → `ACCEPTED`

```text
**Estado**: ACEITE
```

### `00_nucleo/adr/typst-adr-0073-comemo-introspector.md` → `ACCEPTED`

```text
**Status**: **ACEITE (completo retroactivo + paridade trait
estendida M9c, P212 2026-05-12)**.
```

### `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md` → `ACCEPTED`

```text
**Status**: **ACEITE** (final, P205E 2026-05-07).
```

### `00_nucleo/adr/typst-adr-0075-vanilla-integration.md` → `ACCEPTED`

```text
**Status**: **ACEITE** (final, P206E 2026-05-08).
```

### `00_nucleo/adr/typst-adr-0076-introspector-completion.md` → `ACCEPTED`

```text
**Status**: **ACEITE (completo retroactivo, P212 2026-05-12 —
com excepções documentadas em cond #2 e cond #4 reinterpretadas
per `P207A.div-1` reshape)**.
```

### `00_nucleo/adr/typst-adr-0077-regex-l1.md` → `ACCEPTED`

```text
**Status**: **ACEITE** (transição PROPOSTO → ACEITE em P209E
2026-05-12 após verificação dos 8 critérios do §Plano de
validação).
```

### `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md` → `IMPLEMENTED`

```text
**Status**: **IMPLEMENTADO** (P215 PROPOSTO 2026-05-12 → P221
IMPLEMENTADO 2026-05-12 — sub-fases (a)+(b) materializadas;
DEBT-56 ENCERRADO simultaneamente).
```

### `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md` → `ACCEPTED`

```text
**Status**: **`EM VIGOR`** (PROPOSTO P249 → **EM VIGOR P254** —
paridade pattern P229 ADR-0080 promoção administrativa XS
dedicada pós-N=3 citantes documentados literal).
```

### `00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md` → `IMPLEMENTED`

```text
**Status**: **`IMPLEMENTADO`** (PROPOSTO P257.B → **IMPLEMENTADO
P257.D** — materialização cumpre 8 variantes + scope-outs
documentados; paridade observable preservada).
```

### `00_nucleo/adr/typst-adr-0084-auditoria-condicional.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md` → `IMPLEMENTED`

```text
**Status**: **`IMPLEMENTADO`** (PROPOSTO P270.B → **IMPLEMENTADO
P270.D** — materialização cumpre L1+stdlib 3 variants × 8 spaces;
defaults Oklab preservam P262/P264/P267 bit-exact)
```

### `00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md` → `IMPLEMENTED`

```text
**Status**: **`IMPLEMENTADO`** (PROPOSTO P270.3.B → **IMPLEMENTADO
P270.3.D** — materialização infra-estrutura Coons RGB L3 +
4 anotações cumulativas paralelas ADR-0089/0090/0091/0054;
flag opt-in interno default OFF preserva 2545 baseline bit-exact;
P270.4 activa para CMYK)
```

### `00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR` (criada directamente; paridade pattern P260
ADR-0084/0085 e P268.1 ADR-0090)
```

### `00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR` (criada directamente; paridade pattern P260
ADR-0084/0085 e P268.1 ADR-0090)
```

### `00_nucleo/adr/typst-adr-0095-dedup-arc-as-ptr-resources.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR` (criada directamente; paridade pattern P271
ADR-0093/0094 — meta-ADR formalizando sub-padrão empírico N≥3
cumulativo)
```

### `00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md` → `IMPLEMENTED`

```text
**Status**: IMPLEMENTADO (desde P281; formalizado em P288)
```

### `00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md` → `IMPLEMENTED`

```text
**Status**: IMPLEMENTADO (desde P285; formalizado em P289)
```

### `00_nucleo/adr/typst-adr-0100-coesao-por-dominio-l3.md` → `IMPLEMENTED`

```text
**Status**: `IMPLEMENTADO`
```

### `00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0102-math-style-mechanism.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0103-math-style-composition.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0104-atomicidade-para-agentes.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0105-modelo-elemento-d-agora-f-destino.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0106-fronteira-de-extensao-e1.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md` → `ACCEPTED`

```text
**Status**: `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0109-atomizacao.md` → `ACCEPTED`

```text
**Estado:** aceite (decisão do dono).
```

### `00_nucleo/adr/typst-adr-0110-marcador-vanilla.md` → `PROPOSED`

```text
**Estado:** `PROPOSTO` (Passo 385; diagnóstico-primeiro — não materializa código).
```

### `00_nucleo/adr/typst-adr-0111-crates-parsing-loading.md` → `IMPLEMENTED`

```text
**Estado:** `IMPLEMENTADO` (Passo 387 — módulo `loading` materializado; `cargo build --workspace` + `crystalline-lint .` verdes; 13 testes de decode L1 passam).
```

### `00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md` → `IMPLEMENTED`

```text
**Estado:** `IMPLEMENTADO` (Passo 399 — `Value::Decimal` materializado; `cargo test --workspace` + `crystalline-lint .` verdes).
```

### `00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md` → `IMPLEMENTED`

```text
**Status**: IMPLEMENTADO
```

### `00_nucleo/adr/typst-adr-0114-sonda-antes-da-spec.md` → `IMPLEMENTED`

```text
**Status**: IMPLEMENTADO
```

### `00_nucleo/adr/typst-adr-0115-infra-benchmark-scanner.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR` (Passo 441 — infra de benchmark criada; `cargo bench` funcional e reprodutível).
```

### `00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR` (Passo 443 — decisão final sobre DEBT-42).
```

### `00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR` (Passo 453 — mecanismo proposto e adoptado como extensão operacional da ADR-0114; **Cláusula 4 adicionada no Passo 455**).
```

### `00_nucleo/adr/typst-adr-0118-runtime-state-context.md` → `PROPOSED`

```text
**Estado:** `PROPOSTO` (Passo 506; aguarda confirmação do dono para promoção a `EM VIGOR`).
```

### `00_nucleo/adr/typst-adr-0119-disciplina-verificacao.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md` → `ACCEPTED`

```text
**Status**: **ACEITE** (P482 2026-06-27; materializado em P482).
```

### `00_nucleo/adr/typst-adr-0121-proveniencia-medicao.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0122-paridade-defeitos-testes.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0123-geometria-tipografica.md` → `PROPOSED`

```text
**Estado:** `PROPOSTO`
```

### `00_nucleo/adr/typst-adr-0124-checklist-sublayouts.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0125-implementacao-vs-linguagem.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` → `ACCEPTED`

```text
**Estado:** `EM VIGOR`
```

### `00_nucleo/adr/typst-adr-0128-html-target-semantico.md` → `ACCEPTED`

```text
**Estado:** EM VIGOR
```

### `00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md` → `ACCEPTED`

```text
**Estado:** EM VIGOR — APROVADA NO GATE ADR-0127
```

## Files edited in this reconciliation

- `00_nucleo/adr/ORDER.md`; pre-edit SHA-256: `6ddaf5ba82354f788d9a249b15cda6c55d93709993ac3f8e3ab2dee8437fd2f7`.
- `00_nucleo/adr/README.md`; pre-edit SHA-256: `615a746e36c449f2930ccdf0835c9becbb151288ff1b4c9e3de587389342ca6e`.
- `00_nucleo/adr/template-adr.md`; pre-edit SHA-256: `0bd9aead4e8e0f2322f2fcc5253367df8279491191d93def1e580cfbd1a4003c`.
- `00_nucleo/adr/typst-adr-0001-estrategia-migracao.md`; pre-edit SHA-256: `e442d9f147a825f0697ddc775c5d3da5a2361f9770b9c3799dd46d7a3140f84d`.
- `00_nucleo/adr/typst-adr-0005-packagespec-world.md`; pre-edit SHA-256: `2b8aefed1371d5d9ca00a5b73b85692158de617cf2ff21e9f94d0811fcc57fcb`.
- `00_nucleo/adr/typst-adr-0006-typst-timing.md`; pre-edit SHA-256: `52a550dc2ae9a6b1df5907447f420a055fd041f367e34fc9117666269b7c0e9d`.
- `00_nucleo/adr/typst-adr-0008-defer-inline.md`; pre-edit SHA-256: `8293fa84f846be0678f4be8c20fe34983002da47f150282f71b3219ee170efb2`.
- `00_nucleo/adr/typst-adr-0009-math-class.md`; pre-edit SHA-256: `55d912073ef5952eb8e5a60a9c42309947e754848e983d794795de08b1e64d93`.
- `00_nucleo/adr/typst-adr-0010-unicode-ident.md`; pre-edit SHA-256: `117557b2fc069ffc450009ac1d9af92c6e4845ff087c50e2127602023f4f59b1`.
- `00_nucleo/adr/typst-adr-0011-unicode-math-class.md`; pre-edit SHA-256: `2435050d2384fb3930e69a72ce090b76d6630012fd839f5effe8f59b0b2fdac7`.
- `00_nucleo/adr/typst-adr-0012-unicode-script.md`; pre-edit SHA-256: `ec49a113e4471b91d694c9855251cfa0d4fd35661070a2b6823b99f67d383a28`.
- `00_nucleo/adr/typst-adr-0013-unicode-segmentation.md`; pre-edit SHA-256: `cf51a15ce1f8caf1135a08e4e74c7b7dd5c8b68410a3153c5dbf2d83505c956a`.
- `00_nucleo/adr/typst-adr-0014-unscanny.md`; pre-edit SHA-256: `8195e519119d9ba1274bcc42efc3280d8a008c9329cc2378a4aa3e7dbdfb7fb9`.
- `00_nucleo/adr/typst-adr-0015-ecow.md`; pre-edit SHA-256: `e2108dab9ae71f6b6185da33aafc0853a27157b4dba567c9ea19074dd6e79b93`.
- `00_nucleo/adr/typst-adr-0016-lazyhash.md`; pre-edit SHA-256: `b822d5bd51d58e9f4d0d0a9f80a946f6e8f187260daa8d99b1cfc0ff1ae569f4`.
- `00_nucleo/adr/typst-adr-0017-adiamento-eval-typst-library.md`; pre-edit SHA-256: `8f086759b02167e15e2cf6549626ab0bf1922df078bd4544eff2b60fee459880`.
- `00_nucleo/adr/typst-adr-0018-rustc-hash.md`; pre-edit SHA-256: `029136368d7f8f94baec726be621ce94b0d41e0bd4e9733f317564dcfcbc9d7d`.
- `00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md`; pre-edit SHA-256: `6aedc3415f896fd952744ffcaf1443f36b6e6baf64e93c2f5ed82d4b5b45e49e`.
- `00_nucleo/adr/typst-adr-0020-fontdb.md`; pre-edit SHA-256: `337d1263550eee6101965c28cee5e70a76881263a160623af7ef425f5f3c0185`.
- `00_nucleo/adr/typst-adr-0021-datetime.md`; pre-edit SHA-256: `04963c6496794dfe2d90a986202bfa9b7f67fbb5b73589b3aeb8cff28b3c2da9`.
- `00_nucleo/adr/typst-adr-0022-fontbook.md`; pre-edit SHA-256: `ca2456e1a5be6b5934b6a64fc60ceb8900f6d56ec91f9776c52f93d38806e785`.
- `00_nucleo/adr/typst-adr-0023-indexmap.md`; pre-edit SHA-256: `5ee2703c68473d91b30b4a2cd4fa7c1753a3fa84df9a257292cc19e98d0f0693`.
- `00_nucleo/adr/typst-adr-0024-ecow-value.md`; pre-edit SHA-256: `2f944275330203e05e04357fee73ae044fa9a9696c2328b76a75e03394d6062c`.
- `00_nucleo/adr/typst-adr-0025-int-eq-float.md`; pre-edit SHA-256: `17296cec1a27e7ce22880791f21a6755e8fa2a727e611aafa539e1426ab3b4b3`.
- `00_nucleo/adr/typst-adr-0026-R1-content-arc.md`; pre-edit SHA-256: `57cf2f4dcf498bdd3f071674109bd8b4a0d5f43f3e6f32f5e6b3c2235fc0af11`.
- `00_nucleo/adr/typst-adr-0026-content-divergencia.md`; pre-edit SHA-256: `a8794521ef773bac9c5db25eeb6e7c7249646fb1af73a97d4eb35ae7341e875f`.
- `00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md`; pre-edit SHA-256: `a639a40f456bb4e932d606c913a8e621b2b3b9ce468a208fa4a44c5076e4ce10`.
- `00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md`; pre-edit SHA-256: `8f40a2b3532c0d14197fd87b1b6f832b3275c934d7ba7f71634b639e1a858552`.
- `00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md`; pre-edit SHA-256: `d3843cdae7d48938670034a742cc3abaed65e7dc7e5741d7b327e1d230a9368d`.
- `00_nucleo/adr/typst-adr-0031-early-hashing-source.md`; pre-edit SHA-256: `91d1f4848a146f9f41a022767bb26911f8148280ce1708eaf13627065a237282`.
- `00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md`; pre-edit SHA-256: `332616313bddf20631fd9d372bc0a23120733d43769b1b6b2d828d6787596302`.
- `00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md`; pre-edit SHA-256: `8962c57feda24a7c6ad1ba7d52edaab9d59ecf54e95e50aed4a161fdfdbad02c`.
- `00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md`; pre-edit SHA-256: `7edfc6581fe3aec158548a012e884cf1dec58f46c69407f17018c5dfd25a0501`.
- `00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md`; pre-edit SHA-256: `d9e3337e7fa785c2967ef0a2a21215de8714e1c37b2fb90d27ad75608c593443`.
- `00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md`; pre-edit SHA-256: `3107007f00f35c1f225ce07a04c70a5de67a3cabccf77b2ba3cbf355c5f21d4e`.
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md`; pre-edit SHA-256: `5147b64e1d07e52cfc1594fcee30c1cf14129ceeebbbf78dbec8d0f8053426fc`.
- `00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md`; pre-edit SHA-256: `febf4b3e8d5591c19dd59014ad5ae829982c7d9acd602799341700bc6b235631`.
- `00_nucleo/adr/typst-adr-0039-frameitem-style.md`; pre-edit SHA-256: `4e463f04f3720e0d0ec4975d38ae69ad66ca5c0207422dc01484676096921a23`.
- `00_nucleo/adr/typst-adr-0040-activacao-set-rule.md`; pre-edit SHA-256: `7863fe0ceadb05ece7344e9ad7a67b0bc9339e0edf76d61a20f800ab06bdc598`.
- `00_nucleo/adr/typst-adr-0041-activacao-show-rule.md`; pre-edit SHA-256: `ec014948787633515816aceaf3d6ca3761700f99b6a7524600cfe72795e5e644`.
- `00_nucleo/adr/typst-adr-0042-sink-materializado.md`; pre-edit SHA-256: `a83a00490f53dd397cb97270e21ecb0c7bd1df67369526d59a5612373576817b`.
- `00_nucleo/adr/typst-adr-0043-canal-sink-saida.md`; pre-edit SHA-256: `b5897a18b949a878f0105c04619340bc39d0a64b962ac99c15f88417d52c9f91`.
- `00_nucleo/adr/typst-adr-0044-engine-agregador.md`; pre-edit SHA-256: `640c426ccb2b1ea76464e841af71a6e349e13f87276cdb7db6609c1ff8e0baa1`.
- `00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md`; pre-edit SHA-256: `9b427b2c987ff79dad5ed812bdc85e196c4e2bb43d358d849ce7d5d375443013`.
- `00_nucleo/adr/typst-adr-0046-cli-minima.md`; pre-edit SHA-256: `3836c76e6c124382b1a81b2a4311cd34a82bb204271ee51ca35bbb9bb2fdf96d`.
- `00_nucleo/adr/typst-adr-0047-cli-clap.md`; pre-edit SHA-256: `b9913946f4ee0398b2ca6c8b5da4d50a7b72f9d8e4ed611070a56a2756b966ff`.
- `00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md`; pre-edit SHA-256: `f810e37699060439591440648c8ba508f61f68f727e1d73c3a0894aaae48b8d4`.
- `00_nucleo/adr/typst-adr-0049-cli-em-l2.md`; pre-edit SHA-256: `d9ac79862f90affdbd40e796def20d88e43b453c0a32cea6495022d4f812d5e0`.
- `00_nucleo/adr/typst-adr-0050-formatter-em-l2.md`; pre-edit SHA-256: `c6e7ed084f12ff59a147ade06b9f607ce77a87057da7eab9e7c02ee64c59650d`.
- `00_nucleo/adr/typst-adr-0051-flags-funcionais.md`; pre-edit SHA-256: `1ff0266659fdbe874682bbc67dccbce9e9eb22df5298bc7abc799052921318b5`.
- `00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md`; pre-edit SHA-256: `c576f7898150751f0c5a3813a1ecacbec846fc5450a8f8af49b6c771798d95a2`.
- `00_nucleo/adr/typst-adr-0053-font-tipo-composto.md`; pre-edit SHA-256: `6db8bd62abb5db06225df828e556a5b4d0877172b26d32f6a8f993c83f26b450`.
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md`; pre-edit SHA-256: `1632feb2d8985d3318931c04ce259b31f4b838cbfaa3ac4b8ae6ed74fc125e45`.
- `00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md`; pre-edit SHA-256: `1b2dec972335c97b04f8481686624fd6ca0c097e859c1c3c87f3f1535536e8fa`.
- `00_nucleo/adr/typst-adr-0057-lang-hyphenation.md`; pre-edit SHA-256: `7ee98c0c243fd2ddfc090ada4cf3d50c5aebc6fe1abc4e42d2d6b5a72bfd7646`.
- `00_nucleo/adr/typst-adr-0058-value-type-simplificado.md`; pre-edit SHA-256: `4e1d457825e0942c90c20b2b09f8ff7983e7a7dc6db261fb5673fb65b39566f2`.
- `00_nucleo/adr/typst-adr-0059-args-tipo-separado.md`; pre-edit SHA-256: `4755c018502cde2437ab55a67ab8a82a79f135d525737af695f1a48a111ff7e1`.
- `00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md`; pre-edit SHA-256: `4ba9bd472680e328c1ea796a1b5d9eb550b8f3be4dfe4bc337215b8223e29261`.
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`; pre-edit SHA-256: `da8946aa9145c48cd9000380de83767feb1de56e455d9807feeaa0c533a37397`.
- `00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md`; pre-edit SHA-256: `e8b7ef8b564524938be90cee15a61b6920eb780406d863e4360cb383142e34fd`.
- `00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md`; pre-edit SHA-256: `c6af3a976aa2ae7495ec7265cdff577cb36d75462e1220418200e9a4fe620918`.
- `00_nucleo/adr/typst-adr-0068-location-aware-layouter.md`; pre-edit SHA-256: `e5df7708d0a333879e6cec0130794cae6811e387ab9bd56f56b690104e80c0f1`.
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`; pre-edit SHA-256: `1cbd1cb2ded7e3284b8f8d68da5caa996ed79bf69bb18038f84a9f53139eeb58`.
- `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md`; pre-edit SHA-256: `f0c7c9c1e981fe8414d0883f6dafe6d41946223f0aa7fe6c71a7a8c727e0a70b`.
- `00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md`; pre-edit SHA-256: `ae23e7a9df9630b66c9628de15d05f2141ab6a26c09d686c97197988d87f83ad`.
- `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md`; pre-edit SHA-256: `539319fde09cdda84d33ebaaa0bfe5b4ce9e298ae5366b452e4ac7dcda831167`.
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`; pre-edit SHA-256: `7f6cb003f84d3cae552dfa2b3195b1ad5cc788b78904ea7b5e05e31db0279af7`.
- `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`; pre-edit SHA-256: `7fba4f263ddf025f4c11ac89420e3a62ab247b3d1cee2b996778f27fb3638b21`.
- `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`; pre-edit SHA-256: `068b633c27aad5e9c411322fd05e705baec4c5d158eaca6803e1e44a8b26e95c`.
- `00_nucleo/adr/typst-adr-0076-introspector-completion.md`; pre-edit SHA-256: `43ff4473f3e05cfea73ae9f6a8f36aff1de66e0f97362d08d5c201c6ecf9d1d0`.
- `00_nucleo/adr/typst-adr-0077-regex-l1.md`; pre-edit SHA-256: `dbe5d17871fc5a1a4b3e0980e841f52b538e577c684eb0d53399d1acb00fc69b`.
- `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`; pre-edit SHA-256: `35a6b89caa21ebe546d2fb00b2dd128e14e26efb2e36e2a2f14feee876e434e7`.
- `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md`; pre-edit SHA-256: `0d2239c6479a88482112d1e48524f7ddb0220e7234aa034ee0a837cea19bca3c`.
- `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md`; pre-edit SHA-256: `330c9c50bcea518ed61cd0ea8f61319af44bd620cbccae9ace1cfb0e9f314f04`.
- `00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md`; pre-edit SHA-256: `79d3119e8eacd27c280c5fda05efe2122d80740420df5aa49443a43a20342bb6`.
- `00_nucleo/adr/typst-adr-0084-auditoria-condicional.md`; pre-edit SHA-256: `7e4ffcd48cb9acda759e3838acf53ff4f4fb43219f8b0bf76188ee8c8bbe5b56`.
- `00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md`; pre-edit SHA-256: `cdc1504693671e90395164ebb38686c7924eb44caf85f92cf26b93df4ea5ad0b`.
- `00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md`; pre-edit SHA-256: `a64881b1384ff0e9303222580eb27779787c6d7c94a607996fbceecd70194562`.
- `00_nucleo/adr/typst-adr-0087-gradient-linear-only.md`; pre-edit SHA-256: `f251b1b06e4df8e2b7f55cc490e74759bb3c06c7bdc668fc7d7580692a848a85`.
- `00_nucleo/adr/typst-adr-0088-gradient-radial-only.md`; pre-edit SHA-256: `c34682b8b0adc7fd488185ae1196f6d055b9769a5f2952c0802801fbc6cb8a71`.
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`; pre-edit SHA-256: `baa612841e0c0a9743a09bf056f15e400c2de2cb7c872502c08e8f9baf744aaa`.
- `00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md`; pre-edit SHA-256: `7c29884efbe8505bc9053b7b5b704daf011722ac6d721e56cf95f0dd0e4a700f`.
- `00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md`; pre-edit SHA-256: `945377cd8326fa1a1cff135d9c64855073297ab20dac0ffbe6aeb5e2931e510e`.
- `00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md`; pre-edit SHA-256: `ed302e6219e6d9adb403566cacbe45eb2b2c6a941830a82c8099564f7ae80be5`.
- `00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md`; pre-edit SHA-256: `07af75a6b6cc389af27e0662b36bb38faa378d43f834a044c390e41b60b7f904`.
- `00_nucleo/adr/typst-adr-0095-dedup-arc-as-ptr-resources.md`; pre-edit SHA-256: `e4865d0bb8cf77e2e8f2b088f3c7d38c2248467c055d6250461be789df7a3feb`.
- `00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md`; pre-edit SHA-256: `a208cdc3e47d5900d7ad1abca3c38a2b368b04d1909ce90d1ffe0bb96a2331b1`.
- `00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md`; pre-edit SHA-256: `591a8d158a20adb729c49ef48546223a1e34a1d809fc66353b1e942bdefbe5a1`.
- `00_nucleo/adr/typst-adr-0100-coesao-por-dominio-l3.md`; pre-edit SHA-256: `f64cefcf789082c95fd7e510c7ea8731430fb8dd6f5d8f71468fc6d5a6b24938`.
- `00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md`; pre-edit SHA-256: `a98c777bd1b666292bd8adebd27bb47530cf6900f3260bd55e9cb8efa238f349`.
- `00_nucleo/adr/typst-adr-0102-math-style-mechanism.md`; pre-edit SHA-256: `b6271131089e3ae3837831efa37719240896e6069c5dacb9126763d84de1a751`.
- `00_nucleo/adr/typst-adr-0103-math-style-composition.md`; pre-edit SHA-256: `dfd42e3590cca99624a209040de3779fe3c329a1359a604e5d8e02df187de69f`.
- `00_nucleo/adr/typst-adr-0104-atomicidade-para-agentes.md`; pre-edit SHA-256: `bc8dbdf74b39db3f979da719436c4603eaab89ea8432054c80c52638b3fb37d9`.
- `00_nucleo/adr/typst-adr-0105-modelo-elemento-d-agora-f-destino.md`; pre-edit SHA-256: `a240b51ad3ea6dce77fdb9e9d3fc3de516598c0fd16ba7e611e8e5d2603c039c`.
- `00_nucleo/adr/typst-adr-0106-fronteira-de-extensao-e1.md`; pre-edit SHA-256: `255ef37ccc0acd8f95ba7d38afdd941152fc92d940a37471cdffd81d99f4891e`.
- `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`; pre-edit SHA-256: `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d`.
- `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md`; pre-edit SHA-256: `20056a75b2dd861c468a0c079583ad5f1be76ac5b7a6cccf859ac2a1e1605bf3`.
- `00_nucleo/adr/typst-adr-0109-atomizacao.md`; pre-edit SHA-256: `0cbd3049418073e9b8efd0be1a19030c9ecde905a25ebf3d20922e3cef02676e`.
- `00_nucleo/adr/typst-adr-0110-marcador-vanilla.md`; pre-edit SHA-256: `fa69c001eae0e327fa58986864ba91a8d55dfb57d36ab8e62b720a733423cf7b`.
- `00_nucleo/adr/typst-adr-0111-crates-parsing-loading.md`; pre-edit SHA-256: `9c31c57e8f58cf5c86d0bdee5f71db82b78c4d3667180e6ff01dba3a0f972470`.
- `00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md`; pre-edit SHA-256: `32d78e4e58bb6b01b83c35d353bff3edd24549e0c1b20d171b7396f101dc11bc`.
- `00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md`; pre-edit SHA-256: `39d7c6be82a4bd8ba1287cddf6c77fca867255e52f8aea24414c0e9e587dcff9`.
- `00_nucleo/adr/typst-adr-0114-sonda-antes-da-spec.md`; pre-edit SHA-256: `edf031b419c68670afd16cd249687b2877771413f63e546944f232245113d894`.
- `00_nucleo/adr/typst-adr-0115-infra-benchmark-scanner.md`; pre-edit SHA-256: `095c4f27e6561cfa27d94b8ff9371f2e72133578cd87a8fc560a234f4e72de5b`.
- `00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md`; pre-edit SHA-256: `dfac81910f3279ddcc57cf8a4539275d050a4c13f4b1be91956e3818a6bc7b0d`.
- `00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md`; pre-edit SHA-256: `dba5b947687a176a9ea1f1a905eb50a7ab8388566013a8f710eb664b5afe8f6a`.
- `00_nucleo/adr/typst-adr-0118-runtime-state-context.md`; pre-edit SHA-256: `46e81f9a30a0fcc36356fffbaebdb93b1fb714746d5fc263ff1ceedbc0cbbde1`.
- `00_nucleo/adr/typst-adr-0119-disciplina-verificacao.md`; pre-edit SHA-256: `d15068e21b729392b8d95e18000e31b1c62958f037b714e7dc64fb4c921e56e2`.
- `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md`; pre-edit SHA-256: `e8539b4f8c69d9750e196548c58311003275a48499ef7c52d173f0d79919ccde`.
- `00_nucleo/adr/typst-adr-0121-proveniencia-medicao.md`; pre-edit SHA-256: `00be2575c57efb572b7935ab0ae9da80d62e66c8739bffbb17994b2f7a50d5ef`.
- `00_nucleo/adr/typst-adr-0122-paridade-defeitos-testes.md`; pre-edit SHA-256: `e4ab43919aee47167a70d6e8558b3af2fdaddd9c7f1110c7b16d748ed979874f`.
- `00_nucleo/adr/typst-adr-0123-geometria-tipografica.md`; pre-edit SHA-256: `5b6fa07e62008f3933f42dec9fc3248b91a5fa49a65148f9d75cf2b97de723ed`.
- `00_nucleo/adr/typst-adr-0124-checklist-sublayouts.md`; pre-edit SHA-256: `50a398ef86c81a81b7202d4e8ae1f117e989fc635481caac8bbaddc8fe57d7ce`.
- `00_nucleo/adr/typst-adr-0125-implementacao-vs-linguagem.md`; pre-edit SHA-256: `50cdb52ebd161b0eed8042d279ad9f7a7308f40ecce3da921a9e36b041b259de`.
- `00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md`; pre-edit SHA-256: `a9322153f2ed1f03599b9ff5e6bb77904b49508e3753b4eda59c1f671df3bed1`.
- `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md`; pre-edit SHA-256: `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9`.
- `00_nucleo/adr/typst-adr-0128-html-target-semantico.md`; pre-edit SHA-256: `917c671f03be9637aae8c3c8556ab21558a78a5a61842a49e4c8b59605bc274e`.
- `00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md`; pre-edit SHA-256: `b122d052691086846c4c2e9975322907a8f95cc83553e7d2f55e5844cfbdda62`.
- `00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md`; pre-edit SHA-256: `86fe612e0e04047f087ac1f7acaf398a3b175994576b946a2868ea4515576801`.
- `00_nucleo/diagnosticos/adr-0130-step-1060-history.md`; pre-edit SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

## Working-tree provenance before edits

`git diff HEAD --stat` (includes unrelated pre-existing work):

```text
 00_nucleo/adr/README.md                            |  4301 +-----
 00_nucleo/adr/adr-stub-vs-fallback.md              |    76 -
 00_nucleo/adr/template-adr.md                      |    77 +-
 .../adr/typst-adr-0002-hierarquia-contencao.md     |   182 -
 00_nucleo/adr/typst-adr-0003-comemo-contencao.md   |   159 -
 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md |   198 -
 00_nucleo/adr/typst-adr-0007-rustc-hash.md         |    85 -
 00_nucleo/adr/typst-adr-0028-typographic-types.md  |    52 -
 .../adr/typst-adr-0060-model-structural-roadmap.md |   779 --
 .../adr/typst-adr-0061-layout-fase-x-roadmap.md    |  1645 ---
 .../typst-adr-0066-introspection-runtime-adiada.md |   496 -
 .../adr/typst-adr-0078-column-flow-algorithm.md    |     3 +-
 .../adr/typst-adr-0079-layout-fase-5-roadmap.md    |  2628 ----
 ...dr-0081-m7-plus-pipeline-restructuring-scope.md |  1085 --
 ...090-gradient-conic-strategy-type-4-vs-type-1.md |   561 -
 ...tern-debt-37-campo-layouter-consumer-pending.md |   259 -
 ...t-adr-0097-scope-out-reconfirmado-por-fase-a.md |   269 -
 .../adr/typst-adr-0130-referencias-canonicas-l0.md |    42 +-
 00_nucleo/prompts/compiler/eval.md                 |    43 +-
 00_nucleo/prompts/compiler/layout.md               |    23 +-
 00_nucleo/prompts/compiler/layout/equation.md      |     6 +-
 00_nucleo/prompts/compiler/layout/page_geometry.md |     7 +-
 00_nucleo/prompts/compiler/layout/placement.md     |     6 +-
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/parse/parser.md         |    24 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    28 +-
 00_nucleo/prompts/entities/content.md              |   119 +-
 00_nucleo/prompts/entities/syntax-node.md          |   230 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 00_nucleo/prompts/infra/export/html.md             |   405 +-
 00_nucleo/prompts/infra/export/images.md           |   206 +-
 .../prompts/infra/export/p1250_svg_integrated.md   |     6 +-
 00_nucleo/prompts/infra/export/stream.md           |    37 +-
 00_nucleo/prompts/infra/export/svg.md              |   556 +-
 00_nucleo/prompts/infra/font_metrics.md            |  1085 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |    11 +-
 00_nucleo/prompts/infra/plugin_host.md             |   333 +-
 00_nucleo/prompts/infra/shaper.md                  |   954 +-
 00_nucleo/prompts/infra/system-world.md            |   431 +-
 00_nucleo/prompts/wiring.md                        |   978 +-
 01_core/Cargo.toml                                 |     3 +
 01_core/src/compiler/eval/bibliography.rs          |     2 +-
 01_core/src/compiler/eval/bindings/access.rs       |    29 +-
 01_core/src/compiler/eval/bindings/binding.rs      |     4 +-
 01_core/src/compiler/eval/bindings/field_access.rs |    88 +-
 .../src/compiler/eval/bindings/method_dispatch.rs  |    34 +-
 .../src/compiler/eval/bindings/value_methods.rs    |    24 +-
 01_core/src/compiler/eval/call_dispatch.rs         |    49 +-
 01_core/src/compiler/eval/closures.rs              |     6 +-
 01_core/src/compiler/eval/control_flow.rs          |     6 +-
 01_core/src/compiler/eval/font_dict.rs             |     7 +-
 01_core/src/compiler/eval/markup.rs                |     4 +-
 01_core/src/compiler/eval/math.rs                  |    12 +-
 01_core/src/compiler/eval/mod.rs                   |  6587 +--------
 01_core/src/compiler/eval/modules.rs               |    18 +-
 01_core/src/compiler/eval/operators/arithmetic.rs  |    12 +-
 01_core/src/compiler/eval/p1427_tests.rs           |     6 +-
 01_core/src/compiler/eval/p1430_tests.rs           |     2 +-
 01_core/src/compiler/eval/p1431_tests.rs           |     2 +-
 01_core/src/compiler/eval/repr.rs                  |    36 +-
 01_core/src/compiler/eval/rules.rs                 |   406 +-
 01_core/src/compiler/eval/selector_matching.rs     |     9 +-
 01_core/src/compiler/eval/show_rule_termination.rs |     6 +-
 01_core/src/compiler/eval/tests.rs                 |   947 +-
 01_core/src/compiler/introspect.rs                 |    54 +-
 01_core/src/compiler/introspect/fixpoint.rs        |     8 +-
 01_core/src/compiler/introspect/from_tags.rs       |    41 +-
 01_core/src/compiler/introspect/locatable.rs       |     4 +-
 01_core/src/compiler/layout/columns.rs             |    10 +-
 01_core/src/compiler/layout/decorations.rs         |     4 +-
 01_core/src/compiler/layout/equation.rs            |   153 +-
 01_core/src/compiler/layout/format_options.rs      |    43 +-
 01_core/src/compiler/layout/mod.rs                 |   914 +-
 01_core/src/compiler/layout/page_geometry.rs       |    47 +-
 01_core/src/compiler/layout/placement.rs           |   128 +-
 01_core/src/compiler/layout/sequence.rs            |    63 +-
 01_core/src/compiler/layout/set_page.rs            |    75 +-
 01_core/src/compiler/layout/tests.rs               |  1083 +-
 01_core/src/compiler/layout/word_segment.rs        |    44 +-
 01_core/src/compiler/math/layout/accent.rs         |     4 +-
 01_core/src/compiler/math/layout/attach.rs         |     4 +-
 01_core/src/compiler/math/layout/mod.rs            |    47 +-
 01_core/src/compiler/math/layout/p1430_tests.rs    |     2 +-
 01_core/src/compiler/math/layout/p1431_tests.rs    |     2 +-
 01_core/src/compiler/math/layout/spacing.rs        |    18 +-
 01_core/src/compiler/math/layout/tests.rs          |     2 +-
 01_core/src/compiler/parse/parser.rs               |   629 +-
 01_core/src/compiler/stdlib/color.rs               |   219 +-
 01_core/src/compiler/stdlib/counter.rs             |    10 +-
 01_core/src/compiler/stdlib/format.rs              |    12 +-
 01_core/src/compiler/stdlib/foundations/color.rs   |     6 +-
 01_core/src/compiler/stdlib/gradients.rs           |    11 +-
 01_core/src/compiler/stdlib/layout.rs              |    36 +-
 01_core/src/compiler/stdlib/loading.rs             |     5 +-
 01_core/src/compiler/stdlib/mod.rs                 | 13854 +------------------
 01_core/src/compiler/stdlib/structural/document.rs |    45 +-
 01_core/src/compiler/stdlib/structural/math.rs     |     2 +-
 01_core/src/compiler/stdlib/structural/mod.rs      |    54 +-
 01_core/src/compiler/stdlib/structural/outline.rs  |     6 +-
 .../src/compiler/stdlib/structural/p1429_tests.rs  |     2 +-
 .../src/compiler/stdlib/structural/p1430_tests.rs  |     2 +-
 01_core/src/compiler/stdlib/structural/par.rs      |     2 +-
 01_core/src/compiler/stdlib/text/constructor.rs    |     2 +-
 01_core/src/entities/color.rs                      |    31 +-
 01_core/src/entities/content.rs                    |  6763 +--------
 01_core/src/entities/content_hash.rs               |    10 +-
 01_core/src/entities/elements/figure_caption.rs    |     6 +-
 01_core/src/entities/elements/footnote_entry.rs    |     8 +-
 01_core/src/entities/gradient.rs                   |    24 +-
 01_core/src/entities/locator.rs                    |     1 -
 01_core/src/entities/style.rs                      |     4 +-
 01_core/src/entities/style_chain.rs                |    18 +-
 01_core/src/entities/syntax_node.rs                |  1122 +-
 02_shell/src/cli.rs                                |    34 +-
 03_infra/src/export/builder.rs                     |  4274 +-----
 03_infra/src/export/html.rs                        |  1286 +-
 03_infra/src/export/images.rs                      |   596 +-
 03_infra/src/export/stream.rs                      |  2135 +--
 03_infra/src/export/svg.rs                         |  3271 +----
 03_infra/src/export/tests/mod.rs                   |     2 +-
 .../tests/p1475_pdfa_conformance_core_tests.rs     |     8 +-
 .../tests/p1476_pdfa_attachments_af_tests.rs       |    14 +-
 .../p1477_pdfa_icc_version_and_cidset_tests.rs     |    12 +-
 .../p1478_pdfa4f_empty_embedded_files_tests.rs     |     9 +-
 03_infra/src/font_metrics.rs                       |  3373 +----
 03_infra/src/integration_tests.rs                  |     2 +-
 03_infra/src/pipeline.rs                           |  6005 +-------
 03_infra/src/pipeline/context_stabilization.rs     |     7 +-
 03_infra/src/plugin_host.rs                        |   838 +-
 03_infra/src/query_helpers.rs                      |     9 +-
 03_infra/src/shaper.rs                             |  2648 +---
 03_infra/src/watch.rs                              |    67 +-
 03_infra/src/world.rs                              |  1131 +-
 03_infra/tests/p1250_svg_integrated.rs             |     6 +-
 04_wiring/src/main.rs                              |  1358 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    27 +
 139 files changed, 4500 insertions(+), 77017 deletions(-)
```

## Validation

- All 116 active numbered ADRs have exactly one canonical English primary
  Status field: 68 ACCEPTED, 44 IMPLEMENTED, 4 PROPOSED. These are checkpoint
  counts, not a claim that prose translation is complete.
- ORDER.md is a bijection with the current active numbered files; no duplicate,
  missing or nonexistent catalog entries. README section names now match ORDER.
- All concrete L0 examples in README/template/ADR-0130 resolve to existing files.
- Each Rust example in ADR-0130 names its actual existing `@prompt` owner.
- Every non-ADR-0130 numbered record was reconstructed from its normalized
  primary metadata and compared with its pre-edit SHA-256: the rest of each
  document is unchanged. Original metadata trailing whitespace is not normative.
- `git diff --check` for the scoped documents passed.
- `crystalline-lint .` exited 0; pre-existing warning/info diagnostics remain.
- No build or Rust test was needed for this documentation-only change.

No rename, new status promotion, prompt resealing or source edit was performed.
