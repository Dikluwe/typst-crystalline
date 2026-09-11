# P1339 — resolução do bloqueio indevido de Angle NaN

## Resultado

O bloqueio por receiver Angle NaN é retirado por aplicação da condição já
escrita no P1339, não por uma dispensa nova de teste obrigatório. A conclusão
anterior do operador confundiu uma obrigação condicional com uma obrigação
incondicional. A revisão independente `p1339-nan-review-r2.md` identificou
esse erro de interpretação.

O registro do receiver Angle NaN continua **`Unknown`**, sem sucesso,
sem cobertura positiva e sem alegação de paridade dessa entrada. Infinito
é construível e continua obrigatório. Os demais gates do P1339 não mudam.

Esta resolução sucede somente a conclusão bloqueante sobre NaN presente em
`p1339-a1-decision-required.md`, `p1339-full-a2.md` e na marcação
`unknown_obligations.actual-angle-NaN-receiver.mandatory` de
`p1339-full-comparison.json`. Esses artefatos históricos permanecem intactos;
seus resultados brutos não são reclassificados. Não há contrato selado ou
candidato cuja verificação esteja sendo relaxada.

## Evidência antes da decisão

### Fonte: domínio de entrada, não simples ausência de uma fixture

Na referência ratificada, `layout/angle.rs:25` encapsula `Scalar` em campo
privado. `raw` e `with_unit` normalizam via `Scalar::new` (`:34-40`);
graus, radianos, razões e trigonometria inversa retornam por esses pontos
(`:44-55`, `:96-114`). Operadores que retornam Angle usam operações de
Scalar (`:177-235`). Em `typst-utils/src/scalar.rs:15,19-31`, o campo também
é privado, as constantes não são NaN e a normalização de NaN para zero é
documentada e implementada. Seus produtores aritméticos retornam por
`Self::new` (`:139-276`).

Assim, nos produtores seguros dessa fonte congelada, NaN não pertence ao
domínio de receivers Angle. Dividir Angle por Angle retorna float, não uma
brecha que produza Angle NaN. Uma comparação interna de Scalar com f64
também não retorna um novo Angle. Não se propõe fabricar um estado inválido
com `unsafe`, mudar a referência ou copiar sua representação Rust.

SHA-256 das fontes principais:

- `lab/typst-original/crates/typst-library/src/layout/angle.rs`:
  `76edf90e5c2e38c61189644d77ef78655486230723ad1fdce6b624f8259182d5`;
- `lab/typst-original/crates/typst-utils/src/scalar.rs`:
  `e3b7bcf23af0ae7fedd312c74fa21b0a65cc14f1cfd62bec556155f75c902891`.

### Sonda focal adicional

Executada sobre HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, com
`git diff HEAD --stat` vazio, sem código/L0 alterado. O status integral de
arquivos não rastreados e os hashes de entrada estão nos recibos. Referência
vanilla `a51e02804`; os binários são os mesmos pinados no A0/P1338.

`p1339-nan-resolution-manifest.json` identifica 16 casos: construções por
aritmética, divisão e trigonometria inversa, mais controles finito, zeros,
infinidades e float NaN. Cada binário executou 128 chamadas: duas ordens e
quatro perfis, totalizando 256 execuções. Não houve timeout nem instabilidade
entre normal e inversa.

Recibos, com comandos e canais integrais:

- `p1339-nan-resolution-vanilla.json`, UTC
  `2026-09-09T23:50:24.560720+00:00` a `23:50:25.599677+00:00`, SHA-256
  `6db01dfb633d2c3115dbc659b6b0a2801bd8b99859a246870640a7a890064955`;
- `p1339-nan-resolution-crystalline.json`, UTC
  `2026-09-09T23:50:25.652099+00:00` a `23:50:59.421068+00:00`, SHA-256
  `dc2a9487e1422b2600d12649df85335a0e28b3e8634e3226fb8efb44235d13c6`.

