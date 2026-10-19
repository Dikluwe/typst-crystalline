# Matemática dos exportadores PDF/SVG

Substituídos 24 usos de métodos matemáticos rejeitados pela política do
projeto. O inventário Clippy passou de 252 para 228 diagnósticos. Nas
48 renderizações comparadas do Tekt (24 fontes × PDF/SVG), os pixels
antes/depois permaneceram idênticos neste ambiente. Isso é evidência de
preservação local, não certificação de paridade ou portabilidade completa.

## Proveniência e medição anterior à decisão

Branch Tekt, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**, snapshot em `2026-10-14T01:11:52.948609+00:00`.
`before.json.gz` contém fontes, L0, manifest/lock, configuração de lint,
implementações da biblioteca libm consultadas, spans, status e diffstat.
O binário Tekt anterior tem SHA-256
`6d0e9cc0abaa1b1d90e9711f8a10251102038b783e53abb0fd3efe6d5cc8e772`.
A referência foi o vanilla pinado em
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Cada renderização guarda comando, stderr, dimensão e hash dos pixels.

O corpus cobre eixos lineares e conicidade em ângulos 0, 37, 90, 225 e
−450 graus, caixas 96×64 e 64×96 pontos, centros deslocados, espaços
sRGB/Oklab/HSV e controles CMYK. São controles delimitados, não todas
as combinações suportadas. A primeira tentativa escreveu hsv sem o
namespace color; foi rejeitada pelo parser, sem alteração produtiva.
O erro e os inputs iniciais foram preservados; a versão válida está em
cases.json. A coleta retomou somente as entradas ainda não registradas,
com identidade dos binários conferida.

## Mudanças

Os cinco L0 foram atualizados antes do código. O resselo ficou limitado
aos seus consumers.

- Conic PDF: libm::tanf no offset do arco e libm::sincosf nos pares f32;
  mesmos sinais, precisão, progressão, raio/centro, quantização e amostragem.
- Axial PDF e SVG: libm::sin/cos/atan2 em f64, preservando y/x de atan2,
  correção de aspecto e a negação após rem_euclid.
- Helper de erro dos testes adaptativos: quadrado delta*delta, com o
  mesmo somatório, raiz e premultiplicação.
- Referência do teste de arco de 90°: identidade independente
  tan(pi/8)=sqrt(2)−1, preservando todos os inputs e tolerâncias.

L3 declara libm =0.2.11 diretamente, a mesma versão já usada em L1.
O lock somente acrescentou essa dependência ao pacote typst-infra;
versões, fontes e checksums de todos os pacotes foram preservados e
validados com Cargo --locked. Não houve assinatura pública, default,
fase, estratégia de mesh, espaços promovidos, tolerância ou supressão
nova.

## Validação

Checagens estáticas concluídas em `2026-10-14T01:17:47.362552+00:00`.
A suíte infra terminou em `2026-10-14T01:19:32.484566+00:00`: **1316
passaram, 0 falhas, 2 ignorados**; ignorados não recebem crédito.
Os 1.284 nomes da suíte unitária permaneceram iguais.

Cargo offline/locked, execução serial, tmpfs isolado, linter publicado
pinado e oráculo comprovado. workspace-pre.json.gz registra comandos,
ambiente, estado Git e hashes; apesar do nome, essa execução usa
-p typst-infra, não --workspace. Fontes permaneceram inalteradas.

Build, check workspace/all-targets, Rustdoc -D warnings, formato,
linhagem estrita, linhagem reversa e diff check passaram. Linter
arquitetural: 0 erros, 380 avisos, 1611 notas.
O inventário Clippy usa --cap-lints warn para coletar todos os crates;
o gate real -D warnings ainda não passa. Core 37, shell
0, infra 178, wiring 13.

Foram comparados os canvases completos em RGBA premultiplicado, sem
recorte, encaixe, máscara definida pelo erro ou reajuste de tolerâncias.
As 48 saídas candidatas preservaram todos os pixels da baseline do Tekt.
Os 24 PDFs passaram qpdf e contêm uma página; os 24 SVGs foram parseados.
Essas verificações estruturais não provam que a aparência esteja correta.
Binário candidato SHA-256: `9163155b83268d800cafbcd31adba2a4b8024494df71433f17ffa83d77107637`.

## Divergência exige classificação, não cópia do oracle

Orientação explícita do usuário: uma divergência pode estar mais correta
no Tekt. ADR-0107/0108 exigem classificar semântica/sintaxe/morfologia
e separar a mecânica da intenção; o vanilla não é autoridade absoluta.

Antes desta mudança, 44 das 48 comparações de imagem Tekt/vanilla
já diferiam. Esse número não conta defeitos nem decide qual saída está
correta. A tabela completa está em before-comparison.json e foi
preservada após a troca numérica. Os fallbacks SVG CMYK continuam
explicitamente registrados; nenhuma promoção foi inferida de um PNG.

Sinal específico para investigação independente: no caso conic-1-96
PDF, a imagem Tekt é quase branca e a vanilla contém variação de cor.
O arquivo Tekt declara mesh em domínio 0..1 e Pattern Matrix identidade;
`03_infra/src/export/builder/resources.rs:564` aplica transformação de
bbox apenas à variante Spot. A fonte pede um rect de 96×64 pt com fill
gradiente. Esse vínculo entre unidade geométrica, espaço de pintura e
cobertura precisa ser verificado contra o contrato e a matemática do
PDF antes de decidir a correção; sem presumir corretos os detalhes do
vanilla. Nenhuma transformação de bbox foi alterada neste lote.

A suíte completa do workspace e a matriz remota não foram repetidas.
CI/release e os demais itens da auditoria continuam abertos. Nenhuma
release foi publicada.

## Evidências

Snapshots .json.gz permanecem separados. evidence.json.gz contém
receitas, logs, inventários, diffs, fontes do corpus, PDFs/SVGs e PNGs
(arquivos binários codificados em base64), com hashes verificados após
descompressão. Todos os resultados são delimitados pelos estados
before/pre-validation/after registrados.
