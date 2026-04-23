# Passo 108 — Análise de `Introspection`: inventário e decisão de sub-escopo

**Série**: 108 (passo de **análise**, não de construção).
**Precondição**: Passo 107 encerrado; DEBT-49 fechado; 803 L1 +
184 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0026 (divergência de Content), ADR-0033
(paridade funcional), ADR-0036, ADR-0037.
**ADR nova**: **não** neste passo. Este passo produz o
**enquadramento** para a ADR que virá no passo de construção
seguinte.

---

## Natureza deste passo

`Introspection` é estruturalmente maior do que qualquer materialização
feita nos passos 99–107. No vanilla, `Introspection` agrega:

- Contadores por tipo de elemento (heading, figure, equation).
- Localizações (`Location`) atribuídas a cada elemento.
- Queries (`query(heading)`, `counter(figure).final()`).
- Referências cruzadas (`@label`).
- Métadados para numeração hierárquica.

Materializar tudo de uma vez viola o padrão dos últimos passos
(um conceito de cada vez, âmbito estrito). Materializar
arbitrariamente uma parte corre o risco de escolher a parte
errada — parte bloqueada por outra, ou parte que não desbloqueia
nada de valor.

Este passo é **análise**: produz os factos necessários para, no
passo seguinte, escolher com base empírica o sub-escopo correcto
da primeira materialização de `Introspection`.

**Não produz código de produção.** Produz documentos de
diagnóstico.

---

## Objectivo

Ao fim deste passo, estarão disponíveis:

1. **Inventário do vanilla**: o que `Introspection` / `Introspector`
   / `Location` / `Counter` são realmente no vanilla; que
   dependências têm; que funcionalidades exposem.
2. **Inventário do cristalino**: o que já existe com nomes
   parecidos (`rules/introspect.rs`, `counter_state.rs`,
   `Content::Heading { level, body }`, `SetHeadingNumbering`,
   `SetFigureNumbering`). Como se relacionam com o vanilla.
3. **Mapa de dependências**: quem bloqueia quem dentro de
   `Introspection`.
4. **Candidatos a sub-escopo** para a primeira materialização,
   ranqueados por:
   - Pequeno (materializável num passo ≤ Passo 104 em tamanho).
   - Desbloqueia pelo menos um DEBT aberto.
   - Não exige dependências ainda não materializadas.
5. **Recomendação** para o Passo 109 de construção.

**Crítico**: o sub-escopo escolhido **não** é decidido neste
passo. A decisão fica para depois da análise, numa conversa
separada, com base nos factos recolhidos.

---

## Escopo

**Dentro**:
- Leitura de `lab/typst-original/` para entender
  `Introspection`/`Introspector`/`Counter`/`Location` do vanilla.
- Grep em `01_core/` e `03_infra/` para mapear o estado actual
  do cristalino.
- Produção de ficheiros de diagnóstico.

**Fora**:
- Qualquer código de produção.
- ADR nova.
- Alteração de ficheiros em `00_nucleo/` excepto
  `00_nucleo/diagnosticos/`.
- Testes novos.

---

## Sub-passos

### 108.A — Inventário do vanilla

Ler `lab/typst-original/crates/typst-library/src/introspection/`
e ficheiros relacionados. Para cada tipo principal:

**Parte 1 — Tipos centrais**:

Grep por `pub struct Introspection`, `pub struct Introspector`,
`pub struct Location`, `pub struct Counter`, `pub struct Query`
em `lab/typst-original/`. Para cada tipo encontrado:

- Ficheiro:linha.
- Campos públicos e privados.
- Métodos da impl pública.
- Dependências (imports externos, imports internos que apontam
  para tipos não-L1 no cristalino).

**Parte 2 — Funcionalidades**:

Identificar as funcionalidades expostas a utilizadores Typst:

