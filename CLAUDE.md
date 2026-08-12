# CLAUDE.md — typst-crystalline

Este ficheiro é a diretriz suprema para o assistente de IA neste repositório.
Para decisões arquiteturais específicas: **ler os ADRs em `00_nucleo/adr/`**.

---

## Terminologia Crítica: Passos de Execução vs Prompts L0

Para evitar a corrupção da arquitetura por escrita de código não especificado, é obrigatório distinguir estas duas entidades:

- **Passo de Execução** (ex: `typst-passo-59.md` na raiz): documento tático, logístico e temporário, usado para coordenar tarefas imediatas, depurar erros ou planear a sessão entre o humano e a IA. **Não é o L0.**
- **Prompt L0** (ex: `00_nucleo/prompts/compiler/layout.md`): especificação arquitetural pura, perene e definitiva do sistema. **Este é o L0 — a única fonte da verdade que legitima o código.**

**Regra de Ouro (A Trava Arquitetural):**
O assistente **nunca** pode instruir a escrita de código L1/L2/L3 se o Prompt L0 correspondente não existir em `00_nucleo/prompts/` ou estiver desatualizado.

O fluxo de trabalho imutável é:

```text
1. Passo planeia as tarefas → 2. IA redige o Prompt L0 → 3. Humano guarda L0 e calcula Hash → 4. IA escreve Código (L1)
```

**Quando parar para confirmação (ADR-0127):** a paragem no ponto 3 é
obrigatória **só** para (1) mudança de contrato público (campo em entidade,
método em trait, assinatura pública), (2) mudança de comportamento por
defeito do produto (novo modo/flag/caminho padrão), (3) mudança de fase do
pipeline (eval ↔ layout), ou (4) quebra de compatibilidade. Correções de
fórmula interna, entradas em tabelas de mapeamento e correções de paridade
com o vanilla seguem em **fluxo contínuo** (L0 editado primeiro + resselo
de hash, sem paragem — o gate é o teste RED→GREEN + revalidação). Em caso
de dúvida sobre a classe: parar. Ver **ADR-0127**.

---

## ⚠️ Restrição de leitura — pastas de materialização e context

```text
00_nucleo/materialization/
00_nucleo/context/
```

- Não ler estas pastas por iniciativa própria.
- Só aceder quando explicitamente indicado com o path completo.
- Nunca varrer ou listar o conteúdo destas pastas.
- Se uma tarefa parece exigir este contexto mas não referencia um ficheiro explícito: **perguntar antes de agir**.

**Motivo:** estes ficheiros contêm instruções sequenciais históricas. Lê-los fora de contexto injeta estado passado e gera alucinações arquiteturais.

---

## A Arquitetura Cristalina (Tekt)

O código original do compilador está em `lab/typst-original/` (quarentena). A migração acontece gradualmente para as camadas cristalinas. O critério de sucesso primário é `crystalline-lint .` com zero violations.

### Camadas e Topologia de Imports

| Camada | Diretório | Regras de Importação e Propósito |
|--------|-----------|----------------------------------|
| L0 | `00_nucleo/prompts/` | Especificações. Não é código. A origem da linhagem. |
| L1 | `01_core/` | Domínio puro. Zero I/O. Só importa stdlib pura e whitelist. |
| L2 | `02_shell/` | CLI, formatadores. Conhece apenas L1. |
| L3 | `03_infra/` | I/O, filesystem, fontes. Conhece apenas L1. |
| L4 | `04_wiring/` | Composição. Conhece L1, L2, L3. Zero lógica de negócio. |
| lab | `lab/` | Quarentena. Nunca importado por L1–L4. |

---

## Organização das pastas de `00_nucleo`

`00_nucleo/` contém **documentação de processo** e **prompts arquiteturais**. A separação é rígida:

| Pasta | Propósito | Exemplo |
|-------|-----------|---------|
| `adr/` | Decisões arquiteturais formais (ADRs). | `adr/ADR-0109.md` |
| `context/` | Materialização sequencial — **não ler sem path explícito**. | `context/passo-146.md` |
| `diagnosticos/` | Análises, inventários, varreduras, métricas e documentos de estado. | `diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` |
| `debt-anexos/` | Anexos técnicos de débito arquitetural. | `debt-anexos/DEBT-001.md` |
| `materialization/` | Rascunhos de materialização — **não ler sem path explícito**. | `materialization/passo-423.md` |
| `prompts/` | **Prompts L0 vinculados a código L1–L4**. Especificações arquiteturais puras e perenes que legitimam código. | `prompts/compiler/layout.md` |

**Regra de organização:**

- `prompts/` é reservado a especificações L0 que têm correspondência direta com código produzido nas camadas L1–L4.
- `prompts/` **não** recebe documentos de processo, relatórios de varredura, análises de estado, métricas de saúde nem diagnósticos. Esses documentos ficam em `diagnosticos/`.
- Se um documento descreve *o que deve ser implementado* e legitima código, ele é **prompt L0** e vai para `prompts/`.
- Se um documento descreve *o que foi feito*, *como está o repositório*, *métricas* ou *estado atual*, ele é **diagnóstico** e vai para `diagnosticos/`.
- Documentos de análise de estado (ex: `p426-analise-estado.md`) e varreduras mecânicas (ex: `p425-varredura-mecanica.md`) são diagnósticos, não prompts.

### Restrições absolutas do L1

**Nunca:**

- Lê ou escreve ficheiros (`std::fs`), faz chamadas de rede, acede ao relógio (`SystemTime`) ou variáveis de ambiente (`std::env`).
- Tem estado global mutável (`static mut`, `static Mutex<T>`, `static OnceLock<T>`).
- Importa crates externas não declaradas em `[l1_allowed_external]`.

**Permitido** (performance de RAM não é I/O):

- `Arc<T>` em campos de struct (ADR-0029), `Vec`, `Box`, `String`, `HashMap`.
- `EcoString` (ADR-0024), `rustc-hash` / `FxHashMap` (ADR-0018).

---

## Paridade — com a linguagem, não com a mecânica (ADR-0107)

A paridade é com a **linguagem** Typst — **semântica, sintaxe, morfologia** — **nunca**
com a mecânica de execução ou a igualdade restrita do Rust. A implementação (estrutura
de dados, `PartialEq` do Rust, bytes de saída, passos do algoritmo) **diverge de
propósito** (P329). Ao medir paridade ou escrever critério de aceitação, usar os três
níveis da linguagem, não a mecânica. **Morfologia** = a forma do conteúdo enquanto
objeto da linguagem (texto, markup, estilo semântico `*bold*`), distinta do estilo de
**render** assado/derivado. Ver **ADR-0107**.

---

## Disciplina anti-deriva — medir antes de decidir (ADR-0108)

Todo prompt **mede antes de decidir**: a secção de decisão/classificação é **precedida**
pela medição (`file:line`) que a produz — nunca o contrário. Classifica **língua vs
mecânica** da fonte (semântica/sintaxe/morfologia = paridade; igualdade do Rust/bytes/
passos/estrutura = diverge, ADR-0107); distingue **intenção de comportamento** (não inferir
intenção do comportamento); **marca inferência** e o que a refutaria; **desconfia do
enquadramento lisonjeiro/cômodo** (checagem extra da fonte antes de aceitar). **Aceitação no
nível da língua**, nunca mecânica — exceto onde a mecânica **é** o observável (mensagem de
erro). Um prompt que viole a forma é **malformado**; o dono **audita a substância** (reduz a
dependência do freio, não a elimina). Ver **ADR-0108**.

---

## Regra — registar a proveniência de cada medição

**Data:** 2026-07-05  
**Aplica-se a:** todo o projecto, todos os passos que produzem um número usado para decidir algo.

### O que aconteceu

