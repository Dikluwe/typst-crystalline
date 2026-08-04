# Passo 959 — espaçamento entre operador grande (∫,∮,∑,∏,⋃,⋂) e seus limites, 2-6× mais largo que o vanilla

**Precede este passo**: achado novo, auditoria externa (2026-08-04) — distância vertical entre o
símbolo do operador e o limite superior/inferior medida nas 38 ocorrências dos dois documentos
(mesma contagem, mesma ordem): vanilla quase sempre 1.9-3.8pt; cristalino 6.3-24.0pt. Padrão
sistemático, reproduzido nas 38 ocorrências, não pontual. **Espaçamento interno de matriz testado
em paralelo e confirmado idêntico nos dois lados** — a divergência é isolada aos limites de
operador grande, não é um problema geral de espaçamento.

**Conexão com achado já catalogado**: P944 §4 já tinha registado, como scope-out em `attach.rs`,
os termos `lower_limit_baseline_drop_min`/`upper_limit_baseline_rise_min` da tabela MATH como não
lidos/aplicados — candidato forte a ser a causa raiz deste espaçamento.

**Pré-condição de árvore**: `git status`. Confirmar P958 (se já executado) presente.

---

## Fase A — confirmar a causa

1. Ler a fórmula real do vanilla para posicionamento de limites acima/abaixo de operador grande
   (`typst-layout/src/math/`, provavelmente `underover.rs`/`limits.rs` ou equivalente) — confirmar
   os termos exatos consumidos da tabela MATH (`upper_limit_gap_min`, `upper_limit_baseline_rise_
   min`, `lower_limit_gap_min`, `lower_limit_baseline_drop_min` — nomes a confirmar, não presumir).
2. Confirmar a implementação atual do cristalino (`attach.rs`/`underover.rs`, o caminho que trata
   `Content::MathAttach` com `limits: true`) e confirmar se os termos que P944 já tinha registado
   como ausentes são de facto a causa, ou se há outra fórmula em jogo.
3. Medir os valores reais destes termos na fonte (`NewCMMath-Regular.otf`, `fontTools`) para ter o
   alvo numérico exato antes de implementar.
4. Confirmar se a variação observada (6.3pt a 24.0pt, não um valor fixo) é explicada por a fórmula
   correta ter mais de um termo (mesma família de descoberta de P952 — cinco causas somadas) ou se
   é só um termo com escala diferente por tamanho de fonte/estilo.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria, afeta todo operador
grande com limites no documento)

1. Agente A escreve testes com o valor esperado derivado da fórmula real do vanilla e dos termos
   medidos na fonte — cobrir pelo menos `∑`/`∫` com limite abaixo e com limite acima+abaixo.
2. Agente B implementa.
3. Revisão do orquestrador — testar um caso com o operador em tamanho `Display` vs `Text` (a
   fórmula pode escalar diferente conforme o nível, mesma família de cuidado que P945/952 já
   tiveram com `MathSize`).
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir as 38 ocorrências de novo (mesmo método da auditoria externa) e confirmar que o
   cristalino cai para a banda 1.9-3.8pt do vanilla.
2. `compare.py` no documento de 30 secções — confirmar melhoria nas secções com somatórios/
   integrais (4, 15, 25, 29, entre outras).
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta confirmada (termos de `MathConstants` para limites de operador grande, ou outra),
  com valores reais da fonte, não estimados.
- Espaçamento das 38 ocorrências dentro da banda do vanilla.
- Benchmark completo, zero regressão.