- `counter(heading).get()` — como funciona?
- `counter(figure).final()` — como funciona?
- `query(heading)` — como funciona?
- `locate(x)` — como funciona?
- Numeração hierárquica (`1.1`, `1.2.3`) — como é implementada?
- Referências cruzadas (`@label`) — como chegam ao Introspector?

Para cada funcionalidade, registar:
- Tipos envolvidos (de 108.A.1).
- Onde o eval chama o Introspector.
- Onde o Introspector é construído.
- Quando a introspecção "acontece" (single pass vs. multi-pass
  layout; isto é importante).

**Parte 3 — Dependências entre tipos**:

Para cada tipo central, listar os outros tipos centrais de que
depende. Produzir grafo textual:

```
Location depende de: FileId, Span
Counter depende de: Location, Value
Introspector depende de: Location, Counter, Query
Introspection depende de: Introspector, [outros]
```

**Escrever em** `00_nucleo/diagnosticos/vanilla-introspection-passo-108.md`.

### 108.B — Inventário do cristalino

**Parte 1 — O que já existe**:

Grep em `01_core/src/` por nomes próximos de `Introspection`:

- `introspect`, `Introspector`, `Location`, `Counter`, `Query`.
- `CounterState`, `counter_state` (já existe segundo a ADR-0017).
- `Content::Heading`, `Content::SetHeadingNumbering`,
  `Content::SetFigureNumbering`.
- Funções `materialize_time`, `step_hierarchical`.

Para cada, registar:
- Ficheiro:linha.
- API pública.
- Relação com o nome/conceito equivalente no vanilla.
- É stub, implementação parcial, ou implementação completa?

**Parte 2 — DEBTs relacionados**:

Cruzar com o DEBT.md:

- DEBT-10 (resíduos do motor de introspecção) — o que está
  aberto?
- DEBT-45 (check_layout_depth não chamados em alguns pontos) —
  relação com Introspection.
- Outros DEBTs que referenciam `Introspection`, `Counter`,
  `location`.

**Parte 3 — Consumidores no cristalino**:

Onde é que `Content::Heading`, `Content::SetHeadingNumbering`,
etc. são consumidos hoje? O `rules/introspect.rs` processa-os.
Ler e documentar o que faz.

**Escrever em** `00_nucleo/diagnosticos/cristalino-introspection-passo-108.md`.

### 108.C — Mapa de dependências e lacuna

Produzir tabela de "precisa de / é precisado por":

| Conceito | Vanilla | Cristalino | Lacuna |
|----------|---------|------------|--------|
| `Location` | existe | stub/ausente? | ... |
| `Counter` | existe | `CounterState` parcial | ... |
| `Introspector` | existe | ausente? | ... |
| `Query` | existe | ausente? | ... |

Identificar o **caminho crítico**: se o objectivo fosse ter
numeração hierárquica de headings a funcionar, que tipos
mínimos têm de ser materializados? E para `query(heading)`? E
para `@label`?

**Escrever em**
`00_nucleo/diagnosticos/dependencias-introspection-passo-108.md`.

### 108.D — Candidatos a sub-escopo

Produzir lista ranqueada de 3-5 candidatos a primeira
materialização. Para cada:

- **Nome do candidato** (ex: "Location mínima", "Counter real",
  "Query read-only").
- **Tipos a materializar**.
- **DEBTs que desbloqueia** (ex: DEBT-10, DEBT-45, ou permite
  Heading colapsar em Styled).
- **Tamanho estimado** (comparar com passos anteriores:
  104 médio, 100 grande, 107 médio).
- **Dependências necessárias** (o que tem de já estar em L1
  para o candidato ser viável).
- **O que o candidato NÃO resolve**.

Pelo menos um candidato tem de qualificar como "passo ≤ 104 em
tamanho". Se nenhum qualifica, documentar explicitamente e
recomendar `Engine<'a>` com stub interno como alternativa.

**Escrever em**
`00_nucleo/diagnosticos/candidatos-introspection-passo-108.md`.

### 108.E — Recomendação

Resumo executivo num único ficheiro:
`typst-passo-108-relatorio.md`.

