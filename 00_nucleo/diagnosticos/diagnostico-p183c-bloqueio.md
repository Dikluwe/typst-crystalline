# Diagnóstico — P183C bloqueio (C2 equation counter)

**Data**: 2026-05-03
**Passo**: P183C — auditoria semântica explícita antes de migrar
**Escopo**: tentativa de migração do consumer C2 (`Layouter::layout_equation`
em `01_core/src/rules/layout/equation.rs:97`) de
`self.counter.get_flat("equation")` legacy para
`self.introspector.flat_counter("equation")` com fallback.
**Postura**: zero código tocado em L1–L4; zero testes modificados; zero L0
modificado; produzir decisão de gate + plano de escalada.

---

## §1 Premissa testada

> `flat_counter` Introspector tem paridade semântica com `get_flat` legacy
> para a chave `"equation"`, suficiente para suportar substitution-with-fallback
> (padrão P168/P181G/P182D) sem regressões observáveis.

---

## §2 Resultado

**Falsa em duas dimensões independentes**, ambas suficientes para gate
substancial.

### §2.1 Asimetria temporal — snapshot-during-walk vs snapshot-final

Mesma natureza descoberta em P183B para C1 (heading prefix). O Layouter
constrói o `CounterStateLegacy.flat` **incrementalmente durante o walk de
layout**: `Layouter::new` inicializa `flat` vazio
(`01_core/src/rules/layout/mod.rs:150`) e o copy-site
`layout_with_introspector` em `mod.rs:1428` confirma explicitamente:

```rust
// NÃO copiar hierarchical, flat — reconstruídos nó a nó.
```

O arm `Content::Equation` em `equation.rs:35` faz `step_flat("equation")`
**inline** antes de `equation.rs:97` ler `get_flat("equation")`. Logo,
para a equation N, `get_flat` devolve `N` — o valor **do momento**.

O Introspector, em contraste, é **um snapshot pós-walk**: `from_tags`
processa o walk inteiro antes do layout começar. Mesmo que existisse
arm `Equation`, `CounterRegistry.value("equation")` devolveria sempre
`[N_total]` (final). `value_at(key, location)` resolveria isto **se** o
Layouter conhecesse a sua própria `Location` no momento da consulta —
mas não conhece (causa raíz documentada em P183B).

### §2.2 Ausência de arm `Equation` em `from_tags`

Segunda dimensão, descoberta agora em `.B` de P183C (não foi descoberta
em P183B porque heading **tem** arm em `from_tags`, line 51–70 de
`01_core/src/rules/introspect/from_tags.rs`).

Inspecção empírica em `01_core/src/entities/element_payload.rs`: o enum
`ElementPayload` **não tem variant `Equation`**. Inspecção em
`01_core/src/rules/introspect.rs:377–382`: o walk de introspect chama
`state.step_flat("equation")` no **state legacy**, mas **não emite Tag**
para a equation. Resultado: `from_tags` nunca recebe um Tag de equation
e portanto **nunca chama `apply` no `CounterRegistry` para a chave
`"equation"`**.

Corolário: mesmo que adicionasse o método trait `flat_counter("equation")`
delegando a `CounterRegistry.value("equation")`, a resposta seria
**sempre `None`** — registry vazio para esta key.

Sem alteração da pipeline de introspect (emissão de Tag para equation
em `introspect.rs` + variant `ElementPayload::Equation` + arm em
`from_tags`), `flat_counter` Introspector não pode sequer ter dados.

---

## §3 Categoria confirmada

> **Consumers que precisam de valor durante o walk, não do final.**

Categoria definida originalmente em P183B (C1 heading prefix). P183C
confirma a mesma categoria para C2 (equation counter) e adiciona uma
sub-categoria: **C2 não tem sequer dados no Introspector** (lacuna de
emissão de Tag, agravada à asimetria semântica).

Inferência por extensão para os restantes consumers C1–C5 (refino do
quadro P183A §1 e §3):

