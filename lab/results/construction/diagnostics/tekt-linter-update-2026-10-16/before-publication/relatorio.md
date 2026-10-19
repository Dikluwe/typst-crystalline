# Atualização do linter — preparada, pendente de publicação

Pin atual: `49c6bb0e584104a0c89270431d8402f5c6c61150`. Candidato validado: `e53de6fd2bff9f3a87a3e8a076f5e470ce4bb7a0`.
A revisão candidata não pode ser baixada do remoto: `git fetch --depth=1`
retornou exit 128 com `not our ref`. Os refs anunciados pelo GitHub ainda
apontavam master para o pin antigo. Evidência: `provenance.json`.

## Alteração preparada

`update-pin.patch` troca exclusivamente os dois pins em `.github/workflows/ci.yml`
e as duas referências de reprodução em `tools/ci/README.md`. A aplicação foi
verificada por `git apply --check`. Os arquivos ativos permanecem no pin publicado.
Não houve edição de fontes produtivas, L0 ou configuração de severidade.

O patch tem SHA-256 `be2d1555182e9a9d514c1773f23e8e752c6b53bfa6799251c719ec8272e417ee`.
`publication-scope.json` contém todos os commits locais ancestrais que precisariam
ficar acessíveis ao publicar a revisão candidata. Publicar uma branch não exige
merge em master nem criação de release.

## Validação local

Compilação offline/locked a partir de `git archive` do commit candidato,
em pasta temporária, sem alterações no checkout do linter. A identidade da árvore,
hash do archive, fontes congeladas, comandos e SHA-256 do binário estão em
`provenance.json` e `validation.json`.

- Testes unitários selecionados por V19: 12 aprovados.
- Integração do relatório V19: 6 aprovados.
- Auxiliares Python do CI: 16 aprovados.
- Integração crystalline_lint do typst-wiring com o executável novo no PATH:
  2 aprovados.
- Gates V1/V5/V15/V26 estritos e linhagem reversa: aprovados.

## Impacto real no gate global

A versão nova aplica a política já existente `V16 = { level = "error" }`.
Assim, os 242 warnings e 25 notes V16 anteriores passam a 267 erros.
O gate global retorna exit 1. Não foram encontrados diagnósticos não-V19 novos
ou removidos ao comparar regra, mensagem e localização sem a severidade.
`architecture-comparison.json` conserva as duas comparações, com e sem nível;
`remaining-v16.json` registra cada V16 e seu nível anterior.

V19 mantém 864 notas informativas, com mensagem observacional e contagem revista;
V20 mantém 871 notas. A aplicação futura do pin preservará a falha global
decorrente dos V16 existentes; esta etapa não corrige esses casos nem relaxa o gate.
A interrupção inicial na arquitetura foi preservada em `initial-validation/`;
a retomada verificou explicitamente essa mudança de severidade e concluiu os
demais testes. Não há alegação de CI global verde.

## RAM e proveniência

Execuções sequenciais em cgroup com MemoryMax=12 GiB, MemorySwapMax=0 e
CARGO_BUILD_JOBS=1. Maior pico: 4433678336 bytes (4.129 GiB),
zero OOM e OOM-kill. Recibos em `resources.json` e
`initial-validation/resources.json`.

Projeto HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Snapshot inicial UTC `2026-10-17T00:33:08.089176+00:00`; validação UTC
`2026-10-17T00:33:55.753215+00:00`–`2026-10-17T00:37:44.167236+00:00`. Lista exata de fontes e hashes,
status completo e git diff HEAD --stat em `before.json.gz`. Fontes, L0,
configuração e arquivos operacionais monitorados permaneceram idênticos durante
a validação. Não houve commit, push ou release nesta etapa.
