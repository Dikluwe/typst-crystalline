# Passo 967 — espaço ausente na transição entre matemática inline e a palavra seguinte (seção 8)

**Precede este passo**: achado novo, auditoria externa (2026-08-05, seção 8.2) — na seção "Alinhamento
de Equações", toda transição entre expressão matemática inline e a palavra de anotação seguinte
("dado", "multiplique", "subtraia", "divida") perde o espaço: 0.0-0.06pt no cristalino contra
3.65-4.85pt no vanilla, nas 4 ocorrências, nos dois documentos. Espaçamento palavra-palavra comum
("multiplique por") permanece normal nos dois lados — a divergência é isolada à transição
matemática→palavra especificamente.

**Pré-condição de árvore**: `git status`. Confirmar P966 presente.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente: `$ 3x + y = 9 $ dado` (ou equivalente mínimo) e confirmar, via
   `mutool trace`/`pdftotext -bbox`, que o espaço desaparece mesmo fora do documento de 30 seções.
2. Ler o código que decide o espaçamento entre o fim de uma equação inline e o texto que a segue —
   candidato: o ponto onde `Content::Equation` (inline) é composto de volta ao fluxo de parágrafo
   (`flow`/`par`, fora de `math/layout/`) — confirmar se o espaço declarado no `.typ` (espaço
   literal entre `$...$` e a palavra seguinte) está sendo descartado na transição de volta para
   texto normal.
3. Confirmar se o problema é específico a equações **sem** número/rótulo (a seção 8 usa `\=` para
   alinhamento, sem números de equação visíveis) — comparar com uma equação inline numerada
   seguida de palavra, para isolar se o rótulo de número interfere.
4. Ler o mecanismo real do vanilla para esta transição, confirmar a origem do espaço de 3.65-4.85pt
   (é um espaço fixo entre inline math e texto, ou é literalmente o espaço do `.typ` preservado
   corretamente?).

## Fase B — Implementação (TDD directo se for correção pontual na transição de fluxo; protocolo de
dois agentes se envolver mudança mais ampla no tratamento de `Content::Equation` inline)

1. Teste com os quatro casos reais da seção 8, confirmando o espaço presente e com a largura certa.
2. Implementar.
3. Suíte completa verde, discriminada por crate. Confirmar que espaçamento palavra-palavra comum
   não regride (guarda de não-regressão).
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir as 4 ocorrências de novo, mesmo método da auditoria. `compare.py` na seção 8.
2. Verificar se o mesmo padrão aparece em outras seções com matemática inline seguida de texto
   (não só a seção 8) — se sim, confirma que a correção tem alcance maior que o caso reportado.
3. Benchmark completo, 7 cenários, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta confirmada (transição de fluxo math-inline→texto descartando espaço).
- Espaço restaurado nos quatro casos e em qualquer outra ocorrência do mesmo padrão no documento.
- Benchmark sem regressão.
