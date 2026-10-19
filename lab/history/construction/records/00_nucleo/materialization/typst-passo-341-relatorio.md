# Passo 341 — relatório: spike de divergência eager-cascade vs fixpoint (medição)

> **Resultado:** o multi-passe/fixpoint **NÃO é preciso** para fidelidade
> comportamental. O eager-cascade é **fiel** nos casos centrais (incl. a cascata
> A→B, o discriminador central). Há **2 divergências reais, ambas estreitas** —
> nenhuma exige um fixpoint. Spike content-preserving: zero produto tocado, suíte
> 2719/3238 inalterada, lint 0/0.

## §0 — Por que este passo

O P340 assumiu que a F-realização precisa trocar o eager-cascade pelo fixpoint do
vanilla, e parou ao ver que isso colidia com 18 testes eager. Mas
content-preservation a bloquear o fixpoint **não prova que o fixpoint é preciso**.
O norte (P329) é fidelidade **comportamental** (mesma `.typ`, mesma saída), não
estrutural. Este passo **mede** a divergência observável entre os dois modelos,
contra o **vanilla compilado**. Só uma divergência medida justifica mudar o
comportamento dos nativos.

## Setup de medição (reprodutível)

- **Vanilla compilado:** `lab/typst-original` não tinha manifesto de workspace
  (renomeado p/ `Cargo.toml.original`, destacando-o do workspace cristalino).
  Para medir: `cp Cargo.toml.original Cargo.toml` com `members = ["crates/*"]`
  (os membros `docs`/`tests` não existem na quarentena); um fix compile-only de
  `E0282` em `crates/typst-library/src/foundations/str.rs:711`
  (`<EcoString as AsRef<str>>::as_ref(self).repr()` — zero mudança de
  comportamento, toolchain atual). `cargo build -p typst-cli` → `typst 0.14.2`.
  **Ambas as mudanças de lab foram revertidas** ao fim (tree do produto limpo); a
  reprodução fica aqui.
- **Cristalino:** `target/debug/typst` (bin do `04_wiring`).
- **Comparação:** `typst compile X.typ X.pdf` (vanilla) / `typst X.typ X.pdf`
  (crist) → `pdftotext` → texto normalizado. Saída **observável** (texto), não
  estrutura interna.

## Fase A — os testes eager por balde (`rules/eval/tests.rs`)

| Balde | Testes | Risco de divergir |
|-------|--------|-------------------|
| **(i) auto-reaplicação cortada por guard** | `show_rule_composicao_sem_loop` (2924), `show_rule_nao_recursiva_sem_stack_overflow` (2868) | baixo (guard existe nos dois) |
| **(ii) interação multi-regra (A→B)** | `show_rule_encadeamento_duas_regras` (2941), `show_rule_encadeamento_texto_sequencial` (2913) | **a pergunta real** |
| **(ii-disjunto) multi-regra alvos disjuntos** | `show_rule_multiplas_regras_nodekind_travessia_unica` (2968), `show_rule_multiplos_tipos_independentes` (2892) | baixo (sem interação) |
| **(iii) outra** (regra única / erro / escopo) | `eval_show_rule_text_substitui_ocorrencias`, `eval_show_rule_funcao_no_heading`, `show_rule_resolve_por_identidade_nao_por_nome`, `show_rule_map_content_transversal`, `show_rule_texto_usa_map_text_nao_map_content`, `show_rule_closure_anonima_rejeitada`, `eval_show_rule_falha_explicita_tipo_retorno_invalido`, `show_rule_active_guards_limpos_apos_erro`, `show_rule_respeita_escopo_lexico` | nenhum (não-interação) |

**Leitura:** só **2** testes exercitam interação multi-regra A→B genuína; nenhum
testa precedência de múltiplas regras no **mesmo** nó nem recursão patológica.

## Fase B — o mapa de divergência (vanilla compilado vs cristalino)

