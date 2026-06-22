# Passo 386 — Diagnóstico: as duas listas de "o que falta migrar" + inventário de prontidão i18n

**Tipo**: Diagnóstico (não materializa código de produção L1–L4).
**Data**: 2026-06-21.
**Padrão**: diagnóstico-primeiro; inventariar-primeiro (ADR-0065); medir-antes-de-decidir (ADR-0108).
**Passo fonte**: continuação directa do Passo 385 (decomposição do só-vanilla) — consome o resíduo (balde 3) que o 385 produziu.
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0052 (Lang tipo semântico), ADR-0054 (scope-out graded), ADR-0057 (hyphenation por língua), ADR-0107 (paridade é com a língua), ADR-0108 (medir antes de decidir), ADR-0110 (marcador `@vanilla`).

> **Dois eixos, um passo.** O eixo 1 (§A–§B) responde à pergunta "o que falta refatorar para o código novo" — dívida de feature. O eixo 2 (§C) inventaria a superfície de strings user-facing para prontidão de i18n. São axes distintos; estão no mesmo passo porque a varredura se sobrepõe (ambos percorrem os sítios user-facing). Se a granularidade pedir, o eixo 2 destaca-se como Passo 387 — decisão do dono.

> **Nota de numeração de ADR.** O eixo 2 propõe uma ADR de roadmap i18n. O número fica `ADR-NNNN` de propósito: varrer `00_nucleo/adr/` e usar o primeiro livre (ADR-0110 está ocupada; precedente P160A/P376 — nunca assumir número).

---

## Contexto

O Passo 385 decompôs o só-vanilla (10745) e mostrou que ~96% não é dívida de língua. O que **não** produziu — de propósito, por ser subtrativo — foi a lista positiva: o que ainda falta construir no cristalino. Essa lista tem duas fontes complementares:

- **Top-down (autorada):** as linhas `ausente` e `parcial` do Inventário 148. É a resposta direta, escrita por feature.
- **Bottom-up (rede de segurança):** o resíduo balde 3 do 385 (2339 itens), julgado item-a-item contra o Inventário 148 — pega o que o inventário porventura não listou.

A união das duas é "o que falta refatorar". Este passo extrai ambas.

Em paralelo, o dono fixou um objetivo de refatoração: deixar o cristalino **pronto para i18n completo** — mensagens de erro e outras strings user-facing localizáveis. O cristalino já tem subsistema de língua parcial (`rules/lang/`, `figure_supplement_for_lang`, `localize_quotes`, `state.lang`, tipo `Lang` per ADR-0052, hyphenation per ADR-0057). O que falta para i18n completo é, sobretudo, levar **erros e diagnósticos** (provavelmente literais espalhados) ao mesmo regime. O eixo 2 inventaria essa superfície e propõe o roadmap. Não implementa i18n.

---

## Objectivo

Produzir três listas e uma ADR proposta:

- **Lista A** — `ausente` + `parcial` do Inventário 148, por categoria, user-facing.
- **Lista B** — resíduo genuíno de língua (do balde 3 do 385) confirmado como dívida pelo cruzamento com o Inventário.
- **Lista C** — superfície de strings user-facing e o seu estado de prontidão para i18n.
- **ADR proposta** — roadmap i18n (centralização + Lang-lookup), com materialização diferida a passos dedicados.

Tudo diagnóstico. Zero código L1–L4 tocado.

---

## EIXO 1 — O que falta refatorar (dívida de feature)

### §A — Lista A: Inventário 148 (`ausente` + `parcial`)

Método (determinístico):

1. Ler `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` no HEAD actual.
2. Extrair todas as entradas de classe `ausente` e `parcial`, com a categoria (Model / Layout / Text / Introspection / Math / Foundations / Visualize / …) e a marca user-facing.
3. Para cada `parcial`, extrair a ressalva (o que falta para `implementado`), já registada na entrada do inventário.
4. Emitir tabela: categoria · feature · classe · ressalva (se parcial) · ADR/DEBT de roadmap associada (ADR-0060 Model, ADR-0061 Layout, ADR-0066 Introspection, DEBT-55, DEBT-56, …).

Saída: `00_nucleo/diagnosticos/typst-falta-migrar-lista-A-passo-386.md`.

### §B — Lista B: julgamento do resíduo (§4.3 do 385)

Método (script + julgamento):