P569 registou "4 linhas" para um documento de teste. P574 e P575, mais tarde, tentaram reproduzir esse número a partir do commit de P569, e obtiveram "2 linhas" — nos dois casos, com e sem o código órfão de L1 que se pensava ser a causa. O número de P569 não foi reproduzido, e a causa real ficou sem explicação.

O problema não é o número em si. É que não há registo de **o que exactamente gerou esse número** — qual o estado do código no momento exacto da medição, se havia alterações não commitadas nessa altura, ou qualquer outra coisa que distinga esse momento do commit final que ficou no histórico.

### Regra

Qualquer número usado num relatório para decidir se algo está fechado ou aberto (contagem de páginas, de linhas, de palavras, posições, tempos de execução) tem de vir acompanhado de:

1. **O hash do commit** em que o teste foi corrido, ou "working tree não commitado" se for o caso, com a lista exacta de ficheiros alterados nesse momento (`git diff HEAD --stat`).
2. **A hora exacta**, se houver razão para pensar que o estado pode ter mudado entre uma medição e outra no mesmo passo.

Isto não substitui as regras já escritas (decisão nova obrigatória; disciplina de verificação). Complementa-as: aquelas dizem para medir antes de decidir; esta diz para registar o suficiente sobre a medição para que outra pessoa, mais tarde, consiga voltar a chegar ao mesmo número, ou perceber porque não consegue.

### Como aplicar

Nos relatórios futuros, sempre que um número apareça, verificar se é possível responder à pergunta "a partir de que estado exacto do código veio este número?" sem ter de adivinhar. Se a resposta for "não sei", o número não deve ser usado para fechar nada — só como indicação a confirmar de novo.

### Ligação às regras anteriores

- **Decisão nova obrigatória** — nenhum item aceite é permanente sem decisão nova.
- **Disciplina de verificação** — uma afirmação sobre desempenho ou grandeza de um problema precisa de número, não de adjectivo.
- **Esta regra** — um número, para servir de prova, precisa de se saber de onde veio.

As três juntas cobrem o ciclo completo: decidir de novo, com prova, e com essa prova a poder ser encontrada outra vez.

---

## Atomização — mover a lógica para o arquivo da unidade, na sua camada (ADR-0109)

**Atomização** = mover a lógica de arquivos **monolíticos** para o arquivo da **unidade dona, na
sua camada** (cada elemento/feature **legível sozinho**). **NÃO** é zerar `content→elements`,
**NÃO** é desacoplamento de imports, **NÃO** usa vtable/`dyn`/PropMap, **NÃO** remove o `match`
exaustivo. A **forma é a B**: o `match` no núcleo fica **magro** (corpos delegam a uma free function
`<elem>::layout(self, e)` em `compiler/layout/<elem>.rs`); a lógica de render muda para o arquivo da
feature **na camada de render** (acede ao `Layouter` por **descendência de módulo** — **sem** import
reverso `entities→compiler`, **sem** `pub(crate)`). A **Opção A** (lógica no arquivo do *struct*) está
**rejeitada** (cria o acoplamento dado→render). O `match` exaustivo, a **jump table** e os **imports**
ficam. A métrica da lente (`content→elements`) é **irrelevante** — não a use como gate. Se um plano
propuser despacho dinâmico, "zerar `content→elements`", ou a Opção A em nome de atomização, **isso é
a deriva que esta ADR proíbe** — pare e separe os significados. Ver **ADR-0109**.

---

## Protocolo de Nucleação (obrigatório antes de código)

1. **Auditoria L0:** existe prompt em `00_nucleo/prompts/` para o módulo afetado? Está atualizado face às ADRs vigentes?
2. **Validação L0:** se não existe ou está desatualizado, a IA deve redigir o novo L0 e **PARAR**. Só prossegue quando o humano confirmar que guardou o ficheiro e tem o hash.
3. **Testes primeiro:** escrever os testes no módulo (`#[cfg(test)]`). Confirmar que falham.
4. **Implementação:** escrever o código para os testes passarem.
5. **Linhagem:** adicionar header obrigatório com `@prompt-hash` e `@prompt` corretos.
6. **Validação final:** `cargo build && crystalline-lint .` — zero violations.

