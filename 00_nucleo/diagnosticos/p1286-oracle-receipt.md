# P1286 — recibo dos oráculos black-box independentes

**Natureza:** recibo de autoria dos oráculos e congelamento do baseline. Não é
implementação, teste interno, ataque, gate discriminatório, selo nem veredito
final de materialização.

## 1. Regime, papel e isolamento

- Regime: protocolo completo da skill `tekt-materializacao-segregada`, fase de
  autor independente dos oráculos.
- Executor: agente `/root/oraculos_p1286`, em 2026-08-30.
- Entradas permitidas: `AGENTS.md`; skill e referências diretas; ADR-0107,
  ADR-0108, ADR-0127 e ADR-0129; somente
  `00_nucleo/materialization/typst-passo-1286.md`; receipts P1286 de medição,
  ownership, contrato v2 e L0 pós-gate; os vinte L0s P1286 finais manifestados
  nesses receipts; e `/usr/local/bin/typst` como caixa-preta.
- Escritas limitadas a
  `lab/surface-inventory/run_p1286_oracles.py`,
  `lab/surface-inventory/p1286-oracle-baseline.json` e este receipt.
- Implementação candidata e testes internos não foram lidos nem escritos. O
  executável candidato só foi chamado como caixa-preta **depois** do baseline
  vanilla estar congelado.
- Ambiente compartilhado: **executado sem atestação de isolamento físico do
  host**. A separação de entradas, ordem e capacidade foi respeitada, mas o
  checkout não prova isolamento por filesystem.

## 2. Entradas congeladas

Snapshot observado em `2026-08-30T19:56:12-03:00`:

| entrada | SHA-256 / identidade |
|---|---|
| HEAD | `53d21c5a602f4045a769a0ab0c935baa5ecd3b88` |
| contrato P1286 v2 | `16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb` |
| receipt L0 pós-gate | `27e2f5b931d898396319d7a62875ca377511e93ebe0e8b97cdd4f6817d6663c0` |
| vanilla exclusivo | `/usr/local/bin/typst`, `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| runner final | `cc0d7986804a7abf006d03e44ffb6eb12a736b794db557b9d60d52e5cc02bf61` |
| baseline final | `7452eea8ac3bc055a6f67586820a0754925ca3a1a80f1244e693cae2989e19fb` |

Os dezasseis L0s existentes tinham exatamente os hashes do receipt contratual
v2; os quatro owners novos tinham exatamente os hashes do receipt pós-gate.
Logo, nenhum caso foi adaptado a uma revisão posterior desses contratos.

A working tree era não commitada e compartilhada. No instante acima,
`git status --short | sha256sum` produziu
`bf4900266f28c252ebe9a1b8eb21a3d648383763907fb7fc51126b53ae30f82a` e
`git diff HEAD --stat | sha256sum` produziu
`96b81f2df4c11863144b9ad4058ecd4add63e2ca03d7b4b6708755bca47a8e9d`.
O status incluía alterações concorrentes de P1281–P1286, produção e suites de
outros papéis. Essa árvore global não é tratada como identidade do baseline:
as dependências causais deste papel são identificadas individualmente acima e
as fixtures geradas têm SHA-256 por caso dentro do JSON. A lista global podia
mudar durante a execução por agentes concorrentes; portanto não se alega uma
fotografia isolada do checkout.

## 3. Política de `Unknown` e ferramentas

`Unknown` nunca é `Preserved`, sucesso ou equivalência. O runner sai com código
3 se identidade/contrato/baseline estiver ambíguo, se uma observação opaca não
puder ser extraída, se as duas ordens divergirem ou se faltar qualquer
ferramenta PDF requerida. Falta de `qpdf`, `pdfdetach` ou `mutool` é sempre
`Unknown` e exit não-zero; `pdftotext` recebe a mesma política porque é a lente
de preservação visual/textual deste fragmento.

Ferramentas observadas:

| ferramenta | versão |
|---|---|
| qpdf | 11.9.0 |
| pdfdetach | 24.02.0 |
| mutool | 1.23.10 |
| pdftotext | 24.02.0 |

`--no-pdf-tags` e `--pdf-standard` são sondados por `compile --help`. Um caso
opcional cuja superfície CLI não existe recebe `NotApplicable`, nunca
`Unknown` nem `Preserved`. Os erros `line(..., dx:/dy:)` são conservados no
baseline vanilla como `BaselineOnly`: o contrato v2 declara explicitamente a
extensão cristalina compatível fora da paridade alegada.

## 4. Cobertura congelada

O baseline contém 61 casos, todos `Observed` no vanilla e nenhum `Unknown`:

| fatia | casos | fragmento observável |
|---|---:|---|
| smartquote | 10 | defaults; alemão alternativo; precedência de quotes; string/array/dict; disabled; cardinalidade/chave |
| line | 10 | origem positiva/negativa; vetor invertido; length/angle/default; precedência de end; aridade; dx/dy baseline-only |
| color.mix | 20 | uma/N cores; método; float/ratio/zero/negativo com soma positiva; quatro espaços; hue N>2; pesos/somas/tipos inválidos |
| pdf.attach | 12 | path e bytes extraídos; nome/Filespec/name tree; MIME/descrição; relationship PDF/A-3; duplicado; path/MIME/relationship/tipos |
| pdf.artifact | 9 | controle normal; default/other/header/background; tags enabled/disabled; texto preservado; propriedades; sem MCID/StructElem próprio; erros |

PDF/SVG/QDF são lentes semânticas, não igualdade de bytes. Attachments são
extraídos e comparados ao payload; artifacts são comparados por texto
extraído, envelope `/Artifact`, property list e ausência de MCID/StructElem
**próprio**, sem exigir a mesma contagem global de nós estruturais entre
implementações. Com tags disabled, o texto permanece e a marcação é omitida.
Com tags enabled, `other` usa marca sem properties; Header e Background têm as
property lists congeladas.

## 5. Comandos e resultados reproduzíveis

Freeze final, executado antes de qualquer candidato:

```text
python3 lab/surface-inventory/run_p1286_oracles.py \
  --freeze --force --binary /usr/local/bin/typst