Conteúdo:

- Resumo do que é `Introspection` no vanilla (3-5 linhas).
- Resumo do que existe no cristalino (3-5 linhas).
- Gráfico de dependências (texto).
- Lista ranqueada de candidatos (de 108.D), com
  recomendação primária.
- Avisos sobre o que pode dar errado em cada candidato.
- Listagem dos ficheiros de diagnóstico produzidos
  (108.A-D).

O relatório **não** contém ADR. Serve como input para a conversa
onde a decisão de sub-escopo é tomada.

---

## Critério de conclusão

Todas em conjunto:

1. Ficheiros de diagnóstico escritos em
   `00_nucleo/diagnosticos/`:
   - `vanilla-introspection-passo-108.md`
   - `cristalino-introspection-passo-108.md`
   - `dependencias-introspection-passo-108.md`
   - `candidatos-introspection-passo-108.md`
2. Relatório `typst-passo-108-relatorio.md` escrito.
3. **Zero** mudanças a código de produção.
4. **Zero** ADRs novas.
5. **Zero** testes novos ou removidos.
6. `cargo test --workspace` com contagem inalterada (803 L1 +
   184 L3 + 6 ignorados).
7. `crystalline-lint` zero violations.
8. Pelo menos 3 candidatos a sub-escopo documentados, com 1
   recomendação primária.

---

## O que pode sair errado

- **Tentação de "já que estou a olhar, faço algo pequeno".**
  Resistir. Este passo não constrói. Se durante análise surgir
  uma correcção trivial (ex: DEBT-10 residual que já podia ter
  sido fechado), registar como **candidato** em 108.D, não
  executar.
- **Inventário do vanilla demasiado profundo.** O Introspection
  do vanilla tem centenas de linhas. Não transcrever tudo. Focar
  em: tipos centrais + 1 linha de descrição + dependências. Se
  um tipo tem 20 métodos, listar os 3-5 mais relevantes e dizer
  "mais N métodos para contadores / queries / ...".
- **Cristalino diverge bastante do vanilla.** Se o `rules/introspect.rs`
  actual faz algo que o vanilla não faz dessa forma (por decisão
  de ADR-0026 ou outra), documentar a divergência em 108.B; não
  a "corrigir" em análise.
- **Candidatos todos grandes.** Se nenhum candidato fica em tamanho
  ≤ Passo 104, isto é informação valiosa: significa que
  `Introspection` não se materializa por partes triviais. A
  recomendação em 108.E passa a ser "fazer Engine<'a> primeiro com
  Introspection stub interno" ou "dividir em sub-candidatos ainda
  menores" — registar a conclusão em vez de forçar.
- **Perda de escopo.** Se o executante começa a propor mudanças
  para `Engine<'a>`, `comemo` isolamento, ou outros trabalhos
  tangentes: fora do escopo. Registar em 108.E como "trabalho
  relacionado" no máximo.

---

## Notas operacionais

- Este passo não toca código. Se `cargo test` regride, algo foi
  tocado por engano.
- Ler o vanilla pelo menos para os ficheiros em
  `lab/typst-original/crates/typst-library/src/introspection/`.
  Não ler recursivamente tudo — focar em tipos centrais.
- Comparar com o que já existe em `rules/introspect.rs` é
  crítico: o passo anterior (103) activou `#show heading: ...`
  via este ficheiro. Qualquer materialização de `Introspection`
  tem de conviver com essa máquina.
- O DEBT-1 tem secção residual "propriedades adicionais
  bloqueadas por tipos não materializados". Algumas delas
  podem ser de Introspection (ex: `counter(heading).at(here())`).
  Registar em 108.B quais propriedades de Introspection estão
  referenciadas em DEBTs.
- Se o inventário revelar que `Introspection` já tem mais
  código do que o esperado (terceira vez em cinco passos que o
  cristalino está mais avançado que o relatório de continuidade
  sugere), documentar. Não tentar esconder.
