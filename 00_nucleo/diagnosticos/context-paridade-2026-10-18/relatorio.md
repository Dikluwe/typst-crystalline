# Context — resultado da materialização da arena

## Resultado e escopo

A API aditiva aprovada foi implementada. As quatro lacunas iniciais estão
corrigidas no percurso PDF; a correção textual também passou em HTML.
A aceitação ampliada de consultas contextuais **não está integralmente fechada**:
persistem limites de replay e de `query` em HTML, discriminados abaixo.

| Sonda inicial | Tekt antes | Tekt verificado | Vanilla pinado |
|---|---|---|---|
| Regra textual global | A | R | R |
| Regra textual local | A | L | L |
| Duas equações contextuais | EQ EQ Q1 | EQ EQ Q2 | EQ EQ Q2 |
| Heading antes de equação contextual | H EQ Q0 | H EQ Q1 | H EQ Q1 |

O controle de posição por `it.location()` passou de 00 para 01, igual ao vanilla.
Aliases mantêm uma ocorrência por identidade; equações iguais de produtores
distintos, Contexts aninhados e elementos novos emitidos pelo callback têm
contagens corretas nos controles PDF. Crescimento/retração passam quando o
observador participa do protocolo seletivo vigente.

## Implementação e aprovação

Proposta preservada em
`00_nucleo/materialization/context-occurrences-2026-10-18/review.md`, SHA-256
`500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388`.
O recibo conserva a resposta humana “Aprovar a proposta de identidade”.
L0 instalado antes do código; ownership 1:1 e núcleos validados antes do resselo.

Novo owner: `00_nucleo/prompts/compiler/realize/context_occurrences.md`.
Consumer: `01_core/src/compiler/realize/context_occurrences.rs`.
A arena reserva as identidades do conteúdo original; cada produtor retém seu
frame de slots, reutiliza-os em reexecução e pede novos slots para crescer.
Aliases Located permanecem intactos. A operação restaura o contexto anterior
também em retorno Err. A Session cria a arena na primeira inclusão de produtor.
Não foram alteradas as assinaturas existentes, seleção contextual ou fase.
O lote textual anterior é descrito separadamente em `relatorio-textual.md`.

## Validação e falhas preservadas

- RED inicial de identidade: nove falhas e um controle aprovado,
  `red-occurrences.log`. O primeiro candidato aprovou sete e expôs três limites
  adicionais; fontes e log preservados, sem declarar essas sondas verdes.
- Dez controles integrados delimitados de identidade passaram; quatro testes
  da arena verificam reserva, disjunção, replay, crescimento/retração e Err.
- A primeira suíte completa detectou uma regressão no teste existente de retenção
  ao incluir um produtor numa Session inicialmente vazia. Corrigida a inicialização
  privada em add, com L0 primeiro; os dois testes de retenção passaram sem alterações
  nos esperados. `validation-occurrences-final` é a execução que falhou;
  `validation-occurrences-verified` é a revisão posterior validada.
- Suíte verificada: **9133 aprovados, 0 falhas,
  7 ignorados**, incluindo 5 doctests. A conta por executável
  está em `suite-summary-occurrences-verified.json`.
- Build workspace, Clippy all-targets estrito e expectativas, Rustdoc estrito,
  formato, linhagem direta/reversa e diff-check passaram.
- SARIF verificado: 105 V16:error, 1140 V19:note e 879 V20:note. Nenhum novo erro/warning em relação ao
  baseline de fontes comprovadas. Os **105 V16:error preexistentes permanecem**;
  o linter global não está verde. Não foram adicionadas supressões.
- CLI bilateral PDF: 20 de 25 sondas coincidem em status e
  texto normalizado. HTML: 10 de 25 coincidem; as 25 mantêm
  os mesmos observáveis do binário anterior à arena. `probe-audit-verified.json`
  exige os sucessos delimitados e conserva explicitamente os casos pendentes.

## Pendências medidas

1. `equation-alias` com assert sobre a primeira query vazia termina antes do
   replay legado. O controle separado `equation-alias-count` imprime Q2
   corretamente; isso não resolve a asserção prematura da sonda original.
2. `equation-grow`/`shrink`, numerados ou não, mantêm Q0 quando a query está em
   outro produtor não selecionado. As variantes `*-selected-query` fazem uma
   leitura real do contador filtrado também no observador e passam; não são
   substitutas das quatro sondas gerais ainda divergentes.
3. Em HTML standalone, query/it.location de equações continuam divergentes;
   filtered-counter não é aceito por esse protocolo. `prepare_html` conserva
   suas fronteiras Position/Page. Não se ampliou a seleção para fechar a tabela.

A recompilação controlada do freeze textual reproduziu exatamente seu binário
SHA-256 `f3dc15354db6b3b27829ef0f0da9fcd20b2be29ec44e7a1dd6aa613cc1d2b341`.
`occurrences-html-before.json.gz` e a coleta verificada comprovam ausência de
regressão nos status/textos/erros primários medidos. Isso não certifica paridade
HTML, todas as geometrias, ausência universal de bugs ou ganho de RAM.
R3-T2 e o restante do inventário não estão fechados por este lote.

## Proveniência e recursos

HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, branch Tekt, **working tree não commitado**.
Freeze autoritativo: `occurrences-verified-sources.json.gz`, incluindo o novo
owner e consumer; diffstat e status incluídos. A patch comprimida contém os
arquivos já rastreados; os arquivos novos completos estão no freeze e na árvore.
Baseline: `before-sources.json.gz` e `before.json.gz`.
Comandos, início/fim UTC, diffstat e cgroup: `validation-occurrences-verified-resources.json`.
Encerramento: `2026-10-18T17:19:46.716692+00:00`.

Vanilla: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Tekt verificado: SHA-256 `7c42071ef5a60a2791c5f9636cd3b80672784043f793fa70cadbceedabd35a29`.
Sondas completas: `occurrences-verified.json.gz` e `occurrences-html-verified.json.gz`;
cada caso conserva fonte, argv, status, stderr e hash dos binários/outputs.

Execução sequencial, Cargo jobs=1, debug=0, incremental=0, Max12GiB,
High8GiB, swap zero. Pico da validação verificada: **3.779 GiB**.
Eventos: `low 0; high 0; max 0; oom 0; oom_kill 0; oom_group_kill 0`.
Não é benchmark comparativo. Sem commit, push, merge ou release.

Logs e fixtures textuais ficam em `evidencias-commit.json.gz`, formato
base64-files-v1, verificável pelo helper histórico
`00_nucleo/diagnosticos/commits-por-etapa-2026-10-18/restore-evidence.py`.
`manifest.json.gz` registra hashes dos artefatos e da aprovação.