Exemplos novos: `calc.atan(float("nan"))`, `calc.asin(float("nan"))`,
`calc.acos(float("nan"))`, `calc.atan2` com um argumento NaN, divisão de
ângulo por NaN e de ângulo infinito por infinito devolvem ângulo zero no
vanilla. O controle `float("nan")` continua float NaN: a normalização é do
domínio Angle, não uma ausência geral de NaN na linguagem. Infinidades,
`-0deg` e o ângulo finito continuam distintos de zero/NaN conforme seus casos.

No cristalino, a sonda preserva evidências de dívida: diversos produtores
retêm NaN; `asin`/`acos` o rejeitam; soma de ângulos também é rejeitada.
Nenhuma dessas divergências foi corrigida fora das dez rotas, ocultada ou
atribuída às conversões ainda não implementadas. A sonda investiga produção
do receiver, não executa um gate RED→GREEN dessas conversões.

Nota de reprodução: o runner original `p1339-nan-resolution-probe.py`
(SHA-256 `51dcf7503646ba98fbe3e734dbc601c1a6561878dff7527a05365a1b3053266f`)
usou por engano o caminho do binário no nome do recibo. Depois da conclusão,
somente os caminhos foram corrigidos com `apply_patch`, conservando bytes e
hashes: `p1339-nan-resolution-/usr/local/bin/typst.json` tornou-se
`p1339-nan-resolution-vanilla.json`; o correspondente
`p1339-nan-resolution-/tmp/p1338-target.vlNAmp/release/typst.json` tornou-se
`p1339-nan-resolution-crystalline.json`. O runner e o manifesto original
foram preservados; reexecutá-lo produz os caminhos originais. Nenhum recibo
anterior à rodada foi removido ou sobrescrito.

## Aplicação da regra existente

O passo permanece byte-idêntico, SHA-256
`817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9`.
Em `:121-122`, exige NaN/infinito **apenas se** a sintaxe bilateral puder
construí-los e determina `Unknown` no caso contrário. Em `:185`, bloqueia
`Unknown` **obrigatório**. A segunda regra não elimina a condição da primeira.
Essa condição já existia antes da retificação Angle→float.

A obrigação de testar a conversão sobre Angle NaN não se ativa quando seu
domínio está excluído pela referência. Isso é diferente de um teste cuja
entrada existe e não conseguimos observar, ou cuja execução falhou: nesses
casos, a política bloqueante permanece. Não se criou uma exceção genérica
para `Unknown`, nem se promoveu a posteriori qualquer falha a controle opaco.
Somente a aplicabilidade dessa condição foi corrigida, com prova de fonte e
checagem pública. A proposta anterior de pedir dispensa era desnecessária.

Refutadores que reabrem a questão: produtor seguro da fonte ratificada que
retorne Angle com NaN; expressão pública que efetivamente construa esse
receiver no vanilla; divergência entre fontes/binários pinados; ou decisão
normativa explícita tornando a condição incondicional. A conclusão não vale
automaticamente para versões futuras do vanilla.

## Revisão, limites e próximo ponto real

Revisor `/root/p1339_nan_review`, sem leitura de implementação cristalina ou
candidato; parecer `p1339-nan-review-r2.md`, SHA-256
`5e9ea0ab4dfd36e96c3d861cfc7c915959a21ecd56d52060b670b5a0a7c300dd`.
Operador `/root` realizou a sonda focal e escreveu esta resolução. Regime da
skill `tekt-materializacao-segregada`: executado sem atestação de isolamento.
Não há selo nem veredito final de P1339.

O bloqueio de interpretação sobre NaN está resolvido. Continuam pendentes a
limitação de transporte do suplemento show, o desenho L0 e o gate público de
Selector para `function.where`, contrato, ataques, selo e implementação das
dez rotas. Não foram alterados passo, L0, código produtivo ou política geral;
não foi feito commit. Esta resolução não é conclusão de paridade de Angle.