| Consumer | Site | Lê durante walk? | Tem Tag? | Estado |
|----------|------|------------------|----------|--------|
| C1 heading prefix | `mod.rs:310` (`format_hierarchical`) | sim | sim (Heading) | bloqueado (P183B) — snapshot-final |
| C2 equation counter | `equation.rs:97` (`get_flat`) | sim | **não** (sem arm) | **bloqueado (P183C)** — snapshot-final + sem dados |
| C3 figure auto-number per kind | `mod.rs:435–439` (`figure_numbers[kind][idx]`) | sim | sim (Figure) | provavelmente bloqueado (mesma natureza C1; auditar em P183D) |
| C4 resolved label | `references.rs:53` (`resolved_labels`) | n/a — labels são identidade | sim (Heading.label, Figure.label) | provavelmente OK (P183E) |
| C5 TOC entries | `outline.rs:24` (`headings_for_toc`) | n/a (separado) | n/a | bloqueado por lacuna #3 separada |

---

## §4 Implicação para a série P183

P183 originalmente desenhada como série de migrações S × 4 (B/C/D/E).
P183A diagnóstico passou. P183B falhou (C1). P183C falha (C2). C3
provavelmente falha pela mesma razão. C4 é o único candidato com chance
real de prosseguir.

**Recomendação para o humano** (decisão fora deste diagnóstico):

- **Saltar** P183D (C3 figure) — categoria confirmada bloqueada; auditoria
  só serve para assento documental.
- **Executar** P183E (C4 resolved label) — categoria diferente (identidade
  de labels, não contadores incrementais); plausivelmente migrável.
- **P183F** abre DEBT M4-residual cobrindo **C1 + C2 + C3** simultaneamente
  (em vez de cada um abrir DEBT individual).

OU, se o humano preferir confirmar empiricamente C3:

- Executar P183D (com a mesma estrutura `.A`–`.B`–`.G`/`.C`–`.H`); esperar
  bloqueio em `.B` por mesma categoria.
- Depois P183E + P183F com DEBT triplo.

A primeira opção poupa um passo redundante (~30 min de auditoria sem
ganho informacional novo). A segunda opção mantém rigor empírico.

---

## §5 Solução estrutural (M6+ ou M+)

A categoria "consumers que precisam de valor durante o walk" só desbloqueia
quando duas condições se reúnem:

1. **Introspector exporta `flat_counter_at(key, location)`** (já existe a
   primitiva `value_at` no `CounterRegistry`; falta o método trait).
2. **Layouter conhece a sua `Location`** no ponto da consulta — i.e., o
   walk de layout produz `Location`s sincronizadas com o walk de
   introspect, ou o Layouter recebe a `Location` actual via parâmetro
   propagado.

Ambas são trabalho substancial (M6 ou superior — pendência paralela
P182E §5.2 já existente). Desbloqueia C1, C2 e C3 simultaneamente.

Para C2 especificamente, o **prerequisito adicional** é a emissão de
Tag `Equation` no walk de introspect — variant `ElementPayload::Equation`
+ arm em `from_tags` chamando `apply_at("equation", Step, loc)`. Este
trabalho é independente do Layouter location-aware e pode ser feito
isoladamente quando se decidir avançar.

---

## §6 Estado pós-P183C

- **Código produção**: zero linhas tocadas em L1–L4. Linter inalterado.
- **Tests**: workspace baseline P183A mantido (1.756 verdes; zero
  violations). `.A` e `.B` foram apenas leituras.
- **L0**: zero prompts modificados. `entities/introspector.md` continua
  sem `flat_counter` (não foi adicionado).
- **DEBT**: P183C **não abre DEBT formalmente**. DEBT M4-residual será
  aberto em P183F (passo de fecho da série) acumulando C1 + C2 + (após
  P183D ou por inferência) C3.

---

## §7 Aprendizado meta

P183B ensinou: **paridade semântica entre Introspector e legacy é falsa
por defeito para contadores**. P183C ratifica e estende: **a falsidade
pode ser dupla** — semântica errada *e* dados ausentes. A auditoria
explícita `.B` antes de qualquer migração é o que evita regressão de
testes (P183B) ou compilação inválida (P183C, caso tivesse adicionado
o método sem verificar emissão de Tag).

A regra operacional consolidada para passos M4-residual e seguintes:

> Antes de migrar consumer de contador para Introspector,
> auditar empiricamente em **dois eixos**:
> 1. **Existem dados** no sub-store correspondente para a chave?
>    (Verificar arm correspondente em `from_tags`.)
> 2. **A semântica temporal coincide**? (Snapshot-final do
>    Introspector vs valor mutável durante walk no legacy.)
>
> Se qualquer dos dois falhar, declarar gate substancial e escalar
> para DEBT — não tentar substitution-with-fallback.
