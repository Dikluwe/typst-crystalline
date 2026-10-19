# Tarefa P335 — Lote F-2: o canal único das `Set*` (a chain léxica) + caronas de registro

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P335 (confirmar livre).
**Pré-condição**: F-1 (P334) fechado — fronteira no produto, suíte 2720,
lint 0/0, lente 219/3, perf par-ok. Se não, parar.
**Tipo**: caronas de registro (commit próprio, zero código) + **Lote F-2** —
o canal único das `Set*` como entradas na chain (fecha DEBT 99.E).
Content-preserving onde há comportamento fixado; paridade-de-linguagem onde
há lacuna declarada (B1).
**Fontes**: L0 `entities/f_fronteira_e1.md` §3b (a spec do lado estilo),
ADR-0106, `f-plano-lotes-passo-333.md` (F-2 + válvula), inventário 1a
(P331 — os 4 fluxos de ponta a ponta), rede de caracterização (+11, P331
Fase 2 — a spec de paridade), `medicao-pre-f-passo-330.md` (perf),
baseline lente (P333) + delta F-1 (P334).
**Commits**: "Passo 335 — caronas de registro" (1º, tree limpo) e
"Passo 335 — lote F-2".

---

## Caronas de registro (commit próprio; zero código)

1. **C1 — Protocolo de perf canônico**: o absoluto do P330 (0.6518 s) fica
   **superseded como absoluto** (deriva de ambiente comprovada no P334:
   0.7188 s same-machine entre sessões). Regra canônica, gravada em
   `medicao-pre-f-passo-330.md` (adendo) + no plano de lotes: a prova de
   não-regressão é o **par antes/depois back-to-back na mesma sessão**,
   mesmo corpus/método; números absolutos entre sessões não se comparam.
2. **C2 — DEBT do no-op de layout**: o arm `Content::Dynamic(_) => {}` em
   `layout_content` (A2/P334) é buraco comportamental **declarado e
   limitado** (só fixtures existem). Abrir DEBT curto com dono = **o lote
   de realização (F-3)**; o DEBT fecha quando a realização der layout real
   ao dinâmico. Comentário no arm aponta o DEBT.
3. **C3 — Emenda de sequência no plano de lotes** (decisão técnica
   registrada): F-2 = **só** o canal `Set*` (este lote); **F-3 novo =
   realização/`#show`** (S2–S6 + guards na camada `rules/` + o teste de
   transparência da condição Trava-Q1 + o fecho do DEBT C2); fila
   renumerada: `Styled`→F-4, de-bake→F-5, 3 folhas→F-6. Razão: faixa
   validada por lote e dependência real (o canal prova a chain léxica
   sobre a qual as recipes do `#show` montam).

---

## Lote F-2 — Fase A (reconhecimento) → checkpoint → Fase B (código)

### Fase A — plano de toque

1. Auditar o L0 §3b contra o código: a chain do Layouter hoje (10 nativas,
   `style_chain.rs`), o canal aberto (`PropKey→Value`) a nascer, e os **4
   fluxos atuais** das `Set*` (inventário 1a, reconferir por grep):
   `SetHeadingNumbering`/`SetEquationNumbering` via Introspector, `SetPage`
   mutando `page_config` (2 produtores — D4: `eval/rules.rs` vs
   `stdlib/layout.rs`; investigar qual é legacy), `SetFigureNumbering`
   assado em `Figure`.
2. **Decidir na Fase A (e registrar)**: D4 — se um produtor de `SetPage`
   for legacy, a remoção é dependência do desenho; **B1** — o produtor eval
   de `SetEquationNumbering` que falta (paridade com
   `#set math.equation(numbering:)`) nasce neste lote como **paridade de
   linguagem declarada**: testes novos cobrem o efeito; o teste de
   caracterização existente (`carac_set_equation_numbering_estado_atual`)
   **continua verde** (assertava só que o corpo renderiza).
