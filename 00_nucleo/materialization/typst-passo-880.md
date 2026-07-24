# Prompt — typst-passo-880: extração de `coverage` lazy — recuperar a vantagem original nos cenários simples

**Origem**: causa isolada por P877 e confirmada por P879 — `font_info_from_bytes` (P875) extrai `coverage` (percorrendo a tabela `cmap`) incondicionalmente para todas as ~1086 fontes do sistema durante a descoberta inicial, mesmo em documentos que nunca usam fallback. Isso custa ~55-60ms fixos por compilação, e é o que impede os cenários simples do benchmark de voltarem à vantagem original que o cristalino já tinha sobre o vanilla (0.35×-0.44×, medido em P872, antes de qualquer regressão).
**Estado**: aguardando execução

---

## Contexto: o que já está confirmado, não precisa resondar

- P877 mediu: com `coverage: Coverage::new()` (vazio) em vez de `extract_coverage(&face)`, `01-hello` volta a 0.35× (2.71× mais rápido que o vanilla) — confirma que a extração eager é a causa isolada da regressão residual.
- P879 já corrigiu a parte do fallback que consome a cobertura (`FallbackFontMetrics::covering` e `CandidateSet::covering_all`, este último desde P875) — o problema agora é só a extração acontecer cedo demais, não o uso dela.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate. Benchmark completo do P872 obrigatório no fechamento, com os dois critérios já definidos por P879: math mantendo ~19× ou melhor, **e agora** os quatro cenários simples voltando a 0.35×-0.44×.

## Passo 1 — Tornar a extração de `coverage` lazy

1. Localizar `font_info_from_bytes` (`03_infra/src/fonts.rs`) e o ponto onde `extract_coverage(&face)` é chamado incondicionalmente durante a descoberta (`discover_fonts`/`pair_slots_with_book`/`load_system_fonts`, conforme já mapeado por P877).
2. Mudar `FontInfo.coverage` para ser calculado sob demanda, não na descoberta — por exemplo, `Coverage` vira `OnceLock<Coverage>` (ou equivalente) dentro de `FontInfo`, calculado na primeira vez que `candidates_for_char` (ou qualquer consumidor) precisar dela para aquela fonte específica, não para as 1086 de uma vez.
3. Confirmar que isso não reintroduz o problema original que motivou P875 em primeiro lugar (busca sem filtro, abrindo arquivo inteiro para descobrir cobertura) — a diferença é que agora só a fonte que **de fato** é candidata (já filtrada por outro critério mais barato, se houver, ou pela ordem de tentativa) paga o custo de extrair cobertura, não todas de uma vez adiantado. Se não houver like um filtro mais barato antes de chegar em `coverage`, considerar se faz sentido calcular a cobertura só depois que o glyph_index real já foi verificado uma vez (cache do resultado, não pré-cálculo).

## Passo 2 — Confirmar que não regride o ganho de matemática

A extração lazy precisa continuar entregando cobertura correta quando `candidates_for_char` for chamado no caminho de fallback de matemática (o que P879 corrigiu) — só adiando **quando** a cobertura é calculada, não mudando **o que** ela calcula. Testar o cenário de matemática de novo para confirmar que continua em ~19× ou melhor, não regredindo pela mudança de timing.

## Passo 3 — Adicionar teste de regressão para o loop do P879 (pendência que ficou aberta)

P879 corrigiu `FallbackFontMetrics::covering` para usar `candidates_for_char` em vez de `0..book.len()`, mas não deixou nenhum teste automatizado travando esse comportamento — hoje só um `strace` manual comprova. Isso é o mesmo tipo de regressão silenciosa que already aconteceu uma vez (P875 quebrou algo sem ninguém notar até o benchmark completo). Adicionar:

1. Um teste unitário que confirme que `FallbackFontMetrics::covering` não carrega faces fora do conjunto devolvido por `candidates_for_char` (ex.: um `FontBook` de teste com uma fonte CJK sintética e uma fonte latina, verificando que buscar cobertura para um caractere latino não toca a fonte CJK).
2. Se possível, um teste de nível mais alto (integração) que conte aberturas de arquivo/chamadas de carregamento de fonte para um documento de teste simples, travando um limite superior razoável — para que uma regressão futura desse tipo quebre um teste automatizado, não só apareça num benchmark manual que alguém precisa lembrar de rodar.

## Passo 4 — Validação final

1. Repetir o benchmark completo do P872, sete cenários, mesma metodologia.
2. Critério de fechamento, agora completo: matemática mantendo ~19× ou melhor (não regredir o ganho de P879); os quatro cenários simples voltando para perto de 0.35×-0.44×; imagens melhorando também, já que P879 já confirmou que a causa ali é a mesma (descoberta de fontes geral, não específica de imagem).
3. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-880-relatorio.md` com: a implementação da extração lazy, a confirmação de que matemática não regrediu, os testes de regressão novos (Passo 3), a tabela completa dos sete cenários comparando P872 (original), P876 (regredido), P879 (correção parcial) e P880 (este passo) lado a lado, e as contagens de teste discriminadas por crate. Se algum dos critérios de fechamento não for atingido, dizer isso com a mesma clareza que P879 usou, não forçar a conclusão.
