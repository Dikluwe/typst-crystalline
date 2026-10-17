# TS1 — tamanho textual abs + em

Materialização da superfície aprovada concluída e gates executados. **Este registro é o checkpoint TS1; a igualdade Styled necessita o lote corretivo separado descrito abaixo antes da entrega final.** O objetivo global permanece aberto.

## Mudança e autorização

O humano aprovou o manifesto 771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba, resposta “Aprovar proposta TS1”. Style::Size agora recebe Length e StyleDelta.size é Option<Length>. TextStyle.size continua Pt resolvido, StyleChain::size retorna f64; nenhum outro contrato público mudou. Migração Rust: Size(Pt(v))→Size(Length::pt(v)); size:Some(v)→Some(Length::pt(v)). Não houve publicação, merge, release ou commit/push do Typst. O pin CI do linter permanece12797073972e375d9ab078cf6750c8b30b71e2b9, publicado na etapa anterior.

Constructor encontra Length/Color em qualquer posição, com preferência nomeada, ocorrência causal, validação de duplicatas e body Content/Str/None. Erros de corpo, sobras e length/hint foram corrigidos. Set text passa a avaliar argumentos uma vez e transportar os posicionais; em não é truncado. Dobra afim, collapse e captura preservam declaração e ausência sem inserir default cedo. Typed vence custom no mesmo nó; captura exclusivamente custom conserva sua transparência morfológica. Raw/math recebem valores já resolvidos como comprimentos absolutos.

Os 19 consumers originais e seus L0s foram migrados. A validação S3/S4 exigiu um 20º consumer: Heading vincula o Pt resolvido como Length absoluto na chain do corpo e a restaura. Show-set de Heading usa proveniência privada nos caminhos legado e nominal Located; a cópia local do delta normaliza só abs antes da escala já existente. O Length declarado original permanece intacto; nenhuma resolução foi movida de layout para eval. Sobre 12pt, 2em+3pt em show-heading passou 37.8→36.6pt, e o filho relativo 54→73.2pt; os valores finais coincidem com o pin. L2=31.8pt e L3=27pt. A substituição completa por função ainda perde defaults e não está certificada.

L0 primeiro e resselo somente dos 20 owners. Núcleo afim pinado por sete owners. Metadados dos candidatos precisaram de posição/ordenação corretas e hash efetivo Tekt; bytes normativos originais foram preservados inicialmente e o refinamento interno de Heading foi medido antes de acrescentar sua claim. O pacote aprovado original permanece imutável. SHA bruto não foi usado como pin efetivo.

## Verificação

Seis RED funcionais iniciais; quatro RED adicionais de Heading. **14 testes novos GREEN**. Suíte completa: **8810 aprovados, zero falhas, sete ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. Os dois V17 introduzidos na primeira tentativa foram corrigidos por desdobramento de guards, sem supressões.

SARIF final: **124 V16 erros**, **1069 V19 notas**, **865 V20 notas**. Todos os diagnósticos fora dos consumers alterados são idênticos; V16 por consumer preservado; nenhum erro/warning novo. Gate global continua exit 1 pelos V16 existentes. Notas novas de Paint e lookup AST têm política coesa prescrita no L0; não foram silenciadas.

Coletas finais:48 pares principais +41 estendidos +16 suplementares =105 pares textuais; são entradas, não105 defeitos nem105 certificações universais. Oito pares de cor sólida foram conferidos em vetores derivados de PDF; a paleta Typst red é #ff4136, não vermelho RGB puro. Cinquenta pares Par/Box preservam a geometria anterior dentro de 0.002pt;105 pares RG1 preservam códigos/saídas/diagnósticos, estrutura de corpo HTML e geometria de linhas coloridas SVG. O pacote RG1 segue pendente, sem implementar seus contratos. Sete sondas avaliam admissão/repr de zero, negativos e não finitos; não certificam render de NaN/infinito.

A primeira implementação e seu binário/fontes estão arquivados em candidate-1.tar.gz. Falhas de metadados, parser de SVG/paleta e script parcial foram guardadas; não contam como RED funcional. A compilação parcial foi interrompida antes do GREEN válido. O comparador SARIF foi corrigido para calcular um mapa por arquivo. As coletas/gates finais usam fontes congeladas; tentativas não foram ocultadas.

## Limites que permanecem explícitos

A igualdade Styled já estava errada contra o pin, mas TS1 também expôs **duas regressões observáveis**: relative-vs-zero e affine-vs-absolute-part mudaram true→false, com vanilla true. A causa é a comparação estrutural dos Styles na igualdade da linguagem. Não são aceitas como contrato novo nem tratadas como sucesso; requerem lote separado do operador antes do relatório final ao usuário. Não apagar wrappers ou alterar PartialEq Rust para esconder o problema. Os corpora e o diagnóstico anterior conservam a medição antes/depois.

Paint Gradient/Tiling, seletor show text, defaults de Heading substituído, scripts, Box1fr, SVG multipágina, literal mínimo e V16 restantes continuam no objetivo global. Os controles absolutos de scripts coincidem com os relativos dentro de cada backend e confirmam que a diferença contra o pin antecede a migração. Resíduos de contorno/xMax e os demais limites estão nos dados brutos, sem afirmar paridade global.

## RAM e proveniência

MemoryMax 12GiB, MemoryHigh 10GiB, swap zero, Cargo jobs=1 e comandos pesados sequenciais. Pico observado nos grupos: **10741538816 bytes (10.004 GiB)**; eventos {'high': 1329, 'max': 0, 'oom': 0, 'oom_kill': 0}. Memória inclui cache, não é RSS isolado. Styles/StyleDelta medidos 336bytes, antes 328; Style permanece 32. O custo do campo é +8bytes; não foi alegada otimização de RAM.

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before/final-state.json.gz, snapshots, validation-result.json e recursos registram UTC, status/diffstat, hashes de fontes/L0/config, argv, binários e artefatos. Oráculo 9f2b6e8715237cb086899a42873660fe744622e8, SHA 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2.

[Inventário V16](inventario-v16-restante.md) · [Pendências](pendencias.json) · [Patch restrito](patch-since-checkpoint.diff). RG1 deve ser rebaseado preservando TS1 em apply/tests; seu manifesto original foi conferido, não substituído.