3. **Checkpoint no chat**: plano de toque (arquivos × mudança), contagem
   prevista (~108 sites; largura reconferida por grep), o achado D4, e a
   **válvula**: se `SetPage` empurrar o lote acima do teto, fatiar em
   F-2a (`SetHeadingNumbering`+`SetFigureNumbering`+`SetEquationNumbering`,
   ~83) e F-2b (`SetPage`, ~8 + a mecânica `page_config`←chain) — propor
   com números e PARAR para a decisão.

### Fase B — o código (após o checkpoint)

Conforme o L0 §3b, na ordem:

1. **O canal aberto na chain**: `PropKey` (chave dinâmica) → `Value`
   (fechado, espelho da linguagem) ao lado das 10 nativas; resolução por
   fallback léxico instância→chain→default (C1+C3 do contrato).
2. **As 4 `Set*` viram entradas na chain** (uma por vez, validação
   intermediária build+suíte entre variantes — precedente L12): o efeito
   observável de cada uma é **idêntico** (a rede de caracterização +11 é a
   spec; nenhuma asserção alterada). Os 4 canais antigos colapsam no único;
   código morto dos canais antigos removido no mesmo movimento.
3. **B1**: o produtor eval de `SetEquationNumbering` (a lacuna) — com
   testes novos do efeito (numeração de equação aparece na saída como no
   vanilla; conferir o comportamento esperado na quarentena e registrar
   `file:line`).
4. **B3**: remover `world_types::Styles(())` (a chain real substitui o
   stub — confirmar que nada mais o referencia além do smoke-test dele).
5. **A trava ADR-0105 cláusula 3, lado estilo**: onde o canal aberto perde
   a exaustividade do compilador (chave dinâmica → valor), o
   teste-varre-tabela correspondente nasce ANTES (toda `PropKey` usada
   pelos consumidores resolve; tipo errado = erro declarado, não silêncio —
   a lição do downcast do E2).
6. Linhagem: `@prompt entities/f_fronteira_e1.md` nos arquivos novos;
   `--fix-hashes`.

**Não fazer**: realização/guards/`#show` (F-3); `Styled` (F-4); de-bake
(F-5); folhas (F-6); tocar os 65; alterar asserção existente (= bug).

### Verificação (gates do lote)

- `cargo build` limpo; suíte com `RUST_MIN_STACK=33554432` — **2720
  existentes verdes + N novos** (zero asserções alteradas; o carac de
  equação continua verde + ganha os testes do efeito novo).
- `crystalline-lint .` = 0 violations, 0 warnings.
- **Lente** `--comparar` antes/depois (R5 em par real): esperado —
  o colapso dos 4 canais **reduz** acoplamento (registrar os edges que
  somem); 65 independentes; ciclos não pioram.
- **Perf**: par back-to-back na mesma sessão (protocolo C1), mesmo corpus
  10×; sem regressão fora do ruído.

---

## Relatório (`typst-passo-335-relatorio.md` + resumo no chat)

- Caronas C1–C3 (onde moram; diffs).
- Fase A: os 4 fluxos confirmados; o achado D4 (qual produtor, o que foi
  feito); a válvula (rodou inteiro ou fatiou — com números).
- Fase B: diff por item; B1 (o efeito novo + testes + referência vanilla);
  B3 removido; a trava do canal aberto descrita.
- Gates: suíte (2720+N), lint, lente antes/depois (os edges que sumiram),
  perf par.
- **Contabilidade do F**: F-2 fechado (DEBT 99.E **fecha** — registrar);
  próximo **F-3 = realização/`#show`** (S2–S6, guards, transparência
  Trava-Q1, fecho do DEBT C2) — com o que a Fase A do F-3 deve reconhecer.
- `git log --oneline` (2 commits); `git status` limpo; caveat do stack.

## Fora de escopo (confirmado)

F-3+ (realização/`#show`, `Styled`, de-bake, folhas); B2 (→F-4 com
`Styled`); refinamentos da lente (sessão R4 em paralelo); otimizações
(perf é gate de não-regressão).