---

## Travas do linter e Erros Comuns

| Cód | Nome | Solução |
|-----|------|---------|
| V3 | `ForbiddenImport` | Import viola a topologia de camadas. Reorganize a dependência. |
| V4 | `ImpureCore` | Símbolo de I/O detetado em L1. Mova para L3 e injete via trait. |
| V5 | `PromptDrift` | Hash do prompt diverge. Corra `crystalline-lint --fix-hashes .` após editar L0. |
| V13 | `MutableStateInCore` | Estado global detetado em L1. (`Arc` instanciado em struct não aciona isto.) |
| V14 | `ExternalTypeInContract` | Import externo em L1 não declarado. Padrão: não usar `pub use self::X::Y` em L1. |

---

## ADRs Vigentes — ler antes de propor arquitetura

| ADR | Decisão |
|-----|---------|
| ADR-0018 | `rustc_hash` autorizado em L1 (revoga ADR-0007) |
| ADR-0024 | `EcoString` em `Value::Str` — clone O(1) em `eval()` |
| ADR-0026 | `Content` como enum fechado; `Arc` em `Sequence` |
| ADR-0029 | Pureza física — `Arc` em struct de domínio permitido (revoga ADR-0028) |
| ADR-0030 | Performance de RAM é domínio de L1; corrige ADR-0004/0015 |
| ADR-0031 | Early hashing em `Source`; complementa ADR-0016 |
| ADR-0107 | Paridade é com a linguagem (semântica/sintaxe/morfologia), não com a mecânica/igualdade do Rust |
| ADR-0108 | Disciplina anti-deriva: medir antes de decidir (6 regras verificáveis); o dono audita a substância |
| ADR-0109 | Atomização = lógica de render para `compiler/layout/<elem>.rs` (forma B, free function; Opção A dado→render rejeitada); `match` exaustivo + estático + imports ficam; NÃO é desacoplar `content→elements` |
| ADR-0126 | Export PDF: modo verboso (vanilla-espelhado) é o padrão de produção; compacto = flag `--compact` validada por decalque; PDF tagueado (acessibilidade) é eixo separado |
| ADR-0127 | Gate de L0: paragem obrigatória só para contrato público/comportamento por defeito/fase de pipeline; correções internas e de paridade seguem em fluxo contínuo (L0 primeiro + resselo, sem paragem) |

ADRs revogadas não constam na tabela e não devem ser seguidas.

---

Convenções operacionais para `#[comemo::track]` em L1 (gotchas de Clone,
`Option<&str>`, `TrackedMut::reborrow_mut`): ver `01_core/CLAUDE.md`.

## Regra de divisão de constructor entre passos

Quando um constructor (ou qualquer funcionalidade) é deliberadamente dividido entre
vários passos, o **Prompt L0 do passo inicial deve declarar explicitamente que está
incompleto** e nomear o passo que completa cada subconjunto. Não deixar o L0 num estado
intermediário sem registo — isso gera deriva documental (ex.: `duration()` string em P403
vs named args em P405). O registo pode ser uma nota no cabeçalho do L0 ou uma cláusula de
"scope-out futuro" com referência ao passo.

## Regra de leitura do L0 vigente antes de propor arquitetura

Antes de propor uma opção arquitetural numa spec (especialmente subdivisão/consolidação
 de módulos), **ler o Prompt L0 vigente dos módulos afetados** e confirmar o hash. Se o L0
já decidir a questão (ex.: `stream.md` determina não subdividir shape primitives), a spec
**não propõe o contrário como opção preferida**. Qualquer conflito deve ser tratado como
atualização de L0 (com hash novo) ou scope-out, nunca como opção β recomendada contra o
L0 vigente. Este gate evita deriva como a observada em P427 (opção `shape_emit.rs` vs
`stream.md` hash `9acca994`).

Ver `01_core/CLAUDE.md` para a terceira convenção (`TrackedMut::reborrow_mut`
em descida de lifetime).
