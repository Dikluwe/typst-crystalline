# Passo 908 — `Content::Place`/`Content::Align` aninhado em sub-frame: correção geométrica completa

**Precede este passo**: `typst-passo-904-relatorio.md`, Item 2. Ler antes de começar — a mitigação
de segurança já está implementada (entradas pendentes órfãs são descartadas, não aplicadas ao item
errado); este passo é sobre a correção completa (posição certa), não sobre segurança (já resolvida).

**Escopo potencialmente grande** — P904 já concluiu que a correção completa exige propagar posição
de sub-frame pela hierarquia, com múltiplos callers usando convenções de coordenadas diferentes
(delta incremental em `layout_align`, alvo absoluto em `layout_place`, divergentes desde P897).
Tratar como passo de arquitetura, mesmo cuidado de P896/906.

**Pré-condição de árvore**: `git status`.

---

## Fase A — desenhar antes de implementar

1. Reler o diagnóstico de P904 (`layout_sub_frame`, `start_idx` relativo à lista local vs lista do
   pai) e confirmar se ainda é actual — outros passos desta frente (P905-907, se já aplicados) podem
   ter mexido em código adjacente.
2. Mapear todos os callers de `layout_sub_frame` e a convenção de coordenadas de cada um
   (confirmado parcialmente em P904: `layout_align` usa delta incremental, `layout_place` usa alvo
   absoluto — confirmar se há mais callers além destes dois, e a convenção de cada um).
3. Avaliar pelo menos duas abordagens de correção, com custo/risco explícito para cada uma:
   - **(a) Propagar posição do sub-frame para baixo**: passar a posição absoluta (ou o delta
     acumulado) do sub-frame como parâmetro adicional a `layout_sub_frame`/`layout_content`, para
     que qualquer `Align`/`Place` aninhado grave a entrada pendente já re-baseada para o referencial
     do frame raiz. Mais correto, mas toca a assinatura de `layout_sub_frame`, usada por múltiplos
     callers — mudança mecânica extensa, potencialmente um novo campo em L0.
   - **(b) Re-basear no ponto de composição**: manter `layout_sub_frame` como está, mas, no ponto
     onde o caller compõe os items do sub-frame no pai (onde já traduz `x`/`y` local para posição no
     pai), também traduzir as entradas pendentes que vieram com o sub-frame, ajustando `start_idx`/
     `origin_x`/`origin_y` para o referencial do pai antes de as anexar às listas `pending_*` do
     `Layouter`. Mais contido (não muda a assinatura de `layout_sub_frame`), mas duplica a lógica de
     tradução em cada caller (que já a tem, para os items normais — reaproveitar, não duplicar, se
     possível).
4. Escolher uma, com justificação registada. Se tocar L0 estrutural: parar para confirmação do dono,
   mesmo protocolo de P893/896/906.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria com múltiplos callers,
risco de regressão nos casos já corrigidos de P896-898)

1. Agente A escreve teste(s): caso mínimo de P904 (`#box[#place(bottom, dy: 0.3cm)[nested
   placed]]` sob `height: auto`) com posição exacta esperada (medida contra o vanilla real,
   `mutool trace`), mais um caso de `#box[#align(center)[...]]` aninhado (P897 corrigiu o caso não
   aninhado, confirmar que o aninhado também é coberto agora). Confirma vermelho.
2. Agente B implementa conforme o desenho escolhido na Fase A.
3. Revisão do orquestrador — mesmo padrão de P898/901/906: testar pelo menos um caso de aninhamento
   mais profundo (dois níveis de `#box`, não só um) não coberto pelos testes do Agente A.
4. Suíte completa verde, discriminada por crate. **Atenção especial**: os testes de P896/897/898
   (casos não aninhados) devem continuar a passar sem alteração — esta correção não pode regredir o
   caso simples ao generalizar para o caso aninhado.
5. Recompilar o `.typ` de 30 secções e confirmar sem regressão (mesmo que não exercite este caminho
   directamente).
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Relatório da Fase A com as abordagens avaliadas, escolha justificada, confirmação do dono se
  necessário.
- Teste(s) novo(s) cobrindo aninhamento simples e (pela revisão do orquestrador) aninhamento
  duplo.
- Confirmação de que os casos não aninhados (P896-898) continuam correctos.
- Benchmark completo.
- Se a Fase A concluir que o custo é desproporcional (por exemplo, exige reescrever a interface de
  `layout_sub_frame` de forma que afeta muitos outros consumidores não relacionados a alinhamento):
  registar isso explicitamente e manter a mitigação de segurança de P904 como estado aceite por
  mais tempo, em vez de forçar uma correção estrutural grande sem essa avaliação.