exit=0
Frozen; cases=61
baseline sha256=7452eea8ac3bc055a6f67586820a0754925ca3a1a80f1244e693cae2989e19fb
```

O runner executou uma passagem forward e outra reverse em diretórios
temporários distintos. As 61 observações foram idênticas. Uma tentativa
anterior foi recusada com exit 3/`Unknown` porque a mensagem de path inexistente
incluía a raiz aleatória do tempdir; somente essa identidade transitória foi
normalizada para `<WORK>/`, preservando a mensagem semântica, e nenhum baseline
dessa tentativa foi escrito.

Self-check contra o próprio vanilla congelado:

```text
python3 lab/surface-inventory/run_p1286_oracles.py \
  --binary /usr/local/bin/typst
exit=0
verdict=Preserved
Preserved=59; BaselineOnly=2; NotApplicable=0; Unknown=0; Violated=0
```

Exercício da interface contra o candidato que existia após o freeze:

```text
python3 lab/surface-inventory/run_p1286_oracles.py \
  --binary target/debug/typst
exit=1
candidate sha256=08e0b7cd1967b00a26178c7daa240989f0c1e0e7c418ddc268c552b427a43509
verdict=Violated
Preserved=6; Violated=52; Unknown=0; NotApplicable=1; BaselineOnly=2
```

Esse resultado apenas caracteriza o binário pré-materialização/stale observado;
não é veredito sobre um futuro rebuild. A reprodução contra qualquer candidato
é feita pelo mesmo `--binary CAMINHO`, sempre em duas ordens.

## 6. Estado da cadeia

Artefatos de oráculo e baseline estão congelados e reproduzíveis pelos hashes
acima. O poder discriminatório contra mutações e o veredito final pertencem a
autoridades posteriores; este papel não os emite. A atestação proporcional é:
**oráculos segregados por capacidade e ordem, executados sem atestação de
isolamento físico do host**. O fragmento cobre somente as cinco fatias P1286 e
não prova equivalência funcional geral do compilador.
