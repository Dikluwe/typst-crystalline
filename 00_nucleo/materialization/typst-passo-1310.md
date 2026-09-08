# Passo 1310 — diagnóstico de tipo da fonte nos decoders

## Medição e decisão

P1309 selecionou `loader-data-source-cast` no baseline
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`. Certificado:
`00_nucleo/diagnosticos/p1309-certificate.json`, SHA-256
`ba40e1a5189efd2191dfad8d04841e1ddfad9a6cb6052ddf478efde8fe22c376`.
A testemunha `json(42)` no vanilla ratificado upstream `a51e02804` informa
`expected path, string, or bytes, found integer` e aponta para `42`;
o cristalino usa mensagem portuguesa, tipo curto e origem detached.
Fonte causal: `01_core/src/compiler/stdlib/loading.rs:1047-1064`;
cast normativo vanilla: `lab/typst-original/crates/typst-library/src/loading/mod.rs:46-63`.

Corrigir essa causa comum para `cbor`, `json`, `toml`, `xml` e `yaml`.
Mensagem e localização diagnóstica são observáveis de linguagem (ADR-0108),
não exigência de copiar o mecanismo Rust do vanilla. É inferência que o carrier
Args vigente basta; uma origem conservada no vanilla mas irrecuperável no
consumer a refuta e exige diagnóstico antes de ampliar o escopo.

## Escopo e autorização

O dono pediu escrever e implementar este passo. Aplicar ADR-0127 em fluxo
contínuo: L0 primeiro, resselo e RED→GREEN. Nenhuma assinatura, entidade,
trait, feature ou fase de pipeline muda.

Único owner produtivo: `01_core/src/compiler/stdlib/loading.rs`.
L0: `00_nucleo/prompts/compiler/stdlib/loading.md`.
Aceitar os mesmos tipos Path/Str/Bytes; rejeitar os demais com a lista de tipos
esperados e o nome público longo do tipo recebido, na origem do primeiro
posicional. Preservar detached legítimo; não inventar span da chamada inteira.

Fora: argumento ausente, named/excesso, parsing, I/O, read/csv, encoders,
call_dispatch, novos carriers e as 37 mutações de produto pendentes de P1307.
Essas dívidas não são quitadas por este patch.

Fronteira descoberta antes do candidato: Symbol é coercível a Str no vanilla,
mas continua rejeitado no cristalino. Não implementar essa coerção aqui.
O L0 explicita o delta do diagnóstico comum para Symbol, sem chamá-lo de
paridade ou preservação literal; as expectativas desse controle devem ficar
separadas das expectativas vanilla. Manter a medição inicial que refutou a
hipótese de rejeição bilateral universal.

## Execução proporcional

Usar a skill `tekt-materializacao-segregada` em regime A/B: o risco é adaptar
testes à implementação de um único ramo, sem novo contrato de refinamento.
Testador independente recebe L0 e baseline, não patch candidato, congela
fixtures/expectativas antes da implementação. Coordenador escreve intenção,
testes unitários locais e código; revisor distinto julga os artefatos, sem
editar a solução ou os testes congelados. Contextos de agentes novos sem
herança; filesystem compartilhado: **executado sem atestação de isolamento
técnico**. Não alegar selo do protocolo completo ou score de mutação de produto.

1. Registrar baseline, hashes dos artefatos P1309 preservados, UTC, diff/stat,
   versões e binários. Temporários em target dedicado na RAM; bytecode Python
   desativado. Não sobrescrever o binário baseline P1309.
2. Atualizar o L0 antes de Rust e validar ownership/Núcleo, sem reparo amplo.
3. Congelar testes independentes; escrever `#[cfg(test)]` no owner e executar
   RED real antes do patch.
4. Corrigir só o cast inválido e validar GREEN; executar matriz bilateral nos
   perfis default/html/a11y/html+a11y, repetir e inverter ordem.
5. Cobrir nomes longos, tipos variados, valor/call span distintos, origem de
   With/Args/spread e detached; controles válidos Path/Str/Bytes e fronteiras
   excluídas. Preservação baseline não é igualdade vanilla.
6. Rodar fmt, build workspace release, testes workspace release, linter e
   diff-check; publicar avisos/ignorados sem chamar exit zero de zero findings.
7. Revisor confere escopo, hashes, RED→GREEN, saídas integrais e limites;
   relatório substantivo em `00_nucleo/diagnosticos/p1310-final-report.md`.

Timeout, crash, fixture/binário incompatível ou ausência de observável
obrigatório são Unknown e bloqueiam fechamento. Budget: duas revisões focais
sem ganho na mesma causa obrigam reexaminar hipótese; não repetir corpus
integral para tentar fabricar sucesso. Registrar qualquer fronteira refutada.

Escritas: este passo, o owner/L0 citados e evidências novas `p1310-*` em
diagnósticos. Preservar todos os artefatos anteriores e alterações do dono.
Não fazer stage, commit ou push neste pedido. Não escrever o passo seguinte.