1. Regerar o resíduo balde 3 do 385: `python3 lab/parity/tools/decompor_so_vanilla.py --lista-residuo` (2339 itens). Determinístico.
2. **Filtrar para candidatos de língua** (descartar mecânica per ADR-0107): manter `struct`/`enum`/`type`/`trait` de nível-de-língua e free-fns; descartar métodos inerentes, membros de impl de trait, tipos associados, e os módulos de execução já marcados no 385 (`typst_layout::*`, `typst_eval::{vm,call,flow}`, `typst_realize::*`, `typst_library::math::ir::*`, maquinaria de introspecção). Esta regra de filtro é **script**, com âncora escrita (ADR-0107).
3. Para cada candidato sobrevivente, **cruzar com o Inventário 148** (julgamento — Sonnet ou humano):
   - bate `implementado`/`implementado⁺` → **não é dívida** (mecanicamente divergente; chave K4 diferente — ex.: `cos`/`sin`/`abs` migrados em `make_calc_module`).
   - bate `scope-out` → fora.
   - bate `ausente` **e** é user-facing → **dívida confirmada** (sobe à Lista A se ainda não estiver).
   - sem correspondência no inventário → **lacuna do inventário**: feature vanilla que o inventário não cataloga. Registar como achado (o inventário precisa de uma entrada nova).
4. Emitir: contagem por veredicto + lista das dívidas confirmadas + lista das lacunas-do-inventário.

Saída: `00_nucleo/diagnosticos/typst-falta-migrar-lista-B-passo-386.md`.

> A Lista B existe para **refutar** ou **completar** a Lista A, não para duplicá-la. O valor dela são os dois extremos: dívida que o inventário já tem (confirma A) e dívida que o inventário **não** tem (corrige A).

---

## EIXO 2 — Prontidão para i18n (§C)

### §C.1 — Inventário da superfície de strings user-facing

Método (script + classificação Haiku):

1. Varrer L1/L2/L3 por **strings que chegam ao utilizador**:
   - **Mensagens de erro / diagnóstico** — sítios que constroem `SourceDiagnostic`/erro (validação stdlib: "named arg desconhecido", "year negativo", etc.).
   - **Supplements e rótulos de conteúdo** — "Figure"/"Table"/"Equation", "Bibliography", aspas, separadores, ordinais.
   - **Saídas formatadas** — `format_bib_entry`, placeholders de cite, etc.
2. Para cada sítio, classificar (Haiku propõe, humano confirma):
   - **(L) já lang-aware** — passa por `rules/lang/` ou por catálogo indexado por `Lang` (ex.: `figure_supplement_for_lang`, `localize_quotes`).
   - **(H) literal hardcoded** — string fixa no código, não-localizável.
   - **(N) não-user / interno** — `panic!`/debug/trace; fora de i18n.
3. Emitir tabela: sítio (ficheiro:linha) · categoria (erro/supplement/saída) · classe (L/H/N) · língua actual da string.

Saída: `00_nucleo/diagnosticos/typst-i18n-superficie-strings-passo-386.md`.

### §C.2 — O gap até "i18n completo"

A partir de §C.1, descrever o gap em três pontos, factuais:

1. Quantos sítios (H) precisam migrar para o regime (L).
2. Onde já existe infra reusável (o catálogo de texto L2; o módulo `rules/lang/`; o tipo `Lang`; o pattern `*_for_lang`).
3. Riscos de tradução: concatenação de strings, ordem de palavras fixa, pluralização, género — sítios onde o texto não é parametrizável de forma segura para tradução.

### §C.3 — ADR proposta: roadmap i18n

Propor `ADR-NNNN` (PROPOSTO; varrer número) fixando:

- **Decisão**: strings user-facing passam a um **catálogo indexado por chave + `Lang`** (estende o catálogo de texto L2 e o `rules/lang/` existentes); o sítio de código referencia uma chave, não um literal. Parametrização translation-safe (sem concatenação; placeholders nomeados).
- **Construir sobre o que existe**: `figure_supplement_for_lang`/`localize_quotes` são o precedente; o roadmap generaliza o pattern a erros e diagnósticos.
- **Roadmap de materialização (passos dedicados — NÃO neste passo)**:
  1. Centralizar os (H) de erro/diagnóstico no catálogo, mantendo a língua actual como default.
  2. Indexar o catálogo por `Lang`; semear pt/en primeiro (precedente: 6 línguas já em quotes/supplements).
  3. Trava de linter (nova `V<n>`): rejeitar literal user-facing novo fora do catálogo (análogo à trava de linhagem).
  4. Restantes línguas conforme prioridade.
- **DEBT** aberto para a materialização i18n (XL; rastreador).

> Observação literal: "i18n completo" inclui escolher a **língua das mensagens de erro do compilador**. Hoje o vanilla emite erros em inglês; o cristalino emite no que estiver hardcoded. A ADR deve declarar a política de default (provavelmente inglês, paridade vanilla) e separar a língua do **conteúdo do documento** (supplements, já lang-aware) da língua das **mensagens do compilador** (erros) — são dois eixos de `Lang` que podem ou não coincidir.

