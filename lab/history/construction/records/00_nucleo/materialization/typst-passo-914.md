# Passo 914 — Achado de P911 em `attach.rs`: offsets de sub/sobrescrito são constantes fixas; vanilla computa valor adaptativo

**Precede este passo**: `typst-passo-911-relatorio.md`, "Achado — `attach.rs`". Ler antes de
começar. **Este é o maior escopo dos três achados de P911** — envolve um conceito ("cramped") que
o cristalino não tem hoje em modo matemático. Tratar como passo de arquitetura, mesmo cuidado de
P896/906/908.

**Pré-condição de árvore**: `git status`. Independente de `P912`/`P913` (área de código diferente,
`attach.rs` vs `stretchy.rs`/`assembly.rs`/`delimited.rs`/`matrix.rs`/`cases.rs`) — pode rodar em
paralelo.

---

## Achado (não redescobrir — já confirmado termo a termo, `file:line` dos dois lados no relatório)

`sup_offset`/`sub_offset` (`attach.rs:40-47`) são sempre a constante fixa do tipo de letra
(`constants.superscript_shift_up`/`subscript_shift_down`), independente da base ou dos scripts.
Vanilla (`scripts.rs:318-382`, `compute_script_shifts`) computa cada shift como o **máximo** de
3-4 termos (incluindo um termo "cramped", ausente do cristalino) e, quando sup e sub coexistem,
**ajusta os dois** para garantir um gap mínimo entre eles (linhas 363-379) — mecanismo totalmente
ausente hoje.

Kerning (`attach.rs:220-249`): usa só o kern da base, uma única altura de correção. Vanilla
(`scripts.rs:389-425`) soma o kern da base **com** o kern do próprio script, em duas alturas de
correção distintas, tomando o maior dos dois somados.

## Fase A — desenhar antes de implementar (escopo grande, decisão a registar)

1. **Confirmar o que "cramped" significa no vanilla** — não é campo de `attach.rs`, é estado de
   `EquationElem`/estilo propagado através do layout (`scripts.rs:325-330` só *consome*
   `EquationElem::cramped`, não o define). Localizar onde "cramped" é decidido no vanilla (tipicamente:
   dentro de um denominador de fração, dentro de certos delimitadores, ativa cramped para o conteúdo).
   Confirmar se introduzir isso no cristalino exige um campo novo em `TextStyle`/`MathStyle` (mudança
   de assinatura propagada, mesmo padrão de P893/P906) ou se pode ser um parâmetro adicional só em
   `layout_attach`/`compute_script_shifts`, sem propagar mais longe.
2. **Decidir o escopo deste passo**: implementar tudo de uma vez (cramped + extremos + gap
   simultâneo + kern de duas alturas), ou dividir em sub-passos (por exemplo: primeiro os extremos +
   gap simultâneo, que não exigem o conceito novo de "cramped"; depois "cramped" como passo
   separado, já que introduzir um conceito de estilo novo é mudança estrutural maior que ajustar uma
   fórmula). Registar a decisão explicitamente — não presumir "tudo de uma vez" só porque o achado
   veio junto.
3. Se decidir introduzir "cramped": confirmar todos os call sites que precisariam de propagar esse
   estado (frações, denominadores, outros contextos que o vanilla marca como cramped) antes de
   escrever código — mesmo padrão de inventário de P899/P901.
4. Ler a fórmula completa de `compute_script_shifts` e `math_kern` (scripts.rs) termo a termo,
   confirmando cada constante de `MathConstants` consumida (`superscript_shift_up`/`_cramped`,
   `sup_drop_max`, `sup_bottom_min`, `subscript_shift_down`, `sub_drop_min`/`sub_top_max`,
   `sub_superscript_gap_min`) — já devem existir na struct (confirmados em P893), só não são
   consumidos ainda por `attach.rs`.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria com múltiplos termos
interdependentes, o caso de maior risco desta frente inteira)

1. Agente A escreve testes com ground-truth calculado a partir da fórmula do vanilla lida na Fase A
   (não hardcoded), cobrindo pelo menos: sup sozinho, sub sozinho, sup+sub simultâneos com gap
   insuficiente (deve expandir), base com ascent/descent extremos, e — se dentro do escopo decidido
   — um caso cramped vs não-cramped. Confirma vermelho.
2. Agente B implementa conforme o desenho e escopo da Fase A.
3. Revisão do orquestrador — mesmo padrão de P898/901/906/908: testar pelo menos um caso composto
   não coberto pelos testes do Agente A (por exemplo, sup+sub simultâneos **e** base com descent
   extremo ao mesmo tempo).
4. Suíte completa verde, discriminada por crate.
5. Recompilar casos reais (`x^2_i`, bases com radicais/frações como script, scripts simultâneos
   próximos — secções 4/13/14/17 do `.typ` de 30 secções, que já usam sub/sobrescritos compostos) e
   confirmar visualmente/via `mutool trace` contra o vanilla real.
6. `cargo run -- .` — zero violations, e gate de L0/confirmação do dono se `TextStyle`/`MathStyle`
   ganhar campo novo (mesmo protocolo de P893).

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`. Atenção especial a `04-math` (usa muitos
sub/sobrescritos).

## Resultado esperado

- Decisão de escopo registada (tudo de uma vez ou dividido; se dividido, este passo cobre a parte
  decidida, resto fica para passo seguinte, nomeado explicitamente).
- Se "cramped" for introduzido: gate de confirmação do dono antes da Fase B, mesmo protocolo de
  P893/896/906.
- Testes novos com ground-truth derivado da fórmula real do vanilla.
- Confirmação visual/geométrica em casos compostos, não só isolados.
- Benchmark completo.
