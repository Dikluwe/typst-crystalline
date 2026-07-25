# Passo 906 — mecanismo de esticamento horizontal de glifo (para `underbrace`/`overbrace`, Parte C de P899 adiada)

**Precede este passo**: `typst-passo-899-relatorio.md`, secção "Parte C (ADIADA)". Ler antes de
começar — o diagnóstico do porquê já está feito, este passo é sobre desenhar e implementar o
mecanismo em falta.

**Escopo maior que a maioria dos passos recentes desta frente** — é um mecanismo novo, não uma
correção de bug existente. Tratar como passo de arquitetura, com o mesmo cuidado dado a P896
(gate de confirmação antes de mudanças estruturais).

**Pré-condição de árvore**: `git status`.

---

## O que falta (confirmado em P899, não redescobrir)

`Content::MathUnderover`/`layout_underover` (`engine/math/layout/mod.rs:588-634`, Passo 297) centra
o acento/chave no seu tamanho natural, sem esticar para cobrir a largura da base. O trait
`FontMetrics` já tem `vertical_glyph_assembly` (esticamento **vertical**, usado para delimitadores
altos como `(`/`[`/`{`) mas nada equivalente para esticamento **horizontal**.

## Fase A — desenhar antes de implementar

1. Confirmar como o vanilla implementa esticamento horizontal (`math/ir/resolve.rs`, mecanismo
   `Stretch`/`StretchInfo` já referenciado em P899) — ler o código real: usa a tabela
   `MathGlyphVariants`/`MathGlyphAssembly` da fonte (mesma família de dados que
   `vertical_glyph_assembly` já lê, mas para o eixo horizontal, se essa tabela suportar os dois
   eixos) ou um mecanismo totalmente diferente?
2. Confirmar se `NewCMMath` (a fonte usada pelo cristalino) tem dados de assembly horizontal para os
   glifos relevantes (chave de `underbrace`/`overbrace`, e os acentos largos como `hat`/`tilde`
   quando a base é larga) — via `fontTools`, mesmo método já usado nos passos anteriores desta
   frente. Se a fonte não tiver esses dados, o mecanismo não tem o que ler, e a implementação muda
   de "ler tabela da fonte" para "sintetizar esticamento" (repetir um segmento do glifo, ou usar um
   glifo diferente por faixa de largura) — confirmar qual dos dois casos é real antes de desenhar.
3. Desenhar o método novo no trait `FontMetrics` (paralelo a `vertical_glyph_assembly`) e como
   `layout_underover` o usaria — nome, assinatura, e como se integra com o código existente sem
   quebrar o caso já funcional (acentos de base curta, Parte A de P899).
4. Confirmar o alcance: só `underbrace`/`overbrace`/`underbracket`/`overbracket`, ou também acentos
   largos (`hat`/`tilde` sobre expressões, não só sobre uma variável) — o vanilla usa o mesmo
   mecanismo para os dois casos; decidir se este passo cobre só o caso que motivou (chaves) ou os
   dois, e registar a decisão.

**Gate**: se a Fase A concluir que isto exige mudança de assinatura de trait (provável, dado o
padrão de P893) ou edição de L0 estrutural, seguir o mesmo protocolo — editar os L0s necessários,
sincronizar hashes, e **parar para confirmação** antes da Fase B, mesmo padrão de P893/896.

## Fase B — Implementação (protocolo de dois agentes de P898 — é geometria nova, o caso de maior
risco desta categoria até agora)

1. Agente A escreve teste(s) a partir da especificação da Fase A (sem ver a implementação),
   confirma vermelho.
2. Agente B implementa.
3. Revisão do orquestrador antes de fechar — mesmo padrão de P898/901, testar pelo menos um caso não
   coberto pelos testes do Agente A (por exemplo, base muito larga vs base muito estreita, para
   confirmar que o esticamento não quebra em nenhum extremo).
4. Suíte completa verde, discriminada por crate.
5. Recompilar a secção 10 (acentos) e, se implementado, a secção correspondente a
   `underbrace`/`overbrace`/`underbracket`/`overbracket` (P899 deixou-as em fallback de texto
   literal nessas secções — confirmar quais secções do `.typ` de 30 secções as usam) e confirmar
   visualmente contra o vanilla.
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Relatório da Fase A com: mecanismo do vanilla confirmado, dados da fonte confirmados (ler tabela
  vs sintetizar), desenho do método novo no trait, decisão de alcance (só chaves ou também acentos
  largos), confirmação do dono se tocar L0 estrutural.
- Teste(s) novo(s), protocolo de dois agentes documentado.
- Confirmação visual comparando com o vanilla real.
- Benchmark completo.
- Se a Fase A concluir que o esforço é desproporcional ao ganho (por exemplo, a fonte não tem dados
  e sintetizar é muito complexo): registar isso explicitamente e decidir com o dono se vale manter o
  fallback de texto literal por mais tempo, em vez de forçar uma implementação frágil.