---

## O que produzir (resumo)

| # | Artefacto | Eixo |
|---|-----------|------|
| 1 | `typst-falta-migrar-lista-A-passo-386.md` | 1 |
| 2 | `typst-falta-migrar-lista-B-passo-386.md` | 1 |
| 3 | `typst-i18n-superficie-strings-passo-386.md` | 2 |
| 4 | `ADR-NNNN` roadmap i18n (PROPOSTO) | 2 |
| 5 | DEBT i18n (rastreador, EM ABERTO) | 2 |
| 6 | Extensão do script de extracção em `lab/` (reusa o do 385) | 1+2 |

---

## O que NÃO fazer (scope-out do passo)

- **Não** materializar feature alguma da Lista A/B. Isto é diagnóstico; a materialização é a série de passos que estes diagnósticos alimentam.
- **Não** centralizar string alguma nem tocar mensagens de erro. O eixo 2 inventaria e propõe; não materializa.
- **Não** escrever os checks de linter (i18n ou marcador). Posteriores.
- **Não** pôr LLM no loop de contagem. Contagem é script; LLM só no julgamento do resíduo (§B) e na classificação L/H/N (§C.1), sempre proposta a confirmar.
- **Não** tocar código de produção L1–L4.

---

## Critérios de aceitação

1. Lista A extraída integralmente do Inventário 148 (toda entrada `ausente`/`parcial`, com categoria e ressalva).
2. Lista B com veredicto por candidato; dívidas confirmadas e lacunas-do-inventário explícitas e separadas.
3. Reconciliação do eixo 1: todo candidato de língua do resíduo recebe exactamente um veredicto (nenhum item órfão).
4. Lista C cobre os sítios user-facing das três categorias (erro, supplement, saída), cada um classificado L/H/N com ficheiro:linha.
5. ADR i18n PROPOSTA, construída sobre `rules/lang/` + `Lang` existentes, com roadmap e política de default declarada.
6. Script reprodutível (eixo 1 e a extracção do eixo 2): duas corridas → output idêntico.
7. Zero ficheiros de produção L1–L4 alterados (diff vazio fora de `lab/`, `diagnosticos/`, `adr/`, `DEBT.md`).

---

## O que pode sair errado

- **Inventário 148 atrás do código (HEAD).** Uma entrada `ausente` pode já estar implementada e não reclassificada. Mitigação: para cada `ausente` da Lista A, conferir existência no código antes de a dar como dívida; divergências viram achado de "inventário desactualizado", não dívida falsa.
- **Lacunas-do-inventário confundidas com mecânica.** Um candidato do resíduo sem entrada no inventário pode ser mecânica que a regra de filtro não pegou. Mitigação: o filtro per ADR-0107 é conservador; o que passa e não bate o inventário vai para revisão, não direto para dívida.
- **Strings (N) classificadas como (H).** `panic!`/debug não são i18n. Mitigação: a classificação L/H/N é proposta (Haiku) e confirmada (humano); o linter futuro só trava onde o dono marcar user-facing.
- **Eixo 2 inflar o passo.** Se a superfície de strings for grande, §C vira Passo 387 dedicado. Decisão do dono no início da execução.

---

## Referências

- Passo 385 — decomposição do só-vanilla (fonte do resíduo).
- ADR-0110 — marcador `@vanilla` (o eixo 1 beneficia: balde 1 exato reduz o resíduo a julgar).
- ADR-0052 — `Lang` tipo semântico (base do eixo 2).
- ADR-0057 — hyphenation por língua (precedente lang-aware).
- ADR-0107 — paridade é com a língua (regra de filtro do §B).
- Inventário 148 — `typst-cobertura-vanilla-vs-cristalino.md` (fonte da Lista A).
- Infra lang existente: `rules/lang/quotes.rs`, `figure_supplement_for_lang`, `state.lang`, catálogo de texto L2.

---

## Nota sobre o Tekt

O eixo 1 fecha o ciclo aberto na origem da lente: "o que falta migrar" é resposta do **inventário autorado**, não da contagem mecânica — a lente mede símbolo, o inventário mede língua. Registar isto como confirmação empírica da lição de eixo (ADR-0107 + a separação inventário/lente).

O eixo 2 é candidato a um padrão Tekt mais geral: "prontidão para uma capacidade transversal (i18n) inventaria-se antes de materializar, como qualquer feature" — diagnóstico-primeiro aplicado a uma propriedade não-funcional. Registar a possibilidade; não materializar no Tekt neste passo.
