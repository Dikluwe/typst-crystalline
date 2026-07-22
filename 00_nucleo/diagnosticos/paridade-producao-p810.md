# Relatório de Verificação — Passo 810: triagem sistemática, lote 4 (15 módulos)

**Data:** 2026-07-21
**Status:** Concluído — 15/15 módulos com achados (taxa de sinal 100%, com nuances registadas); 15 achados na fila
**Proveniência da Medição:**
- **Commit Base:** `c98ffc8ac` (HEAD) + working tree P808/P809 não commitado (18 ficheiros, +419/-876)
- **Hora das Medições:** 2026-07-21 ~18:45–20:00 (-0300)
- **Relatório detalhado (prova literal completa por módulo):** `00_nucleo/diagnosticos/typst-passo-810-relatorio.md`
- **Método:** 15 subagentes (1 por módulo), cada um obrigado a teste específico do módulo (lição de P798: proibido documento genérico), com comando + saída literal vanilla vs cristalino.

---

## 1. Selecção

Lista congelada `lente-lista-B-2026-07-15.txt` (secção `lacuna-inventario`: 82 módulos no snapshot actual) menos os 45 já triados (P785/P786/P798) = **37 restantes** (a estimativa "~21" do handoff era aproximada). Seleccionados os 15 de maior superfície de língua. Restam **22** para o lote 5 (maioritariamente internos/mecânicos).

## 2. Resultados por módulo (resumo; prova completa no relatório de materialização)

| # | Módulo | Classificação | Achado principal |
|---|---|---|---|
| 1 | `typst_eval` | achado | `#eval` sem `mode:`/`scope:`; erros genéricos + span detached |
| 2 | `typst_eval::methods` | achado | método inexistente: caminho normal diverge; dict-key-call sem hints |
| 3 | `typst_library::diag` | achado | `#set` prop inválida → warning (exit 0!) em vez de erro; sem warning "unknown font family"; `set text(size: 12)` aceite |
| 4 | `foundations::calc` | achado | asin/acos/atan/atan2 float vs `angle`; quo trunc vs floored; pow int-neg; decimal ausente; log10/deg/rad extra; precisão erf/log/exp |
| 5 | `foundations::ops` | achado | ordenação str/array/bool ausente; div Relative/Relative, Ratio/Ratio; Str*Int; eq/ord Length↔Relative; coerção não aninhada |
| 6 | `foundations::plugin_` | achado | `plugin.transition` ausente (não registado no L0); mensagens L1 divergentes; spans detached (caminho feliz em paridade total) |
| 7 | `foundations::scope` | achado | `Deprecation` ausente (`join` erro vs warning; `bowtie` ausente); núcleo em paridade verbatim (12 testes) |
| 8 | `foundations::target_` | achado | `#target()` fora de `#context` não erra (vanilla: erro + 2 hints); colateral: `#context type()` vazio |
| 9 | `layout::grid::resolve` | achado | header/footer não repetem (débito reconfirmado); mensagens divergentes; footer fora do fim aceite |
| 10 | `loading::cbor_` | achado | mensagem de erro CBOR: Debug do ciborium vs texto amigável + ficheiro + span |
| 11 | `loading::read_` | achado | `encoding:` rejeitado (língua válida); não-UTF8 devolve bytes em silêncio (comentário "heurística vanilla" **refutado** por medição) |
| 12 | `typst_library::math` | achado | classes aceites 15 vs 10; field access bare em math compila; fence spaced sem efeito (scope-out L0); LeftRightAlternator em mat ausente |
| 13 | `math::style` | achado | display/inline/script/sscript **sem efeito geométrico**; itálico P809 perdido em wrappers de tamanho; scr bloco errado + sem variation selectors; `frak()` sem arg + **PDF corrompido**; NN/RR/ZZ/QQ/CC ausentes |
| 14 | `pdf::accessibility` | achado | `pdf.artifact(kind:)` rejeitado (gate A11yExtras confirmado nas outras 3 — paridade) |
| 15 | `typst_syntax::package` | achado | erro "pacote não encontrado" não-preview diverge (parsing em paridade verbatim, 5 casos) |

## 3. Taxa de sinal real (cálculo)

**15 achados / 15 módulos = 100%.** Nuance: a selecção foi pelos módulos de maior superfície de língua (não aleatória); vários módulos têm núcleos funcionais em paridade (scope 12/12, plugin caminho feliz, package 5/6, target_ valor, ops 21 expressões byte-idênticas, calc ~40 chamadas, math relation/unary com controlo). Os achados concentram-se em: validações ausentes (documento que é erro no vanilla compila no cristalino — o padrão mais grave), mensagens de erro divergentes (texto/língua/span), e features específicas ausentes.

**Achados transversais:** PDF corrompido com equação vazia (pista de bug de export, follow-up); `bytes()`/`float.nan`/`datetime()` construtores ausentes; `array.join` ausente; spans `<detached>` generalizados; vanilla emite variation selectors (U+FE0E/FE00/FE01) em símbolos e `cal`/`scr`.

## 4. Fila de passos dedicados

15 achados tabelados no relatório de materialização (formato handoff). Sugestões de prioridade pela gravidade medida: **PDF corrompido** (equação vazia); **display/script/sscript sem efeito** (math::style — qualquer documento com tamanhos explícitos); **asin/acos/atan/atan2 → angle** (calc); **#set prop inválida = exit 0** (diag); **Deprecation** (scope). Dois achados P809-adjacentes: itálico perdido em wrappers de tamanho (`letter_base` retorna None para size variants) e `scr`/variation selectors (limitação estrutural de `map_glyph` — 1 char vs `[char; 2]` do codex).

## 5. Verificação de Sucesso do Workspace

Sem código novo neste passo (triagem). Estado verde mantido: typst-core 4348/1i, typst-infra 658/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2. `crystalline-lint .` → exit 0.

Nota de proveniência registada: os relatórios de materialização P799–P806 foram eliminados fisicamente da working tree pela sessão paralela após serem commitados em `c98ffc8ac` (17:36) — estão preservados nesse commit; os de P808/P809/P810 foram escritos depois e estão untracked na working tree.
