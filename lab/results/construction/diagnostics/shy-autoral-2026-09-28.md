# SHY autoral — identidade por ocorrência e correção da duplicação

O SHY escrito no texto conserva agora sua identidade U+00AD nos runs com fonte
resolvida, sem transformar espaços, letras de ligaduras ou hífens literais em SHY.
A seleção de quebras explícitas permanece uma frente distinta.

## Proveniência e escopo

Working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`.
Os registros citados abaixo ficam em `shy-autoral-2026-09-28/`, junto deste relatório.
`initial.json` e `final-receipt.json` contêm UTC, status, `git diff HEAD --stat`
e identidade dos binários; `current-source.json` registra os hashes completos
produtivos/L0, reconferidos depois da validação. Não se atribuem as outras
alterações já existentes na árvore a esta frente.

- Vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`.
- SHA-256 vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- SHA-256 Tekt anterior: `144c32b06abce5458ac652a71dee83add05518c8934d923b4b308664245818a3`.
- SHA-256 Tekt final: `96b383826b294dc147ace9eb9c012f5825ae97abcab85b0c94d82bba5f32ea8d`.

`scope.json`, `changed-scope.json`, `scoped-seals.json`, `before-source/` e
`review.diff.txt` registram sete pares owner/consumer, quatorze arquivos
produtivos/L0 alterados. A unidade nova tem prompt exclusivo; snapshots de
código/L0 têm extensão `.txt` para não se tornarem consumers do linter.

Esta é correção interna de paridade em fluxo contínuo ADR-0127. Reutiliza o
contrato `FrameItem::TextReplacement` já aprovado na frente anterior, sem alterar
campos, assinaturas públicas, flags ou fase de pipeline. Os L0 precederam o código.
A delimitação inicial de uso do envelope foi ampliada expressamente no owner
`00_nucleo/prompts/entities/layout_types.md`, mantendo seus invariantes.

## Medição anterior à decisão

Trinta entradas bilaterais foram medidas antes da implementação. Em larguras
100/300pt, `repre<U+00AD>sentation` extraía dois SHY no Tekt contra um no vanilla.
O controle `spaces` extraía seis contra um; `ligature` também contaminava o texto
posterior `office`. PDFs, XML bbox, extração UTF-8, fontes e comandos estão em
`before/`, com proveniência do binário anterior copiado intacto.

`03_infra/src/shaper/shaping.rs:313-325` no snapshot anterior transportava o
placeholder de SHY produzido pelo rustybuzz. `export/fonts.rs:318-328` atribuía
seu texto ao GID compartilhado com espaço. `export/builder/fonts.rs:523-535`
retinha uma identidade por GID. A inspeção da stream do controle anterior
mostrou CID 0001 mapeado a 00AD e reutilizado no espaço posterior. O L0 antigo
especificava reconstrução de todo cluster e o shaper não protegia SHY autoral:
a lacuna também estava na especificação, sem evidência de intenção histórica.

O teste `red.log` falhou tanto no mapeamento global contaminado quanto na ausência
de identidade por ocorrência. A correção escolhe a causa explícita do texto de
entrada; não decide por GID, avanço zero, sufixo, família ou posição.

## Atomização e implementação

`03_infra/src/shaper/text_identity.rs:10`, owner
`00_nucleo/prompts/infra/shaper/text_identity.md`, protege sub-runs shaped que
contêm SHY autoral com um envelope de texto original e um filho visual.
O percurso em `shaper/traversal.rs:73-107` impede envelopes aninhados e conserva
os wrappers existentes. Fonte ausente mantém o caminho Text anterior.

`export/fonts.rs:321-330` retira SHY somente das substrings usadas no mapeamento
global ToUnicode, conservando letras, ligaduras e pares substitutos. Um cluster
exclusivamente SHY não reserva o GID compartilhado com espaço com identidade
vazia. Runs sem SHY usam a substring emprestada, sem alocação intermediária nova.
A identidade completa permanece por ocorrência; outros controles Unicode não
são eliminados do mapa por essa regra.

## Refutação da primeira tentativa e correção adicional

A primeira materialização corrigiu as contagens, mas os controles `trailing` e
`ligature` perderam espaços na extração comum. A caixa substituta incorporava
3.12pt do glifo invisível de espaço; um cluster de ligadura também podia registrar
hex vazio para esse GID. `first-round/` conserva essa tentativa e seus resultados
refutados. Não são evidências de conclusão.

