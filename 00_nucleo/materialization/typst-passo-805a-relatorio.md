# Relatório — typst-passo-805a (sub-passo de P805): ligaduras fi/ffi sem ToUnicode no embed integral CFF

**Data:** 2026-07-21
**Executor:** Kimi Code (nesta conversa; sub-passo nascido dentro de P805 — bug bloqueante descoberto na validação de P805)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543` + working tree P799–P805. Hora: 2026-07-21 ~17:20 -0300.

---

## Achado (nascido da validação de P805)

`#lorem(30)` divergia do vanilla só em palavras com "fi". Sonda com caso mínimo `fi fierce fish final office`:

- cristalino (`pdftotext`): `erce sh nal o ce` — caracteres "fi"/"ffi" perdidos na extracção.
- vanilla: `fi fierce fish final office`.

## Diagnóstico (com prova em cada passo)

1. **Renderização correcta**: `mutool draw -r 150` do PDF cristalino mostra o texto completo e correcto (imagem verificada). O problema é só de extracção.
2. **Glifo presente no content stream**: o `TJ` emite `<095E>` (= gid 2398) para "fi".
3. **Fonte integral contém o glifo**: o `FontFile3` embutido (`/Subtype /OpenType`, OTTO, 337132 bytes) tem `numGlyphs=2793` e `gid 2398 = "f_i"` (verificado com fontTools na venv do lab). Gids Latin básicos coincidem com o shaping ('e'=70, 'r'=83, 'c'=68).
4. **Causa**: `03_infra/src/export/builder.rs` (`build_cidfont`) — os cluster texts shaped (`collect_shaped_cluster_texts`, que mapeiam gid → texto do cluster, ex. "fi") só entravam no ToUnicode CMap `if !glyph_mapping.is_empty()`. No fallback de embed integral de P797 (CFF, mapping vazio), as ligaduras ficavam sem entrada ToUnicode → poppler/mutool descartam o glifo na extracção.

## Correcção

`builder.rs`: o bloco de cluster texts passa a correr **sempre**; `remap_glyph_id` só quando há subset (no embed integral o gid final é o próprio `old_gid`). L0 `00_nucleo/prompts/infra/export/builder.md` §P805a; hash `5b1854d7`.

```diff
-        if !glyph_mapping.is_empty() {
-            for (old_gid, hex) in collect_shaped_cluster_texts(doc) {
-                let new_gid = remap_glyph_id(old_gid, &glyph_mapping);
-                ...
-            }
-        }
+        // **P805a** — incluir os cluster texts TAMBÉM no embed integral
+        // (fallback P797, `glyph_mapping` vazio): (...)
+        for (old_gid, hex) in collect_shaped_cluster_texts(doc) {
+            let new_gid = if glyph_mapping.is_empty() {
+                old_gid
+            } else {
+                remap_glyph_id(old_gid, &glyph_mapping)
+            };
+            ...
+        }
```

## Validação

- Teste novo (escrito primeiro; falhou antes): `p805a_ligatura_fi_tem_entrada_to_unicode_no_embed_integral` (`03_infra/src/integration_tests.rs`) — compila `fi fierce` com Libertinus Serif embutido (CFF) e exige `<00660069>` ("fi" UTF-16BE) no ToUnicode do PDF.
- E2E: `fi fierce fish final office` → extracção cristalina `fi fierce fish final office` == vanilla ✓.
- E2E: `#lorem(30)` e `#lorem(100)` → extracção IDÊNTICA ao vanilla ✓ (fecha a validação de P805).
- `cargo test -p typst-infra`: **657** passed; 0 failed; 5 ignored (+1 teste novo ✓).
- `cargo test -p typst-core --lib`: **4329** passed; 0 failed; 1 ignored (inalterado — fix em L3).
- `crystalline-lint .` → exit 0.

## Nota residual (registada, não corrigida)

O array `/W` do CIDFont no caminho de embed integral só cobre os glifos com entrada ToUnicode — ligaduras passam a estar cobertas por via desta correcção (o `/W` é construído de `to_unicode_mappings`). Não foi verificado se o vanilla inclui larguras para todos os glifos; sem impacto observado na extracção nem na renderização.
