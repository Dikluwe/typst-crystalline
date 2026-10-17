# Escopo textual e cauda após `1fr` — encerramento da etapa

A etapa corrige o fechamento do parágrafo sob seu escopo lexical compartilhado e a reserva do texto que sucede um bloco fracionário. As alterações são internas ao layout; não introduzem contrato público, flag ou mudança de fase. A classificação segue ADR-0127 em fluxo contínuo, com L0 atualizado antes de L1 e resselo dos owners afetados.

## Comportamento corrigido

- Um wrapper textual que cobre o parágrafo conserva seu tamanho e `par.spacing` ao fechar antes de Block/Align. Wrappers irmãos com valores iguais não passam a constituir um escopo comum por coincidência de estilo.
- Prefixos textuais certificados em uma única linha reservam o fundo lógico da linha e a margem colapsada. Uma linha já fechada conserva sua fronteira; a baseline preparada para a próxima linha não conta como altura do prefixo.
- O texto após `1fr` é reservado como um segmento, incluindo tamanho de fonte, quebras de linha e margem relevante. Uma cauda que termina exatamente no limite não salta indevidamente para outra página.
- Quando a cauda excede a primeira região, a reserva usa o primeiro fragmento real, respeitando a admissão de viúvas/órfãs. As páginas seguintes mantêm a capacidade normal.
- Separadores textuais permanecem neutros para a altura. A medição isolada certifica nós textuais, usa métricas emprestadas e não reexecuta efeitos no emissor.

Owners afetados: `paragraph`, `sequence`, `sequence/fractional_preflight`, `sequence/inline_prefix`, `cursor` e `tests`, em `01_core/src/compiler/layout/` e seus seis Prompts L0 correspondentes. O diff desta etapa está em `stage.diff`; ele é relativo ao snapshot inicial da etapa, sem atribuir a esta entrega alterações anteriores do working tree.

## Testes e evidência

Foram adicionadas quatro regressões (escopo lexical, cauda no limite, cauda multiline e primeiro fragmento com viúvas/órfãs) e fortalecidas duas regressões existentes, P1381/P1383, com posições esperadas. Há registros RED e GREEN em `implementation/scope-tail-*`, preservados no arquivo de evidência.

A fixture P1383 usa largura de 240 pt para que os três fragmentos permaneçam na mesma linha com FixedMetrics. A fonte original com largura de 200 pt foi medida separadamente com fontes reais contra o oráculo e coincide; a alteração da fixture não substitui essa prova.

A bateria principal partiu de 20/28 documentos divergentes e passou a 28/28 coincidentes em páginas, texto por página e coordenadas (tolerância de 0,0001 pt). Os 24 controles de margens e os 11 controles adicionais de bordas tipográficas e viúvas/órfãs também coincidiram. São conjuntos identificados separadamente, não uma contagem de cenários necessariamente disjuntos.

A verificação SVG/PNG cobriu os 26 documentos de página única da bateria principal: 26 SVGs rasterizados idênticos; 17 PNGs idênticos e 9 com diferença máxima de um nível por canal, sem diferença de dimensões. Essa comparação é evidência auxiliar de geometria/renderização; não impõe igualdade de bytes como contrato de linguagem (ADR-0107). Os dois documentos multipágina foram verificados em PDF.

A suíte completa final executou de `2026-10-14T21:31:46.838919+00:00` a `2026-10-14T21:37:41.282138+00:00`: **8.590 testes aprovados, 0 falhas e 7 ignorados**. Comando: `cargo test --offline --locked --workspace --no-fail-fast -- --test-threads=1`; ambiente de teste com temporários em `/dev/shm`, sockets locais permitidos, oráculo pinado e linter CI no PATH. A guarda confirmou fontes/L0 imutáveis durante a execução. Logs e resultados estão em `workspace-final/`. A primeira rodada completa (`workspace/`) também passou, antes das duas limpezas equivalentes de lint.

Validação final de `2026-10-14T21:28:50.055819+00:00` a `2026-10-14T21:31:46.445975+00:00`:

| Verificação | Resultado |
| --- | --- |
| Build, check de todos os targets, fmt e diff | Passaram |
| Rustdoc com `RUSTDOCFLAGS=-D warnings` | Passou |
| Linhagem estrita V1/V5/V15/V26 com `--fail-on warning` | Passou |
| Hash reverso de código | Passou; log integral em `validation-final/followup-reverse.log` |
| Linter Tekt completo | Zero erros; 380 avisos e 1668 notas |
| Inventário Clippy | 181 avisos, mesma contagem anterior e sem delta por classe |

Os dois avisos introduzidos durante a implementação foram eliminados: V16 por wildcard no certificado de frame e Clippy nonminimal_bool. O conjunto de arquivos alterado nessa limpeza está em `cleanup-scope.json`. **O inventário Clippy usa `--cap-lints warn`; não é aprovação do gate CI `-D warnings`.** Os avisos existentes do Clippy e do Tekt são conjuntos distintos e continuam como dívida registrada. Linter usado: commit `49c6bb0e584104a0c89270431d8402f5c6c61150`.

O replay final dos 28 documentos ocorreu de `2026-10-14T21:30:14.753731+00:00` a `2026-10-14T21:30:43.211416+00:00`, após limpeza e resselo, novamente sem divergência e com guarda de fontes válida (`delivery-summary.json`). SHA-256 do Tekt medido: `431e5f5efb38f5bd04584184ae4750628632306b7f6e7b8d13fe1a347dcc657e`. As baterias de margens, bordas e raster antecedem somente essa limpeza sem alteração de comportamento; seus snapshots próprios estão preservados. Não foi realizada medição de desempenho nesta etapa.

## Limites e pendências explícitas

As sondas de extensão não sustentam uma correção universal de prefixos/escopos:

- `p1392_wrap_com_delta_composto_permanece_opaco`: prefixo multiline ainda diverge em paginação.
- `p1389_multiword_permanece_opaco`: prefixo multiline ainda diverge em paginação.
- `nested-partial`: texto exterior de 18 pt seguido de escopo interior de 8 pt contendo Block ainda apresenta deslocamento vertical de 12 pt. O segmento local não prova que possui todo o parágrafo exterior.

Os certificados anteriores e as rotas opacas continuam em vigor nesses casos. Os dois testes negativos legados não foram transformados em provas positivas de paridade. O novo caminho geométrico exige prefixo textual com uma única baseline material; caudas multiline têm tratamento próprio. As sondas estão em `scope-tail-refutators-final/results.json.gz`, com fontes e coordenadas reproduzíveis.

Não foi medida distribuição estatística de falhas entre código e prompts. Neste recorte, houve correção da implementação e explicitação dos contratos L0 de escopo, altura causal e reserva; o sucesso dos testes não autoriza afirmar que todos os rascunhos do projeto foram eliminados. As demais frentes e a certificação completa de CI/release permanecem separadas. Não houve publicação, push nem release.

## Proveniência e reprodução

HEAD: `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. Cada medição preserva `git status --porcelain`, `git diff HEAD --stat`, hora UTC, argv, hashes de fontes e binários. Os snapshots de validação também preservam o texto dos arquivos de origem. Não se usa apenas HEAD ou string de versão para identificar o executável medido.

Oráculo vanilla: commit pinado `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. O Tekt é `target/debug/typst`; seu hash por execução está nos respectivos `results.json.gz`.

`before-edit.json.gz` conserva a base desta etapa; `summary.json`, `edge-summary.json` e os registros de validação/suíte conservam resultados estruturados. `evidence.json.gz` reúne medições, saídas, fontes, scripts, L0 e código final; `archive-receipt.json` registra o SHA-256 do arquivo e a conferência após reabertura. Scripts `/tmp/tekt-scope-tail-*.py`, também arquivados, reproduzem os comandos; os diretórios de saída são exclusivos para evitar sobrescrever evidência anterior.
