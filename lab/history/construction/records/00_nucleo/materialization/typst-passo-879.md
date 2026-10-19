# Prompt — typst-passo-879: aplicar o filtro de coverage em `FallbackFontMetrics` — fechar a regressão e recuperar a vantagem original do cristalino

**Origem**: P878 confirmou e isolou a causa — `CandidateSet::covering_all` (shaper.rs) já usa `candidates_for_char` desde P875, mas `FallbackFontMetrics::covering` (`font_metrics.rs:734`) continua a iterar `0..book.len()` sem filtro, e é esse caminho que domina o tempo em matemática. Isolamento temporário já provou o ganho: 23.64× → 19.22×, RSS 9.41GB → 7.08GB, só corrigindo esse um loop.
**Estado**: aguardando execução — implementação real, com escopo já definido por P878.

---

## Contexto que importa para a prioridade disto

Antes das regressões de P875, o cristalino já era **2 a 3× mais rápido que o vanilla** nos cenários simples do benchmark de P872 (0.35× em hello, 0.38× em tables, 0.44× em context). Este passo não é só "melhorar um número ruim" — é recuperar uma vantagem estrutural que o projeto já tinha e perdeu por uma regressão não validada. O critério de fechamento inclui confirmar que essa vantagem original volta, não só que matemática melhora.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate. Benchmark completo do P872 obrigatório no fechamento — não aceitar como fechado sem ele, mesmo erro do P875.

## Passo 1 — Escopo mínimo (obrigatório, já provado por P878)

1. Em `03_infra/src/font_metrics.rs:734`, `FallbackFontMetrics::covering`: trocar `for slot_idx in 0..book.len()` por `for slot_idx in book.candidates_for_char(c)`, exatamente como P878 já validou em isolamento temporário.
2. Confirmar que a alteração é permanente desta vez (não uma reversão temporária de diagnóstico) — este é código de produção.

## Passo 2 — Escopo alargado (avaliar, não pular sem checar)

P878 identificou mais três loops com o mesmo padrão (`0..book_len()` carregando faces via `cached_face`) em `font_metrics.rs`: `line_metrics` (~linha 909), `cap_height` (~linha 957), `edge_metrics` (~linha 994).

1. Medir se esses três também contribuem de forma mensurável para o tempo nos cenários do benchmark (não só matemática — podem afetar outros cenários de forma mais sutil). Usar `strace`/`--timings-json` (com mais granularidade do que P877 usou, para não repetir o mesmo erro de bucket genérico escondendo causa) para confirmar.
2. Se contribuírem de forma mensurável, aplicar o mesmo filtro (`candidates_for_char`) a eles.
3. Se a contribuição for desprezível, registrar isso no relatório e não mexer — não otimizar código que não está custando nada de verdade.

## Passo 3 — Validação: benchmark completo do P872, os sete cenários

1. Rodar os sete cenários do benchmark original, mesma metodologia (`hyperfine`, `--warmup 1`, `--min-runs 10`, binários release, versões corretas dos dois lados).
2. Critério de fechamento, dois níveis:
   - **Mínimo (já estabelecido por P878)**: matemática cai de ~23× para ~19× ou melhor.
   - **Completo (novo, deste prompt)**: os quatro cenários simples (hello, lorem, tables, context) voltam para perto dos números originais de P872 (0.35×-0.44×), confirmando que a vantagem estrutural do cristalino foi recuperada, não só que matemática melhorou um pouco.
3. Se o critério completo não for atingido nos cenários simples, investigar por quê antes de fechar — pode haver mais alguma regressão residual não capturada por P877/P878 (que focaram em matemática e imagens, não voltaram a confirmar os quatro cenários simples depois da correção isolada).

## Passo 4 — Confirmar o efeito em imagens também

O cenário `03-images` também estava inflado pelo mesmo `load_system_fonts()`/descoberta geral (confirmado por P877 como não específico de imagem) — mas o loop específico corrigido aqui (`FallbackFontMetrics::covering`) pode ou não ser o caminho que imagens percorre (imagens não deveriam precisar de fallback de fonte, a menos que o documento tenha texto/legenda). Medir esse cenário também, sem assumir que a correção de matemática necessariamente o resolve.

## Relatório

`00_nucleo/diagnosticos/typst-passo-879-relatorio.md` com: a implementação do Passo 1 (permanente), a decisão do Passo 2 (o que foi medido e o que foi ou não aplicado), a tabela completa dos sete cenários do benchmark comparando com P872 (original) e P877 (regredido), confirmando se os dois critérios de fechamento (matemática ~19× ou melhor, simples voltando a 0.35-0.44×) foram atingidos, e as contagens de teste discriminadas por crate.
