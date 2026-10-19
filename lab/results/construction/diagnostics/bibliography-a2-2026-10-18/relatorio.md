# A2-CSL — preservação do estilo bibliográfico

A subetapa corrigiu a perda de MLA/IEEE e de CSL customizado sob receitas e
carriers. Bibliografias substituídas ou ocultadas continuam fornecendo o estilo
correto às citações. **E05/A2 e o objetivo geral continuam abertos**: recuos,
separação de blocos, múltiplas bibliografias e estilo de alias ainda têm diferenças.

## O que mudou

Somente [convergence.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/convergence.rs) e seu
[L0](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/convergence.md) mudaram em relação ao
freeze inicial desta etapa. O L0 foi atualizado antes dos testes/código.

A descoberta do primeiro estilo agora prioriza os snapshots de Bibliography
registrados no índice semântico da tentativa, conservando ordem, locale e a
style_key do CSL resolvido. Isso mantém a declaração efetiva mesmo depois de um
show trocar seu corpo por texto ou vazio. Sem snapshot, a entrada de layout cru
usa o visitante estático existente, por empréstimo, em vez de outro catálogo
parcial de containers. Realized.original não é ressuscitado nesse fallback;
Document/Asset continuam fronteiras. A primeira declaração sem style não é
saltada em favor de outra. A interpretação CSL e a política de fallback não
mudaram. Nenhuma API pública, fase, flag ou supressão de lint foi acrescentada.

## Evidência e validação

- RED funcional antes de implementar: cinco dos sete testes novos falharam;
  os dois controles restantes passaram. Depois, os sete ficaram verdes.
- Oito cenários PDF que perdiam CSL passaram a conservar o texto do vanilla:
  show-set bare/where, override de par, identidade, substituição, ocultação,
  Block e IEEE sob show-set. Os 16 pares de sondas continuam individualizados;
  a entrada multiple permanece divergente e block-hidden rejeita nos dois.
  Texto igual não é prova de geometria/morfologia geral.
- Cinco controles adicionais de texto com CSL customizado conferiram com o
  oracle. O sexto, com título padrão, continua **RED de layout**. O primeiro
  auditor estrito detectou-o, e seu resultado foi preservado; não conta como
  paridade aprovada. A reprodução exata anterior confirmou a mesma extração e
  as mesmas posições. [Contraprova](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-a2-2026-10-18/counterfactual-proof.json).
- Cores reais no PDF: BIBWORD recebeu `#ff4136` sob show-set bare/where, enquanto
  OUTSIDE ficou `#000000`, conforme o vanilla. Seis controles de geometria
  anteriores conservaram todas as caixas de palavras/páginas com delta máximo
  **0 pt**. Os 44 controles textuais R3 mantiveram resultados e primeiro erro.
- **8986 testes workspace passaram, zero falhas, 7
  ignorados; 5 doctests passaram.** Build, Clippy all-targets com `-D warnings`
  e `-D unfulfilled_lint_expectations`, Rustdoc `-D warnings`, formato, linhagem
  V1/V5/V15/V26, reversa (987 consumers) e diff-check passaram. Os quatro
  índices Rustdoc são typst_core, typst_shell, typst_infra e **typst**.
- **V16 106→105; V19 1120 e V20 876 inalterados.** O diferencial remove somente
  o wildcard neutro de convergence; não há erro, warning ou nota nova. O
  gate global continua vermelho pelos 105 V16 restantes.

Fontes/L0 ficaram congelados durante a validação. A contraprova posterior
restaurou temporariamente somente o par source/L0 inicial, reproduziu o
executável anterior e depois restaurou o candidato. Todos os hashes produtivos
foram conferidos e o executável restaurado é byte a byte o validado. Isso
permite conservar as suítes já passadas, sem fingir que o RED visual desapareceu.

Duas falhas do harness estão preservadas: a primeira fixture chamou `new` em
vez de `TagIntrospector::empty` e não contou como RED funcional; depois o
verificador documental esperou `typst_wiring/index.html`, embora o target bin
se chame typst. Rustdoc havia passado. A checagem corrigida consultou os alvos
Cargo e concluiu os gates. A tentativa estrita dos seis controles customizados
e sua classificação posterior também estão em final-checks-attempt1.json.gz.

## Limites medidos que seguem para o próximo lote

Com fonte 12 pt/margem 8 pt, continuações da MLA ficam em x=8 no Tekt,
x=26 no vanilla; o override `hanging-indent:3em` conserva x=8 no Tekt e usa
x=44 no vanilla. A primeira entrada também começa na linha da citação no Tekt
(y=5.168), enquanto o vanilla a inicia em y=27.463995. O caso custom-default-title
intercala visualmente partes da citação e da lista: permaneceu idêntico na
contraprova anterior/candidata, e ainda precisa de correção.

Multiple ainda duplica a lista MLA no Tekt. Alias-style já usava CSL, mas ignora
uma cor global posterior: BIBWORD fica preto antes/depois, contra vermelho no
vanilla. Os PDFs anteriores foram recuperados do arquivo e conferidos por
SHA-256; não é regressão atribuída sem prova. Esse caso integra os refutadores
de inserção/escopo R3-T1. [Medições geométricas/cores](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-a2-2026-10-18/geometry-controls.json.gz).

Prioridade seguinte: A2-Indent e fronteiras de bloco/título; depois A2-Multi.
R3-T1 depende da aprovação já solicitada; R3-T2/retornos e os demais E01–E20
permanecem no [inventário geral](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/paridade-novo-vanilla-2026-10-17/inventario.md).
Não houve validação HTML nova nem declaração de paridade completa de bibliografia.

## Recursos e proveniência

Pico dos jobs: **4367253504 bytes (4.067 GiB)**, abaixo de
MemoryMax **12 GiB**, MemoryHigh **8 GiB**, swap **0**. Cargo jobs1,
debug0, incremental0 e comandos pesados sequenciais. Eventos high/max/OOM/
OOM-kill zero em todos os jobs. É pico de cgroup, não consumo total da máquina
nem benchmark pareado de ganho de memória.

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch `Tekt`, working tree não commitado.
[before-sources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-a2-2026-10-18/before-sources.json.gz) preserva fontes/L0, UTC,
status e `git diff HEAD --stat`; complete-freeze registra os 2092
hashes validados. [final-state.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-a2-2026-10-18/final-state.json.gz) registra o
encerramento, os 2090 arquivos preservados e o patch
restrito. Comandos e horários individuais estão em complete-results.json e nos
recibos de recursos; os resultados completos estão em before/after.json.gz,
custom-csl-before/results/audit.json.gz, audit.json.gz e geometry-controls.json.gz.
A fixture audit.bib está preservada no arquivo before-artifacts e por SHA-256.

Oracle pinado **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore
cac836c07e4dec2a89219370428292393a3216b5; binário
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Tekt anterior reproduzido: `57154005875bca986a1c4eb44038c8bd9ad2b3580e5534059574895c94e62f82`.
Tekt final: `97f6fa3078bdd6c40aabb80d6f93f3fd019b405ce70fbf2266686d8fb42271cb`.

A proposta R3-T1 e seus 11 L0s-base continuam intactos, manifesto
`47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`. Esta etapa não altera sua aprovação
pendente. Sem commit, push, merge, publicação, release ou encerramento do objetivo.
