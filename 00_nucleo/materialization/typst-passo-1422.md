# Passo 1422 — retomar a campanha integral das 66 rotas P1420

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado de partida medido

- O P1420 congelou 66 rotas (`44 html.*` e `22 math.*`), mas parou antes da
  campanha integral porque o controle literal
  `#context assert.eq(target(), "html")` falhava no cristalino.
- O P1421 materializou a expansão de `Content::ContextBlock` no pipeline HTML.
  A verificação funcional P1421 passou e o controle focal HTML passou nas três
  ordens; o seu limite protocolar sobre o resselo recíproco de hashes L0 não
  altera o comportamento do produto nem os oráculos P1420.
- Estado causal desta retomada: HEAD
  `fc803d05f0e7eec7f6bc6a8e363e3cfe1450077d`, tree
  `11a404824b77d608b1eda29ad44fb281d89aef03`, binário cristalino identificador administrativo
  [dado administrativo removido]
  e vanilla ratificado identificador administrativo
  [dado administrativo removido].
- O recibo de inventário presence-only P1420 continua aplicável: a alteração
  P1421 não adicionou nem removeu membros públicos. O P1422 deve validar o seu
  hash e a presença literal das 66 rotas antes de executar produto.

## Objetivo e limite

Criar um sucessor P1422 do runner P1420, executar primeiro o focal
`html.area`/`math.abs` e, se os controles obrigatórios passarem, executar as 66
rotas completas em `direct`, `direct-repeat` e `reverse`. Emitir um estado
conclusivo ou `UNKNOWN` fail-closed para cada rota, sem editar código de produto,
L0, Núcleos, expected históricos ou qualquer artefato P1420/P1421.

O passo mede somente as 66 rotas congeladas. Não declara paridade geral de HTML,
math ou Typst e não concede crédito de rota por controle comum.

## Regime e papéis segregados

Aplicar o protocolo completo `tekt-materializacao-segregada`:

- **dono da obrigação:** este documento;
- **autor de contrato e oráculos:** produz manifesto, contrato e oráculos P1422
  a partir deste passo e das entradas históricas protegidas, sem ler o runner
  candidato nem resultados candidatos;
- **adversário:** produz mutantes e controles discriminatórios, sem editar
  contrato, oráculos, runner ou produto;
- **implementador:** cria apenas o runner sucessor P1422 depois do preseal;
- **verificador:** executa entradas seladas, produz resultados, backlog,
  relatório e certificado e não corrige material sob julgamento.

Como os papéis compartilham filesystem e contexto de repositório, a entrega deve
declarar **“executado sem atestação de isolamento”**.

## Entradas protegidas

O preseal deve proteger por identificador administrativo:

1. este passo;
2. manifesto, contrato, oráculos e mutantes P1422;
3. `/tmp/p1420-current-inventory.json`;
4. binários vanilla e cristalino;
5. `p1420-{manifest,contract,oracles,mutants,preseal,results,verification}.json`;
6. `p1421-{results,report,verification}.json`;
7. `[artefato laboratorial removido]`.

Os oráculos públicos P1420 são reutilizados byte a byte. O contrato P1422 deve
registrar a transformação nominal P1420→P1422, atualizar somente identidade do
produto, HEAD/tree, caminhos de saída e razão de retomada, e rejeitar qualquer
mudança de fixture, expected, denominador ou regra de composição de estados.

## Instrumento sucessor

Criar `[artefato laboratorial removido]`. O runner deve:

- validar preseal, identidades, hashes históricos e denominador antes de produto;
- exigir exatamente 66 rotas e 154 probes por ordem;
- conservar transporte bilateral e independente, proibindo `--jobs` e
  `eval --target` em todos os argv;
- executar as alternativas mesmo quando a forma qualificada falhar;
- conservar projeção HTML canônica e bytes/codepoints/stderr/exit para math;
- executar processos seriais com `RAYON_NUM_THREADS=1`, limite de 512 MiB,
  timeout de 15 s e 64 KiB por stream;
- executar `direct`, `direct-repeat` e `reverse` e exigir vetor idêntico;
- escrever brutos apenas em `/tmp` e o consolidado integral somente em
  relatório consolidado da limpeza;
- nunca converter timeout, OOM, swap, truncamento, identidade divergente,
  baseline divergente ou projeção incompleta em sucesso.

O focal é gate: se qualquer controle obrigatório falhar, parar sem campanha
integral e emitir `INCONCLUSIVE`. Se passar, uma campanha integral e uma
repetição integral independente são obrigatórias.

## Ataques e gates

O adversário deve cobrir, no mínimo, remoção/adição de rota, identidade antiga,
alteração de expected, `--jobs`, `eval --target`, features assimétricas,
alternativa genérica, alternativa condicional ao sucesso qualificado, promoção
de diferença/Unknown, descarte de observáveis, relaxamento HTML, ausência
inferida de erro de argumento, falha de recurso promovida, ordem divergente e
edição de entrada histórica. Exigir controles positivos `Preserved`, opacos
`Unknown`, negativos `Violated`, `mutation_score = 1.0` e vetor estável nas três
ordens.

Executar:

```sh
python3 -m py_compile [artefato laboratorial removido]
python3 [artefato laboratorial removido] --validate-only
python3 [artefato laboratorial removido] --gate
python3 [artefato laboratorial removido] --run --focal --output /tmp/p1422-focal.json
python3 [artefato laboratorial removido] --run --output [artefato protocolar removido]
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Não executar rebuild global nem `cargo test --workspace`: o produto está
congelado e este passo é diagnóstico. Qualquer divergência de identidade fecha
como `Unknown(IDENTITY_MISMATCH)`.

## Artefatos e critério de fecho

Produzir:

- `p1422-{manifest,contract,oracles,mutants,preseal}.json` antes do runner;
- `[artefato laboratorial removido]` depois do preseal;
- `p1422-{results,backlog,verification}.json` e relatório consolidado da limpeza pelo
  verificador.

O passo fecha quando focal e campanha integral passam os gates, cada uma das 66
rotas possui `qualified_state`, `alternative_state`, `route_state` e razão
literal, as três ordens e a repetição independente são idênticas, o mutation
score é 1.0, entradas históricas permanecem byte a byte e os gates estáticos
passam. `UNKNOWN` legítimo fecha o diagnóstico como `INCONCLUSIVE`, nunca como
paridade.

