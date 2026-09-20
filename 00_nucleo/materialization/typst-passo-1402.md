# Passo 1402 — `outline.entry`: identidade, consulta e `body()`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado de entrada:** `be15d0ab73af089860d03db390508edf21a4bdac`,
working tree limpa antes deste passo, medido em `2026-09-14T22:02:33-03:00`.
**Execução:** sem atestação de isolamento entre papéis.

## 1. Objetivo

Fechar a menor fatia conclusiva que ficou visível depois de P1401:

1. expor `outline.entry` como função-elemento pública no scope de `outline`;
2. fazer `query(outline.entry)` observar as entradas realmente produzidas por
   `outline()` em ordem documental;
3. expor `outline.entry.body` e fazer `entry.body()` devolver o corpo semântico
   do elemento referido;
4. preservar a morfologia pública de `level`, `element` e `fill` nas entradas
   consultadas.

Este passo não implementa `outline.entry.indented`, `inner`, `page` ou `prefix`.
Esses quatro métodos são contextuais ou dependem de layout/paginação e continuam
como dívida explícita. Também não altera defaults de `outline`, `lof` ou `lot`.

## 2. Medição antes da decisão

Referência ratificada: vanila `586e1bd43`, binário
`lab/typst-original/target/release/typst`, identificador administrativo
[dado administrativo removido].
Cristalino de entrada: `target/debug/typst`, identificador administrativo
[dado administrativo removido].

Todas as sondas foram limitadas a 512 MiB de espaço virtual, 15 segundos e
saída útil menor que 64 KiB. Fixture principal UTF-8, identificador administrativo
[dado administrativo removido]:

```typ
#set heading(numbering: "1.")
#outline()

= H
```

Resultados do vanila:

| expressão | observável |
|---|---|
| `repr(outline.entry)` | `entry` |
| `repr(outline.entry.body)` | `body` |
| `query(outline.entry).len()` | `1` |
| `repr(query(outline.entry).first())` | `entry(level: 1, element: heading(... body: [H]), fill: repeat(body: [.], gap: 0.15em))` |
| `repr(query(outline.entry).first().body())` | `[H]` |
| `.level` / `.element` / `.fill` | `1` / heading realizado / repeat de pontos |
| `outline.entry(1, heading[H])` | `entry(level: 1, element: heading(body: [H]))` |
| `outline.entry(1, strong[H]).body()` | erro `cannot outline strong` |

Sem `#outline()`, a consulta devolve zero entradas; com dois `#outline()` e um
heading, devolve duas. Logo a ocorrência pertence a cada realização do índice,
não apenas ao heading fonte. A fonte ratificada confirma o vínculo público em
`lab/typst-original/crates/typst-library/src/model/outline.rs:246-250`, a entidade
e seus campos em `:468-522`, e `body()` em `:668-674`.

No cristalino, `structural/outline.rs` só constrói `Content::Outline` e não tem
scope `entry`; `introspect.rs:1949-1972` guarda apenas o tuple privado para TOC;
`layout/outline.rs:55-70` desenha as linhas diretamente. A causa é ausência da
identidade pública e da ocorrência realizada, não falha restante de `eval --in`.

## 3. Classificação e gate

Trata-se de nova superfície pública (`outline.entry`, seu construtor e método
`body`). Portanto é `PUBLIC_CONTRACT_CHANGE_STOP` pelo ADR-0127.

O Prompt L0 proprietário
`00_nucleo/prompts/compiler/stdlib/structural/outline.md` é atualizado primeiro
com a proposta abaixo. **Nenhum código L1–L4 pode ser escrito até confirmação
humana explícita deste contrato.** A confirmação autoriza somente esta fatia.

## 4. Obrigação materializável após o gate

Para cada `Content::Outline` de target headings, materializar uma ocorrência
`entry` por item incluído após o filtro de `depth`, preservando a ordem do
documento e multiplicando as ocorrências quando há múltiplos outlines. Cada
entrada carrega:

- `level`: nível 1-based;
- `element`: snapshot realizado do heading referido, com localização;
- `fill`: conteúdo efetivo do outline, com default linguístico equivalente ao
  repeat de ponto com gap `0.15em`;
- `body()`: corpo semântico do elemento referido; para elemento não outlinable,
  erro explícito.

A implementação deve manter o despacho estático e o match exaustivo. Não usar
`Content::Dynamic`, vtable ou mapa aberto como atalho. Se uma nova unidade
`OutlineEntryElem` for necessária, ela recebe ficheiro e Prompt L0 1:1 próprios;
os hubs afetados são atualizados antes do código e ressellados juntos.

## 5. Sequência RED → GREEN

Após a confirmação:

1. auditar por inteiro os L0 dos consumers efetivamente afetados e criar o L0
   1:1 da nova unidade, caso exista;
2. escrever testes pequenos de entidade/scope/campo/consulta; confirmar RED sem
   compilar o harness monolítico de `typst-core`;
3. implementar identidade e construtor públicos;
4. materializar entries no caminho de realização do outline, sem duplicar a
   fonte privada de headings nem criar auto-referência do título;
5. implementar campos e chamada `body()`;
6. rodar as sondas bilaterais e os ataques com limites fixos;
7. ressellar hashes e executar `crystalline-lint .`.

## 6. Ataques mínimos e critérios de aceitação

- remover o registro `entry` do scope deve matar a forma qualificada;
- gerar uma entry por heading, em vez de por outline realizado, deve falhar no
  caso de dois outlines;
- gerar entries sem `#outline()` deve falhar no controle negativo;
- ignorar `depth` deve falhar com heading profundo;
- devolver plain text em `body()` deve falhar com markup forte;
- aceitar `strong` como outlinable deve falhar com diagnóstico;
- omitir `level`, `element` ou `fill` da morfologia consultada deve falhar;
- `outlined: false` não pode produzir entry.

Aceitação no nível da linguagem: as formas qualificadas existem, a consulta
retorna as mesmas ocorrências semânticas e `body()` preserva Content/markup. Não
se exige igualdade de bytes internos, `PartialEq` Rust ou algoritmo de layout.

## 7. Proteção contra estouro de memória

- cada binário de sonda: `ulimit -v 524288`, `timeout 15s`, captura máxima 64 KiB;
- build/check: `CARGO_BUILD_JOBS=1`, teto virtual de 2 GiB, debuginfo desligado;
- não rodar `cargo test -p typst-core --lib`: P1401 comprovou que esse harness
  ultrapassa 2 GiB mesmo com debuginfo desligado e linker econômico;
- testes focais por binário pequeno ou crate já materializado, um processo por vez;
- qualquer OOM/timeout é falha de recurso, nunca evidência de paridade;
- não elevar os tetos para transformar uma falha em sucesso.

## 8. Papéis e evidência

Contrato, implementação, ataques e veredito permanecem artefatos separados. Como
este workspace não fornece isolamento verificável entre os papéis, todo recibo e
o relatório final devem declarar `executado sem atestação de isolamento`. O
veredito exige baseline, hashes de executáveis/fixtures, limites, resultados RED
e GREEN, catálogo de mutantes e `git diff --check`/linter finais.
