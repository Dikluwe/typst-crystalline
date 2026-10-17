# L0 — Resolução integral bidi em L3
Hash do Código: f5281c19

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8

**Layer:** L3.
**Exclusive consumer:** `03_infra/src/bidi_context.rs`.

## Medição anterior à decisão

`00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver linha e shaping isolados; `initial.json.gz` guarda proveniência e thirteen casos bilaterais. `lab/typst-original/crates/typst-layout/src/inline/collect.rs:156–178` conserva embedding explícito de direção oposta. API L1 ratificada pelo manifesto 1ae7513e; nenhuma ampliação de World/FontMetrics.

## Decisão

Registry privado à crate possui Vec entradas com Arc input, string auxiliar e ranges de cada segmento original. Descobrir inputs nos carriers de documento, envelopes e camadas sem inferir identidade por texto/geometria. Arc ptr_eq é somente mecanismo local para deduplicar o mesmo input imutável (não observável de língua). Construir texto sem normalização: Text original, Space U+0020, Linebreak LF; envolver Text cuja dir difere da base com LRE/RLE e PDF somente na string auxiliar.

Resolved empresta registry e possui BidiInfo com lifetime Rust normal para cada string, base explícita. Consulta span converte range original ao auxiliar, encontra parágrafo e usa visual_runs; retorna ranges relativos/RTL apenas dos bytes autorais, nunca embeddings auxiliares. Consulta linha inclui intervalo entre menor e maior span para regras de fim de linha; retorna runs para interseção exata pelos consumers. Não estimar origem pelo ponto médio ou substring. Recusar span cujo trecho não corresponda ao carrier. Fonte pode conter controles e eles permanecem.

Cache tem vida local à compilação e pode ser compartilhado entre reorder/shaper por helpers internos; wrappers públicos constroem equivalente local. Não reter World/watch, sem globals mutáveis/unsafe.

## Aceitação

RED→GREEN: 456 789 dentro de ABC 123 456 789 مرحبا é LTR único na base RTL e não invertido por neutral como a string isolada. Controles auxiliares não aparecem nos runs originais; embeddings autorais permanecem. Prefix/suffix, UTF8, override e linha parcial respeitam identidade. Validar source→PDF bilateral.


## Quebra explícita: medição antes da correção

`verification.json.gz` do segundo candidato registra numeric-explicit: Tekt 456/789 versus vanilla 789/456 na segunda linha; wrap suave 456/789 coincide. `lab/typst-original/crates/typst-layout/src/inline/collect.rs:191–194` insere LF para Linebreak não justificado, U+2028 somente para justify=true. Linebreak do carrier unitário desta rodada representa somente a quebra não justificada e gera LF auxiliar. O producer recusa justify=true (não representar o estado ausente por inferência). Correção interna de paridade, mesma fase e API.

## Consultas sem copiar todos os níveis do input

Medição anterior à decisão: fonte unicode-bidi 0.3.18 `src/lib.rs:546–560,669–676` clona self.levels inteiro em cada visual_runs. Receipt `line-view-before.json.gz` preserva fonte, SHA, HEAD/stat/UTC e resolver anterior. Não repetir essa cópia por palavra/linha: construir ParagraphBidiInfo temporário somente com text/classes/levels do intervalo, copiando os níveis já resolvidos do input inteiro, conservando paragraph_level. visual_runs local aplica apenas L1/L2 do Unicode sobre o slice; traduzir ranges de volta ao input. Nenhum BidiInfo::new adicional nem auto-detecção sobre string fragmentada. Aceitação pelos testes de resolução integral/números/break e CLI; medir custo, não inferir ganho de tempo só do tamanho copiado.
