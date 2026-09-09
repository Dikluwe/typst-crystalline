# Passo 1324 — erro de field ausente em função nativa com namespace

## Medição e escolha

P1323 fechou somente o warning experimental HTML. A fila diagnosticada em
`00_nucleo/diagnosticos/p1322-classification-selection-r2.json` aponta depois
`namespace-function-missing-field`. Isso não basta como aprovação histórica:
a medição fresca `00_nucleo/diagnosticos/p1324-baseline.json`, SHA-256
`8684dabd1370997232a61bc58a48a99528718e49ec0e81962eb14c70b3769b4d`,
confirma nome/aspas/âncora divergentes em json/yaml/toml/cbor/assert/table,
incluindo alias, With e uso como callee. Registra fontes, argv, UTC, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree/diff e binários.
O alvo permanece vanilla upstream `a51e02804`, não uma tag de versão.

`field_access.rs:181–195,271–277,420–435` dispõe da categoria, nome e AST;
`entities/func.rs:291–302,355–365` conserva nome/namespace através de With.
Vanilla `foundations/func.rs:291–308` publica nome da função no erro e
`typst-eval/src/code.rs:347–366` usa o span do field. O owner causal completo
é `01_core/src/compiler/eval/bindings/field_access.rs`.

## Escopo e execução

Atualizar primeiro `00_nucleo/prompts/compiler/eval/bindings/field_access.md`,
substituindo expressamente apenas a proteção de erros Some da obrigação P1311.
Para Native/NativeWithEngine com namespace Some, inclusive vazio e With,
lookup ausente publica o nome público da função e ancora somente o field.
Lookup presente, nativas None, Closure/Plugin/Element e todos os outros tipos
preservam os contratos atuais. Não criar namespace/membro, API ou fallback.

É correção de diagnóstico da linguagem, não igualdade mecânica Rust; fluxo
contínuo ADR-0127. Se categoria, nome ou origem exigirem outro owner, API,
default ou fase, reabrir o L0 e parar no gate aplicável antes desse código.

Usar regime A/B da skill de materialização segregada, sem selo de refinamento:
root escreve intenção/L0 e implementação; autor de testes independente recebe
L0/vanilla/contratos, não source candidato; revisor julga sem editar julgados.
Ambiente compartilhado, sem atestação técnica de isolamento.

1. Congelar baseline, obrigação, papéis, testes e fronteiras antes do candidato.
2. Preservar o teste P1311 original no baseline. Suceder somente suas duas
   expectativas que contradizem a nova obrigação; conservar seus controles.
3. Integrar testes independentes, executar RED real e só então implementar.
4. Verificar nativas com/sem namespace, nomes qualificados, alias/With,
   leitura/callee, identificadores distintos e spans deslocados; sucesso deve
   continuar exercitado. Cobrir default/html/a11y/html+a11y e controles vizinhos.
5. Executar corpus normal/repeat/reverse; Unknown obrigatório bloqueia.
   Comparar diagnóstico completo no recorte e preservar desvios fora dele.
6. Build/test workspace, fmt, lint, V5/V15/V26 e cálculo bilateral de linhagem.
   Registrar warnings/ignorados, incidentes e veredito em diagnósticos.

Budget: duas revisões focais sem ganho na mesma causa exigem reabrir método;
corpus completo só após validação focal. Temporários de build em
`/tmp/p1324-target.x5jDkw`, cópia dedicada sem hardlinks. A tentativa RAM de
P1323 revelou diferença de visão de espaço host/sandbox; não repetir a cópia
RAM sem medição confiável. Não apagar temporários anteriores.

Preservar os seis arquivos dirty anteriores e artefatos P1319–P1323. Escritas
produtivas apenas no par L0/owner; novos diagnósticos/fixtures P1324. Sem commit,
staging, push, novo passo seguinte ou declaração de paridade global.
