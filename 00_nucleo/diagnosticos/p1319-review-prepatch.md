# P1319 — parecer pré-patch definitivo

**Veredito: GO para materializar exclusivamente o recorte P1319 congelado.**
Não é aceite do candidato, selo, atestação técnica de isolamento ou paridade
geral CSV. Regime A/B; revisor `/root/p1319_review`, sem editar julgados.

## Cadeia auditada

Revisões anteriores: `p1319-review-preliminary.md`, `p1319-review-l0.md` e
`p1319-review-red.md`. Elas registram fontes, contratos, classe ADR-0127,
delta explícito de testes e RED genuíno sobre produto ainda basal.

Freeze final `p1319-ab-freeze.json`, SHA-256 recalculado
`61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f`.
L0 raw `5c33854b87f1af32a6bee7f62de0aa27f73ee6496f27e5662d253db2349aff75`,
normativo `da59f9964566dc99d3341145fe8200df15ff8df172158c79d91542d47015e823`.
Root e testador confirmaram que nenhum candidato foi escrito/executado antes
do freeze. O prefixo produtivo foi comparado independentemente ao HEAD na
revisão RED, idêntico salvo linhagem.

Foram recalculados todos os 57 hashes em `freeze.inputs`, sem divergência.
O runner final e a calibração foram lidos integralmente; funções herdadas de
execução/comparação foram inspecionadas. O comparador exige input/L0 íntegros,
identidade de expressão/cwd, conjunto exato de chaves sem duplicação e todas
as ordens normal/repeat/reverse. Falta, drift, timeout e estado desconhecido
não recebem PASS implícito.

## Evidência independente e calibração

Os números abaixo foram recalculados do freeze e dos recibos ligados por hash,
sem reexecutar o corpus. Proveniência: HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`; estado/diff-stat integral nos
recibos baseline e focal; freeze em 2026-09-08T16:09:27.147983+00:00.
Baseline SHA `0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c`;
vanilla ratificado a51e02804, executável SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

- 645 casos, 2580 expectativas, todas com chave única caso/perfil.
- 432 expectativas diferem do baseline: 400 copiadas integralmente do
  diagnóstico vanilla medido; 32 normativas explicitamente separadas.
- 2148 preservações reproduzem integralmente o baseline.
- 2052 replays históricos conservam observação, expressão e cwd anteriores
  antes dos deltas das oito fixtures Path/Str inválidas contratadas.

O revisor comparou programaticamente todos os replays e todas as expectativas
de paridade/preservação com os recibos fontes: nenhuma divergência. As rotas
normativas foram inspecionadas: detached acrescenta apenas a mensagem medida,
preservando detached/traces; excesso mantém parsing legado e acrescenta
origem do primeiro valor. Não são apresentadas como igualdade vanilla.

A calibração inicial encontrou quatro expressões cross-file com erro basal
`current file is outside its sandbox root` antes do CSV. Os brutos, casos e
runner iniciais foram preservados por hash. A revisão focal mantém essas
expressões como controles literais e registra explicitamente ausência de
prova cross-file. Acrescenta quatro expressões Path/Str de subdiretório,
medidas em 32 processos focais; os brutos confirmam chegada ao parser.
Não retirou replays nem converteu a falha de sandbox em sucesso de parsing.

O script de calibração valida esses limites e só agrega as linhas focais ao
recibo r1. O preenchimento posterior dos campos de metadata da calibração,
feito antes de comunicar freeze, está explicado no próprio freeze; não houve
mudança de expectativa decorrente disso.

## Limites e condições para aceite final

O L0 e a evidência justificam fluxo contínuo ADR-0127 para CSV Path/Str com
buffer inteiro inválido após falha vencedora do parser. Preservar root/vpath,
origem Args e ordem vigente. Não usar include_path/source nem introduzir ID
externo, contrato público ou fase nova. UTF-8 válido continua dívida própria.

Package, decoder puro, Args sintético, offsets impossíveis e contagem de
World/parser têm limitações de observação CLI explícitas. Os testes locais
e a inspeção do candidato complementam esses pontos; nenhum fechamento
cross-file ou identidade externa pode ser inferido dos casos de subdiretório.

Para aceite final continuam necessários: GREEN local integral, gates de
build/lint/linhagem, A/B completo normal/repeat/reverse contra este freeze,
preservação dos oráculos e revisão do diff de produto. Nenhuma falha posterior
autoriza editar as expectativas silenciosamente. O GO presente só elimina
o bloqueio pré-patch; não antecipa os resultados desses gates.
