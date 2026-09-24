# Passo 1498 — revalidar o fecho das lacunas confirmadas do P1489

**Estado:** PLANEADO. Dependências: P1490–P1497 concluídos (inclusive gates humanos quando aplicáveis); a triagem P1497 fornece a fila adicional, não substitui as correções.

## Baseline e critério de fecho

O relatório P1489 fixa o alvo vanilla em `9f2b6e8715237cb086899a42873660fe744622e8` (árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`), com binário vanilla SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A matriz tinha 39 casos × seis perfis, sete RED novos; o inventário default tinha seis pais ausentes, 42 filhos bloqueados e 24 diferenças verificadas de 12 famílias duplicadas. Os valores pertencem ao HEAD/worktree e aos hashes de produtos registrados naquele relatório. Antes de comparar, registrar novamente HEAD, `git status --porcelain=v1 --untracked-files=all`, hora quando necessário, comandos, pin e SHA-256 dos dois binários. Não usar `/usr/local/bin/typst`, que permanece no pin antigo.

## Execução e aceitação

1. Rodar testes focais RED→GREEN de cada passo e depois matriz bilateral nos seis perfis `default`, `html`, `a11y-extras`, `html+a11y`, `bundle` e `html+bundle`. As sete sondas `P1489-L-001`, `I-001`, `S-001` e `B-001`–`B-004` só viram MATCH quando suas asserções internas e observáveis de linguagem completos passarem; para `S-001`, usar o comparador focal de sentinelas/formação textual definido no P1494, não igualdade da árvore HTML inteira, que inclui classes e estilos incidentais. Caso desligado por feature não dá crédito.
2. Reextrair e comparar inventário nos seis perfis. Exigir os seis módulos-pai e 42 descendentes comprovados, bem como os 12 grupos de variantes e espelho `sym`/`math` por probes de valor/`repr`. Explicar qualquer diferença verificada remanescente. Extras de compatibilidade aprovados continuam rotulados como extras, não MATCH.
3. Reavaliar os dois `DIFFERENCE` de formato: podem passar a `MATCH` ou `VIOLATED` se uma nova sonda o provar; enquanto não houver testemunho semântico, conservá-los como `DIFFERENCE`, sem tolerância arbitrária. Publicar a triagem P1497 com `UNVERIFIED_METADATA` remanescente explicitamente inconclusivo. Se surgir violação nova, abrir passo causal e não declarar fecho geral.
4. Executar `CARGO_BUILD_JOBS=1 cargo build`, testes relevantes, `crystalline-lint .` com zero violações e `git diff --check`. Confirmar V5/V15/V26 após resselo; evitar builds simultâneos, servidor/watch e processos pendurados. Limpar somente artefatos temporários criados por esta execução, após guardar as provas, sem tocar no trabalho alheio.

**Conclusão permitida:** fechar **as lacunas de linguagem confirmadas pelo P1489** somente se todas as sondas correspondentes estiverem GREEN nos perfis aplicáveis. Não declarar paridade global nem percentual a partir dos 39 casos ou do inventário; registrar separadamente observações inconclusivas, diferenças de formato e extensões deliberadas.