| # | `.typ` (padrão) | vanilla | cristalino | veredito | causa medida |
|---|-----------------|---------|------------|----------|--------------|
| p1 | `#show heading: it => upper(it.body)` · `= capitulo` | `CAPITULO` | `CAPITULO` | **PARIDADE** | — |
| p3b | `#show heading: it => emph[IN]` + `#show emph: it => [WRAP]` · `= z` | `WRAP` | `WRAP` | **PARIDADE** | **cascata-por-criação ≡ fixpoint** (o discriminador central: B aplica-se ao output de A nos dois) |
| p5 | `#show "A":"B"` + `#show "B":"C"` · `A` | `C` | `C` | **PARIDADE** | cadeia de text rules igual |
| p6 | `#show heading: it=>[A]+it.body` + `#show heading: it=>[B]+it.body` · `= z` | `Bz` (última) | `A z` (primeira) | **DIVERGE** | **precedência**: vanilla aplica a **última-declarada** (innermost-first); crist a **primeira** |
| p4 | `#show heading: it => [= X]` · `= A` | **ERRO** `maximum show rule depth exceeded` | `X` | **DIVERGE** | **recursão**: guard `RuleId` (crist) corta a reaplicação; vanilla recursa até o teto-64 e erra |
| p7 | `#show strong: it=>emph[S]` + `#show emph: it=>strong[E]` · `*x*` | **ERRO** `maximum show rule depth exceeded` | `E` | **DIVERGE** | recursão **mútua** — igual p4 |
| p8 | `#show heading: set text(weight:"bold")` · `= z` | `z` | **ERRO** `selector de tipo requer função ou Content, recebeu none` | **N/A** | show-set não implementado → **fatia 3 (P342)** |

**Incidentais (NÃO multi-passe, separados):**
- **Whitespace de baseline:** `A C` **sem** `#show` → vanilla `AC`, crist `A C`
  (`pbase.typ`). É diferença de layout/extração de texto, independente de `#show`
  (afeta também p2: `BD` vs `B D`). Não é divergência de show rule.
- **`.body` em `emph`:** a 1ª versão de p3 (`#show emph: it => […] + it.body`)
  errou no crist (`campo 'body' não existe`) — gap de **leitura de campo (S7)**
  no caminho `emph`/`Styled`, não a cascata. Registrado à parte.

## Recomendação (para o checkpoint do dono)

**O multi-passe/fixpoint NÃO é preciso para fidelidade.** O eager-cascade já é
fiel onde importa: regra única (p1), **cascata A→B (p3b — paridade)**, cadeias de
texto (p5). Desfecho **2** da TRAVA (divergências existem, mas estreitas):

1. **Precedência (p6) — conserto mais estreito = ordenação, não fixpoint.**
   Múltiplas regras no mesmo nó: alinhar a ordem de iteração em `apply_show_rules`
   (`rules.rs:97-167`) para **última-declarada-primeiro** (innermost-first), em vez
   da atual primeira-primeiro. É uma mudança de **ordem**, não um loop. Risco
   content-preserving: **nenhum** dos 18 testes fixa "primeira-vence" (p6 não está
   entre eles) — a confirmar no lote de execução.

2. **Recursão (p4/p7) — política, não fixpoint.** Em input **patológico**
   (auto/mútua recursão), o vanilla **erra** com diagnóstico; o guard `RuleId` do
   crist **corta e produz saída**. Decisão do dono: (a) manter o crist (mais
   gracioso — termina com saída) e **documentar** a divergência como intencional;
   ou (b) alinhar ao erro do vanilla (emitir "maximum show rule depth exceeded"
   quando o guard cortaria uma regra que o vanilla deixaria recursar). Nenhuma das
   opções é um fixpoint.

3. **Show-set (p8) → fatia 3 (P342).** Confirmado não implementado; fora deste
   escopo.

**Consequências propostas:**
- A **§3a.7-bis** (desenho do fixpoint do vanilla) fica como **referência
  registrada, não obrigação** — a medição mostra que o mecanismo não é preciso.
- Os **18 testes ficam** (estão certos; nenhum testa os padrões divergentes).
- A **"fatia 2 multi-passe" encerra**: o caso 4 (P340) foi o trabalho real dela;
  as 2 divergências viram, se o dono quiser, **2 consertos pontuais** (ordenação +
  política de recursão), cada um medido contra o vanilla compilado, com os testes
  do balde (ii) evoluídos **um a um** (saída do vanilla colada). A fila segue para
  a **fatia 3 (show-set, P342)** e **F-5**.

## Item aberto carregado — `content→elementos → 0`

Continua **fora da fila** e **sem dono**. O baseline da lente mede
`content→elements = 66` e espera `target = 0`, que nenhum lote entrega. Três
saídas (decisão, não bloqueio): **reconciliar o baseline** (modelo D tem
`≠ 0` por desenho) / **nomear marco pós-F-6** / **registrar lacuna** do plano.

## Verificação (gates)

```
content-preserving: zero código de produção, zero teste alterado, zero ficheiro
  de produto tocado (lab revertido). Suíte inalterada: 2719 (typst-core --lib)
  / 3238 (workspace).
lint: crystalline-lint . = 0 violations, 0 warnings.
lente: não medida (nada de produto mudou) — declarada inalterada vs P340
  (219 | 676 | [90,4] | 66 | 0).
medição reproduzível: comandos e fixes de build registrados acima; vanilla
  typst 0.14.2 (515e46e7).
```

Nenhum commit de código; o entregável é este mapa + recomendação, para decisão.
