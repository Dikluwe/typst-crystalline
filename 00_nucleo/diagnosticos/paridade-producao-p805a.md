# Relatório de Verificação — Passo 805a: ligaduras fi/ffi sem ToUnicode no embed integral CFF

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD) + working tree P799–P805
- **Hora da Medição:** 2026-07-21 ~17:20 (-0300)
- **Origem:** sub-passo nascido dentro de P805 — bug bloqueante descoberto na validação de `#lorem(30)`
- **Relatório de materialização:** `00_nucleo/materialization/typst-passo-805a-relatorio.md`

---

## 1. O Problema Relatado

Descoberto na validação de P805: `#lorem(30)` divergia do vanilla apenas em palavras com "fi". Caso mínimo `fi fierce fish final office` — extracção cristalina `erce sh nal o ce` (caracteres "fi"/"ffi" perdidos); vanilla correcto. Afecta **qualquer** documento com essas sequências.

## 2. Diagnóstico e Medição

Prova passo a passo:
1. **Renderização correcta** — `mutool draw -r 150` do PDF cristalino mostra o texto completo (imagem verificada). O problema era só de extracção.
2. **Glifo presente no content stream** — o `TJ` emite `<095E>` (gid 2398) para "fi".
3. **Fonte integral contém o glifo** — `FontFile3` embutido (`/Subtype /OpenType`, OTTO, 337132 bytes) tem `numGlyphs=2793` e `gid 2398 = "f_i"` (verificado com fontTools, `lab/.venv`). Gids Latin coincidem com o shaping ('e'=70, 'r'=83, 'c'=68).
4. **Causa** — `03_infra/src/export/builder.rs` (`build_cidfont`): os cluster texts shaped (`collect_shaped_cluster_texts`, gid → texto do cluster) só entravam no ToUnicode CMap `if !glyph_mapping.is_empty()`. No fallback de embed integral de P797 (CFF, mapping vazio), as ligaduras ficavam sem entrada ToUnicode → poppler/mutool descartam o glifo na extracção.

## 3. A Solução Implementada

L0 `infra/export/builder.md` §P805a; hash `5b1854d7`. O bloco de cluster texts passa a correr **sempre**; `remap_glyph_id` só quando há subset (no embed integral o gid final é o próprio `old_gid`).

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `p805a_ligatura_fi_tem_entrada_to_unicode_no_embed_integral` (novo, `03_infra/src/integration_tests.rs`): compila `fi fierce` com Libertinus Serif embutido (CFF — o caminho do fallback) e exige `<00660069>` ("fi" UTF-16BE) no ToUnicode do PDF. Falhou antes da correcção. (Uma primeira versão com fonte TrueType de sistema passava já — reescrita para o caminho CFF, que é o do bug.)

## 5. Verificação de Sucesso do Workspace

E2E: `fi fierce fish final office` → extracção idêntica ao vanilla ✓; `#lorem(30)` e `#lorem(100)` idênticos ✓ (fecha a validação de P805).

```
Suite 'typst-infra':       ANTES 656 passed; 5 ignored → DEPOIS 657 passed; 5 ignored (+1 ✓)
Suite 'typst-core' (lib):  4329 passed; 1 ignored (inalterado — fix em L3)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