A fonte local fixada do rustybuzz 0.20.1 (`src/lib.rs:32-35`) documenta o tratamento
padrão de Default_Ignorable e a flag REMOVE_DEFAULT_IGNORABLES. O teste independente
com essa flag falhou antes da correção (`placeholder-red.log`). A unidade de
identidade configura a flag somente para sub-runs com SHY autoral, em medição e
emissão (`text_identity.rs:28`, `shaping.rs:87,256`). O texto original entra inteiro
no shaping e permanece no envelope. Não há filtro de glifos por aparência/GID.
A flag retira placeholders após o shaping; a seleção de ligaduras continua com
o motor. O teste compara o resultado com rustybuzz configurado independentemente,
incluindo tracking, e verifica não aninhamento e ausência de fonte.

## Evidências finais

- Dois testes causais iniciais RED→GREEN e um controle RED→GREEN do placeholder.
  As suítes finais passaram: 6365 testes core, 1132 infra e 45 de integração;
  duas exclusões core preexistentes. Logs e seus hashes estão no recibo final.
- Builds dev e release offline concluíram. Lint final: zero erros, 503 warnings,
  1554 notes; nenhum V5/V6/V15/V26 nos owners desta frente. Avisos anteriores
  do repositório não foram corrigidos fora do escopo.
- Nos vinte e seis controles principais fora da seleção estreita, palavras bbox
  e tokens da extração coincidem com o vanilla, preservando exatamente U+00AD,
  U+002D e limites entre palavras. Somente `move` conserva diferença de linhas
  em branco na extração bruta, registrada separadamente. Não se normaliza SHY
  na prova textual.
- Vinte e quatro dos trinta controles principais coincidem também em geometria
  com o vanilla dentro de 0.015pt. Quatro controles de seleção estreita, tracking
  e move permanecem excluídos dessa afirmação.
- MuPDF a 144dpi encontrou pixels idênticos antes/depois em vinte e nove dos
  trinta PDFs. Tracking mudou 1065 pixels pela retirada do placeholder com
  intervalo indevido. Dois controles independentes mostram que, após a correção,
  o texto com SHY tem pixels e caixas idênticos ao texto sem SHY, tanto compacto
  quanto verboso; SHY→vazio é usado apenas nesse contrafactual visual.
- A reprodução de 401 entradas anteriores realizou 1930 verificações sem falhas.
  Reexecutou Tekt e reutilizou observações certificadas do vanilla pinado com
  fontes de entrada idênticas; não são 401 novas execuções bilaterais.

`after/`, `after-controls/`, `tracking-controls.json`, `raster/`, `regressions/`
e `final-receipt.json` contêm as provas finais. Os runners estão preservados.
`raw-verification.json` conserva a comparação bruta de formatação e raster, sem
confundir espaços de layout da extração com a identidade dos tokens.

## Falhas abertas e próxima correção

Largura 60pt ainda revela diferenças na seleção de quebras explícitas. O controle
optimized/hyphenate:false também apresenta extração truncada no vanilla; isso é
comportamento observado a investigar, não requisito de linguagem a copiar.
Quebras em compostos continuam fora desta correção. Não há certificação integral
de RTL, texto exclusivamente invisível ou SHY sem fonte resolvida. A aceitação
pelo compilador de um documento UA-1 não certifica toda a acessibilidade.

Os quatro controles adicionais de tracking separam o defeito de SHY de um defeito
anterior do espaçamento: em `plain-verbose`, sem SHY, a diferença máxima contra
vanilla já era 2.652pt antes desta frente e permanece igual depois; em compacto,
0.348pt. O SHY corrigido reproduz o mesmo desenho desse controle sem SHY.
`shaper/shaping.rs:319-326` aplica tracking aos avanços, enquanto
`export/stream/text.rs:639,668` também emite Tc nos runs shaped verbosos. Essa é
uma causa medida de soma dupla a corrigir no owner do exportador. Medição/layout
e arredondamento de tracking também precisam de controles próprios; não foram
fechados pelo transporte de identidade. Move conserva suas divergências visuais
anteriores. Esses limites impedem alegação de paridade global.

## Seguimento posterior

A soma dupla de tracking no PDF foi corrigida na frente
[Tracking PDF](tracking-pdf-2026-09-28.md), com estado e provas próprios.
O residual de medição/arredondamento e a precisão dos espaços continuam abertos.
