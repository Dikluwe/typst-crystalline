# Prompt — typst-passo-874: consertar o subsetting de fontes CFF (tamanho do PDF, decisão de P797 a revisitar com cuidado)

**Origem**: causa 1 de P873 — `03_infra/src/export/subset.rs:79-89` desativa subsetting para qualquer fonte com tabela `CFF ` desde P797, porque `oxifont_subset` produzia CFF Name-keyed inválido para embed `/CIDFontType0`+`Identity-H` (poppler/ghostscript recusavam a fonte). Isso faz o cristalino embutir fontes CFF inteiras em qualquer documento (confirmado por P873 em math, hello e lorem — não é específico de math).
**Estado**: aguardando execução

---

## Atenção: isto não é reverter a decisão de P797, é resolver o motivo dela

P797 não desativou subsetting por capricho — desativou porque a alternativa (`oxifont_subset` produzindo CFF inválido) quebrava a compilação em leitores reais. O teste `p523_subset_cff_nimbus_sans_preserves_cff_table` já existe consagrando esse comportamento (não-subsetting para CFF) como esperado. Este passo não é sobre apagar esse `return None` — é sobre consertar (ou trocar) o subsetter para produzir CFF CID-keyed válido, e só então mudar o comportamento, com prova de que o resultado abre corretamente nos leitores que P797 identificou como quebrando (poppler, ghostscript).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate.

## Passo 1 — Entender o problema exato de `oxifont_subset` com CFF

1. Ler o relatório original de P797 (se existir, `00_nucleo/diagnosticos/typst-passo-797-relatorio.md` ou equivalente no handoff antigo) para entender exatamente que tipo de CFF inválido era produzido (Name-keyed vs CID-keyed, especificamente).
2. Reproduzir o problema: gerar um subset CFF com `oxifont_subset` sobre uma fonte de teste, e confirmar com uma ferramenta de validação de fonte (`fontTools`, ou o mesmo método já usado em achados anteriores deste projeto para inspecionar fontes) que o resultado é de fato Name-keyed em vez de CID-keyed.
3. Confirmar como o vanilla resolve isso — o vanilla claramente consegue produzir CFF subsetted válido (P873 mostrou `sub yes` no PDF vanilla). Ver que subsetter/abordagem o vanilla usa (`lab/typst-original/`) para CFF CID-keyed.

## Passo 2 — Corrigir ou trocar o subsetter

1. Se `oxifont_subset` puder ser configurado/corrigido para produzir CID-keyed CFF (pode ser uma opção da própria biblioteca, ou um passo de pós-processamento), fazer isso.
2. Se não for possível com a mesma biblioteca, avaliar alternativas já usadas no projeto ou disponíveis como dependência L3 — mas isso é decisão de peso de dependência, mesmo padrão de decisão explícita já usado no projeto (SVG, PDF-como-imagem) se a alternativa for uma crate nova significativa.

## Passo 3 — Validar contra os leitores que quebravam antes

1. Gerar um PDF com fonte CFF subsetted pelo novo mecanismo e testar abertura/renderização em **poppler** (`pdftoppm`/`pdftotext`, já usados neste projeto) e **ghostscript** (`gs`) — os dois leitores que P797 identificou como recusando o CFF Name-keyed inválido. Confirmar que ambos abrem sem erro e o texto renderiza corretamente.
2. Testar também com `mutool`/`pdfinfo`, já usados no resto do projeto, para manter consistência de ferramentas.
3. Se qualquer um desses leitores rejeitar o novo subset, a correção não está pronta — não prosseguir para fechar o passo.

## Passo 4 — Reativar o subsetting

1. Remover (ou condicionar corretamente) o early-return de `subset.rs:79-89` para fontes CFF, agora que a produção de CID-keyed válido está confirmada.
2. Atualizar o teste `p523_subset_cff_nimbus_sans_preserves_cff_table` — ele hoje consagra o comportamento antigo (não fazer subset); precisa ser reescrito para verificar o novo comportamento (subset real, produzindo CFF válido), não removido sem substituição.

## Passo 5 — Validação final

1. Repetir a medição de `pdffonts` de P873 (glifos embutidos) nos três casos que P873 confirmou afetados (hello, lorem, math) — confirmar que o cristalino agora embute só os glifos usados, próximo da contagem do vanilla (18 glifos para o caso de math, por exemplo).
2. Repetir o benchmark de tamanho de PDF de P872 para os cenários afetados — confirmar que os PDFs cristalinos ficaram significativamente menores, próximos da ordem de grandeza do vanilla.
3. Repetir a medição de tempo de P872 para o cenário de math — esperado: alguma melhora (menos bytes para escrever), mas **não** espere que resolva o problema todo, porque a causa 2 (busca de fallback, P875) ainda está presente e é a dominante no tempo.
4. Suíte completa, comando + contagem antes/depois, discriminada por crate.

## Relatório

`00_nucleo/diagnosticos/typst-passo-874-relatorio.md` com: o que estava errado no subsetter e como foi corrigido/trocado, a validação em poppler e ghostscript (obrigatória, não pular), a medição de glifos embutidos antes/depois nos três casos, o efeito no tamanho e no tempo dos PDFs, e as contagens de teste discriminadas por crate.
